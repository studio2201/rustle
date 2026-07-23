mod cookie_auth;
mod session_id;
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

pub mod auth;
pub mod routes;
pub mod utils;

use axum::{
    Router, middleware,
    routing::{get, post},
};
use shared_backend::middleware::cors_layer;
use shared_backend::server::ServerConfig;
use shared_backend::tracing_init::{default_log_dir, init_tracing};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use auth::{
    AppState, auth_check, auth_middleware, logout, pin_required, security_headers_middleware,
    verify_pin,
};
use routes::{serve_asset_manifest, serve_index, serve_service_worker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bootstrap tracing — shared helper reads `LOG_DIR` env var and
    // configures file + stdout logging.
    let log_dir = default_log_dir();
    init_tracing(log_dir.as_deref());

    // Load configuration from env. The shared `ServerConfig::from_env`
    // reads common variables like `PORT`, `SITE_TITLE`, `ALLOWED_ORIGINS`,
    // PIN, attempts/cookie settings.
    //
    // Rustle's app-prefix is "RUSTLE".
    let mut config = ServerConfig::from_env("RUSTLE");

    // App-specific tweak: Rustle historically defaulted to port 4502
    // (the shared default is 4401). Preserve that for backward compat.
    if std::env::var("PORT").is_err() {
        config.port = 4502;
    }

    let config = Arc::new(config);
    let app_state = AppState::new(Arc::clone(&config));

    let cors = cors_layer(&config);

    let api_routes = Router::new()
        .route("/pin-required", get(pin_required))
        .route("/verify-pin", post(verify_pin))
        .route("/auth-check", get(auth_check))
        .route("/logout", post(logout));

    let app = Router::new()
        .nest("/api", api_routes)
        .route("/asset-manifest.json", get(serve_asset_manifest))
        .route("/service-worker.js", get(serve_service_worker))
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .fallback_service(ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .layer(cors)
        .layer(middleware::from_fn(security_headers_middleware))
        .with_state(app_state.clone());

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.port));
    println!(
        "Server running natively on http://localhost:{}",
        config.port
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
