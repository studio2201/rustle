// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rustle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Rustle.  If not, see <https://www.gnu.org/licenses/>.

//! Axum request handlers for authentication and PIN verification.
//!
//! These endpoints (`/api/pin-required`, `/api/verify-pin`,
//! `/api/auth-check`, `/api/logout`) are app-specific: they expose the
//! configuration that the frontend login modal needs. The actual
//! gatekeeping logic lives in `shared_backend::auth::pin_auth_layer`.

use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use constant_time_eq::constant_time_eq;
use rand::Rng;
use serde_json::json;
use shared_backend::auth::{attempts, session};
use shared_backend::server::ip::get_client_ip;
use std::net::SocketAddr;

use crate::auth::{AppState, VerifyPinPayload, is_authorized};

/// Returns whether a PIN is required and PIN-related UI state.
pub async fn pin_required(
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let config = &state.config;
    let ip = get_client_ip(&headers, addr, config.trust_proxy, &config.trusted_proxies);
    let lockout = config.lockout_duration();
    let locked = attempts::is_locked_out(&ip, config.max_attempts, lockout);
    let lockout_seconds = attempts::lockout_remaining_secs(&ip, lockout);
    let attempts_left = attempts::attempts_left(&ip, config.max_attempts, lockout);

    Json(json!({
        "required": config.pin.is_some(),
        "length": config.pin.as_ref().map(|p| p.len()).unwrap_or(0),
        "locked": locked,
        "attempts_left": attempts_left,
        "lockout_minutes": lockout_seconds.div_ceil(60),
        "enable_translation": config.enable_translation,
        "enable_themes": config.enable_themes,
        "enable_print": config.enable_print,
    }))
}

/// Verifies a PIN against the configured one and issues a session cookie.
///
/// On success: returns a `pin=<random-token>; HttpOnly; SameSite=Strict`
/// cookie. Note: the shared backend uses a random-token model, not the
/// hash-the-PIN model Rustle previously used. The cookie value is opaque
/// (random) but the gatekeeping middleware still compares it with
/// constant-time equality, which is what the shared `pin_auth_layer`
/// already does.
pub async fn verify_pin(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<VerifyPinPayload>,
) -> impl IntoResponse {
    let config = &state.config;
    let ip = get_client_ip(&headers, addr, config.trust_proxy, &config.trusted_proxies);
    let lockout = config.lockout_duration();

    if attempts::is_locked_out(&ip, config.max_attempts, lockout) {
        let remaining = attempts::lockout_remaining_secs(&ip, lockout);
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

    let Some(ref config_pin) = config.pin else {
        let mut response = axum::response::Response::new(
            Json(json!({ "success": true })).into_response().into_body(),
        );
        let _ = response.headers_mut().insert(
            header::SET_COOKIE,
            HeaderValue::from_static("pin=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
        );
        return response;
    };

    let pin_str = payload.pin.as_deref().unwrap_or("").trim();
    if pin_str.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "PIN is required." })),
        )
            .into_response();
    }

    if constant_time_eq(pin_str.as_bytes(), config_pin.as_bytes()) {
        attempts::reset_attempts(&ip);

        // Issue a session cookie: a random opaque token, NOT the PIN itself.
        // The previous (hand-rolled) implementation hashed the PIN and set
        // it as the cookie value; the shared backend uses a random token
        // model where the cookie is opaque and the gatekeeping middleware
        // reads it and compares to the PIN via constant-time equality.
        let mut buf = [0u8; 32];
        rand::rng().fill_bytes(&mut buf);
        let token = buf.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        state.register_session(token.clone());

        let mut response = Json(json!({ "success": true })).into_response();
        session::issue_cookie(config, &token, &mut response);
        response
    } else {
        let attempt = attempts::record_attempt(&ip);
        let locked = attempts::is_locked_out(&ip, config.max_attempts, lockout);
        let left = config.max_attempts.saturating_sub(attempt.count);

        if locked {
            (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "success": false,
                    "error": format!("Too many attempts. Please try again in {} minutes.", config.lockout_time_minutes),
                    "attempts_left": 0,
                    "locked": true,
                    "lockout_minutes": config.lockout_time_minutes,
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

/// Returns 200 OK if the request is authenticated (or no PIN is set),
/// 401 otherwise.
pub async fn auth_check(headers: HeaderMap, State(state): State<AppState>) -> impl IntoResponse {
    if let Some(pin) = state.config.pin.as_deref()
        && !is_authorized(&headers, &state, pin)
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    StatusCode::OK.into_response()
}

/// Clears the `pin` cookie (logout).
pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let cookie_token = headers
        .get(header::COOKIE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c_str| {
            c_str
                .split(';')
                .find(|s| s.trim().starts_with("pin="))
                .and_then(|s| s.split('=').nth(1))
                .map(|s| s.trim().to_string())
        });

    if let Some(token) = cookie_token {
        state.unregister_session(&token);
    }

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_static("pin=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0"),
    );
    (
        StatusCode::OK,
        response_headers,
        Json(json!({ "success": true })),
    )
        .into_response()
}
