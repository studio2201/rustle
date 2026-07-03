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

//! Axum middleware components for authentication routing and security headers.
//!
//! `auth_middleware` gates routes using [`shared_backend::auth::attempts`]
//! primitives and constant-time PIN comparison — the same building blocks
//! `shared_backend::auth::pin_auth_layer` uses internally. The path-based
//! bypass list is app-specific (Rustle allows HTML paths and the auth API
//! to load before login).
//!
//! Note: Rustle historically used `sha256(pin)` as the cookie value. The
//! shared backend switched to a random-token model where the cookie is an
//! opaque token and the gatekeeping middleware re-checks it against the
//! configured PIN via constant-time equality. The cookie is set by
//! `verify_pin` (random 32 bytes hex-encoded); the middleware reads it
//! back and compares to the PIN.

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use constant_time_eq::constant_time_eq;
use shared_backend::auth::attempts;
use shared_backend::server::ip::get_client_ip;
use std::net::SocketAddr;

use crate::auth::AppState;

const BYPASS_PATHS: &[&str] = &["/", "/index.html"];

fn is_html_bypass(path: &str) -> bool {
    BYPASS_PATHS.contains(&path) || path.ends_with(".html")
}

fn is_auth_api_bypass(path: &str) -> bool {
    path == "/api/pin-required"
        || path == "/api/verify-pin"
        || path == "/api/auth-check"
        || path == "/api/logout"
}

fn extract_pin(headers: &axum::http::HeaderMap, request: &Request) -> Option<String> {
    if let Some(p) = headers
        .get("x-pin")
        .and_then(|h| h.to_str().ok())
        .filter(|s| !s.is_empty())
    {
        return Some(p.to_string());
    }
    shared_backend::auth::read_pin_cookie(request)
}

fn unauthorized_response() -> Response {
    (StatusCode::UNAUTHORIZED, "unauthorized").into_response()
}

fn too_many_requests_response() -> Response {
    (StatusCode::TOO_MANY_REQUESTS, "too many requests").into_response()
}

/// Auth middleware: gates routes behind the configured PIN.
///
/// Bypass list (Rustle-specific):
/// - `/`, `/index.html`, and any `*.html` path (the SPA shell must load
///   before login; the frontend shows the login modal inline)
/// - `/api/pin-required`, `/api/verify-pin`, `/api/auth-check`,
///   `/api/logout` (auth API endpoints are by definition public)
/// - everything when no PIN is configured (public mode)
pub async fn auth_middleware(
    State(state): State<AppState>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path();

    // Bypass: PIN not configured.
    let Some(expected_pin) = state.config.pin.as_deref() else {
        return next.run(request).await;
    };

    // Bypass: HTML paths and auth API endpoints.
    if is_html_bypass(path) || is_auth_api_bypass(path) {
        return next.run(request).await;
    }

    let ip = get_client_ip(
        request.headers(),
        socket,
        state.config.trust_proxy,
        &state.config.trusted_proxies,
    );
    let lockout = state.config.lockout_duration();

    if attempts::is_locked_out(&ip, state.config.max_attempts, lockout) {
        return too_many_requests_response();
    }

    match extract_pin(request.headers(), &request) {
        Some(p) if constant_time_eq(expected_pin.as_bytes(), p.as_bytes()) => {
            attempts::reset_attempts(&ip);
            next.run(request).await
        }
        Some(_) => {
            let attempt = attempts::record_attempt(&ip);
            tracing::warn!(
                target: "auth",
                "failed PIN attempt #{count} from {ip}",
                count = attempt.count
            );
            if attempt.count >= state.config.max_attempts {
                tracing::warn!(target: "auth", "IP {ip} locked out");
            }
            unauthorized_response()
        }
        None => unauthorized_response(),
    }
}

/// Security headers: `X-Frame-Options`, `X-Content-Type-Options`,
/// `Referrer-Policy`, and `Content-Security-Policy`. App-specific CSP.
pub async fn security_headers_middleware(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; img-src 'self' data: blob: https:; connect-src 'self' ws: wss: http: https:; font-src 'self'; manifest-src 'self';",
        ),
    );

    response
}