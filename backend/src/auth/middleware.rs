// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Axum middleware components for authentication routing and security headers.

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::auth::{is_authorized, AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: axum::extract::Request,
    next: Next,
) -> Response {
    let Some(ref pin) = state.pin else {
        return next.run(request).await;
    };

    let path = request.uri().path();
    if path == "/api/pin-required" || path == "/api/verify-pin" {
        return next.run(request).await;
    }

    if is_authorized(&headers, pin) {
        return next.run(request).await;
    }

    if path == "/" || path == "/index.html" || path.ends_with(".html") {
        return next.run(request).await;
    }

    StatusCode::UNAUTHORIZED.into_response()
}

pub async fn security_headers_middleware(req: axum::extract::Request, next: Next) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    headers.insert("X-Frame-Options", header::HeaderValue::from_static("DENY"));
    headers.insert(
        "X-Content-Type-Options",
        header::HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "Referrer-Policy",
        header::HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    headers.insert(
        "Content-Security-Policy",
        header::HeaderValue::from_static(
            "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; img-src 'self' data: blob: https:; connect-src 'self' ws: wss: http: https:; font-src 'self'; manifest-src 'self';"
        )
    );

    response
}
