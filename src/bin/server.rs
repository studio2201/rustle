// Copyright (C) 2026 Jeryd
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

//! Native server backend using Axum.
//! Serves the static compiled frontend from the `/dist` directory.

#[cfg(not(target_arch = "wasm32"))]
mod native_server {
    use axum::Router;
    use std::net::SocketAddr;
    use tower_http::services::{ServeDir, ServeFile};

    #[tokio::main]
    pub async fn run() {
        // Setup directory serving with fallbacks for index.html client routing
        let serve_dir = ServeDir::new("dist").fallback(ServeFile::new("dist/index.html"));

        let app = Router::new().fallback_service(serve_dir);

        let addr = SocketAddr::from(([0, 0, 0, 0], 4409));
        println!("Server running natively on http://localhost:4409");

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    native_server::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Dummy main for wasm32-unknown-unknown target
}
