// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Axum request handlers for authentication and PIN verification.

use axum::{
    extract::{ConnectInfo, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::net::SocketAddr;

use crate::auth::{
    crypto::{hash_pin, safe_compare},
    is_authorized,
    lockout::{
        get_client_ip, get_lockout_time_remaining, get_max_attempts, is_locked_out, login_attempts,
        record_attempt, reset_attempts,
    },
    AppState, VerifyPinPayload,
};

pub async fn pin_required(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let ip = get_client_ip(&headers, addr);
    let locked = is_locked_out(&ip);
    let lockout_seconds = get_lockout_time_remaining(&ip);
    let attempts_left = if locked {
        0
    } else {
        let mut attempts_count = 0;
        if let Ok(attempts) = login_attempts().lock() {
            attempts_count = attempts.get(&ip).map(|a| a.count).unwrap_or(0);
        }
        get_max_attempts().saturating_sub(attempts_count)
    };
    Json(json!({
        "required": state.pin.is_some(),
        "length": state.pin.as_ref().map(|p| p.len()).unwrap_or(0),
        "locked": locked,
        "attempts_left": attempts_left,
        "lockout_minutes": lockout_seconds.div_ceil(60),
        "enable_translation": state.enable_translation,
        "enable_themes": state.enable_themes,
        "enable_print": state.enable_print,
    }))
}

pub async fn verify_pin(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyPinPayload>,
) -> impl IntoResponse {
    let ip = get_client_ip(&headers, addr);

    if is_locked_out(&ip) {
        let remaining = get_lockout_time_remaining(&ip);
        let minutes = remaining.div_ceil(60);
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "success": false,
                "error": format!("Too many attempts. Please try again in {} minutes.", minutes),
                "attempts_left": 0,
                "locked": true,
                "lockout_minutes": minutes,
            })),
        )
            .into_response();
    }

    let Some(ref config_pin) = state.pin else {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            header::HeaderValue::from_static("pin=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
        );
        return (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response();
    };

    let pin_str = payload.pin.as_deref().unwrap_or("").trim();
    if pin_str.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "PIN is required." })),
        )
            .into_response();
    }

    if safe_compare(pin_str, config_pin) {
        reset_attempts(&ip);
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            header::HeaderValue::from_str(&format!(
                "pin={}; Path=/; HttpOnly; SameSite=Strict",
                hash_pin(pin_str)
            ))
            .unwrap(),
        );
        (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response()
    } else {
        record_attempt(&ip);
        let locked = is_locked_out(&ip);
        let mut attempts_count = 0;
        if let Ok(attempts) = login_attempts().lock() {
            attempts_count = attempts.get(&ip).map(|a| a.count).unwrap_or(0);
        }
        let left = get_max_attempts().saturating_sub(attempts_count);

        if locked {
            (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "success": false,
                    "error": "Too many attempts. Please try again in 15 minutes.",
                    "attempts_left": 0,
                    "locked": true,
                    "lockout_minutes": 15,
                })),
            )
                .into_response()
        } else {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "error": format!("Invalid PIN. {} attempts remaining.", left),
                    "attempts_left": left,
                    "locked": false,
                    "lockout_minutes": 0,
                })),
            )
                .into_response()
        }
    }
}

pub async fn auth_check(headers: HeaderMap, State(state): State<AppState>) -> impl IntoResponse {
    if let Some(ref pin) = state.pin {
        if !is_authorized(&headers, pin) {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }
    StatusCode::OK.into_response()
}

pub async fn logout() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        header::HeaderValue::from_static("pin=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
    );
    (StatusCode::OK, headers, Json(json!({ "success": true }))).into_response()
}
