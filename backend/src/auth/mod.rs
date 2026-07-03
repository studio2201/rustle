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

//! Authentication and access verification module.
//!
//! Hand-rolled auth is replaced by `shared_backend::auth::*`. Rustle
//! keeps only app-specific bits:
//!
//! - [`AppState`] — wrapper around `Arc<ServerConfig>` plus app flags
//! - [`is_authorized`] — shim that checks cookie/header against the
//!   configured PIN (used by routes that need to know if a request is
//!   authenticated without going through the auth middleware)
//! - [`LOGIN_HTML`] — the bundled login page

pub mod handlers;
pub mod middleware;

pub use handlers::{auth_check, logout, pin_required, verify_pin};
pub use middleware::{auth_middleware, security_headers_middleware};

use axum::http::{header, HeaderMap};
use serde::Deserialize;
use shared_backend::auth::PinState;
use shared_backend::server::ServerConfig;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ServerConfig>,
    pub pin_state: PinState,
}

impl AppState {
    pub fn new(config: Arc<ServerConfig>) -> Self {
        let pin_state = PinState::from(Arc::clone(&config));
        Self { config, pin_state }
    }

    pub fn pin(&self) -> Option<&str> {
        self.config.pin.as_deref()
    }

    pub fn site_title(&self) -> &str {
        &self.config.site_title
    }

    pub fn enable_translation(&self) -> bool {
        self.config.enable_translation
    }

    pub fn enable_themes(&self) -> bool {
        self.config.enable_themes
    }

    pub fn enable_print(&self) -> bool {
        self.config.enable_print
    }
}

impl From<Arc<ServerConfig>> for AppState {
    fn from(config: Arc<ServerConfig>) -> Self {
        Self::new(config)
    }
}

#[derive(Deserialize)]
pub struct VerifyPinPayload {
    pub pin: Option<String>,
}

// Embed premium Login HTML
pub const LOGIN_HTML: &str = include_str!("../login.html");

/// Checks if client request carries authorized PIN header or cookie value.
///
/// Used by routes that gate content based on auth status without going
/// through the auth middleware (e.g. `serve_index` returning login HTML
/// for unauthenticated users when PIN is enabled).
pub fn is_authorized(headers: &HeaderMap, pin: &str) -> bool {
    use constant_time_eq::constant_time_eq;

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
        (Some(cookie), _) => constant_time_eq(cookie.as_bytes(), pin.as_bytes()),
        (None, Some(hdr)) => constant_time_eq(hdr.as_bytes(), pin.as_bytes()),
        (None, None) => false,
    }
}