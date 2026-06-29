// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Authentication and access verification module.

use axum::http::{header, HeaderMap};
use serde::Deserialize;

pub mod crypto;
pub mod handlers;
pub mod lockout;
pub mod middleware;

pub use crypto::{hash_pin, safe_compare};
pub use handlers::{auth_check, logout, pin_required, verify_pin};
pub use lockout::{
    get_client_ip, get_lockout_time_remaining, get_max_attempts, is_locked_out, login_attempts,
    record_attempt, reset_attempts,
};
pub use middleware::{auth_middleware, security_headers_middleware};

#[derive(Clone)]
pub struct AppState {
    pub pin: Option<String>,
    pub site_title: String,
    #[allow(dead_code)]
    pub allowed_origins: String,
    pub enable_translation: bool,
    pub enable_themes: bool,
    pub enable_print: bool,
}

#[derive(Deserialize)]
pub struct VerifyPinPayload {
    pub pin: Option<String>,
}

// Embed premium Login HTML
pub const LOGIN_HTML: &str = include_str!("../login.html");

/// Checks if client request carries authorized PIN header or cookie value.
pub fn is_authorized(headers: &HeaderMap, pin: &str) -> bool {
    let cookie_pin = headers
        .get(header::COOKIE)
        .and_then(|c| c.to_str().ok())
        .and_then(|c_str| {
            c_str
                .split(';')
                .find(|s| s.trim().starts_with("pin="))
                .and_then(|s| s.split('=').nth(1))
                .map(|s| s.trim().to_string())
        });
    let header_pin = headers
        .get("x-pin")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    match (cookie_pin, header_pin) {
        (Some(cookie), _) => safe_compare(&cookie, &hash_pin(pin)),
        (None, Some(hdr)) => safe_compare(&hdr, pin),
        (None, None) => false,
    }
}
