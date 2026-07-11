# Rustle

Rustle is an optimized, responsive, and secure clone of the popular Wordle game built using Rust, Yew, and WebAssembly, served by a native Axum backend.

## Architecture and Stack

* Frontend: Yew (WASM)
* Backend: Axum (Rust) / Tokio
* Deployment: UBI container (Red Hat UBI9) on Docker Hub / Unraid / Podman / Docker Compose

## Code Map

The project is structured as a cargo workspace containing a Rust-based backend and a WebAssembly frontend built with Yew.

* [backend/src/main.rs](backend/src/main.rs): Application process entrypoint, configuration loading, and HTTP server initialization.
* [backend/src/auth/handlers.rs](backend/src/auth/handlers.rs): HTTP route handlers for login validation, authentication status checks, and session logout.
* [frontend/src/app_state.rs](frontend/src/app_state.rs): Centralized frontend application state and state reducer implementation for state updates.
* [frontend/src/components/weather_engine.rs](frontend/src/components/weather_engine.rs): Particle canvas weather engine for atmospheric ambient particle rendering and boundary collision checking.
* [frontend/src/app/enter.rs](frontend/src/app/enter.rs): Gameplay guess submit handler validating character bounds, dictionary inclusion, hard mode constraints, and completed game stats.

## Key Features

* Standardized UI Alignment: Completely integrated with shared-assets for a uniform theme engine, navigation header, footer, and authentication layout.
* Atmospheric Environments: Selectable map location themes inspired by classic environments (Crateria, Brinstar, Norfair, Wrecked Ship, Maridia, Tourian) with custom color schemes and interactive ambient weather particle effects.
* Classic Gameplay Rules: Standard Wordle guess validation, semantic color grid cell styling, high-contrast settings, and an optional hard-mode toggle.
* Secure PIN Access: Optional lock screen gate with client IP rate-limiting, timing-attack protections, and session cookie validation.
* Performance First: Tiny resource footprint, zero external JS engine dependencies, and rapid page load speeds.

## Local Setup

Ensure you have the Rust toolchain (stable) and Trunk installed.

### Prerequisites

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk compiler
cargo install --locked trunk
```

### Development Commands

```bash
# 1. Run workspace tests
cargo test

# 2. Run clippy workspace checks
cargo clippy --workspace --all-targets

# 3. Start frontend Yew dev server (from frontend/)
cd frontend && trunk serve

# 4. Start backend Axum server (from backend/)
cd backend && cargo run
```

## Deployment and Hosting

Rustle is optimized for self-hosting on Unraid, Docker, and Podman. Official images are built on Red Hat Universal Base Image (UBI9-minimal).

### Unraid Deployment Details

Rustle templates are available through the community application repository.
* Docker Hub Repository: `ubermetroid/rustle` (tags: `latest`, `ubi`, or version pins)
* Network Mode: Bridge (default port: `4502`)
* Volume Configuration: Since word-guessing progress and game statistics are stored client-side in the browser's local storage, no persistent server-side directory volumes are required.
* Security: The container runs with non-root privileges (`--user 99:100`).

### Docker Compose

Create a `docker-compose.yml` file with the following service definition:

```yaml
services:
  rustle:
    image: ubermetroid/rustle:latest
    container_name: rustle
    restart: unless-stopped
    ports:
      - ${PORT:-4502}:4502
    environment:
      PORT: 4502
      SITE_TITLE: ${SITE_TITLE:-Rustle}
      RUSTLE_PIN: ${RUSTLE_PIN:-}
      BASE_URL: ${BASE_URL:-http://localhost:4502}
      ALLOWED_ORIGINS: ${ALLOWED_ORIGINS:-*}
      TZ: ${TZ:-UTC}
      ENABLE_TRANSLATION: ${ENABLE_TRANSLATION:-false}
      MAX_ATTEMPTS: ${MAX_ATTEMPTS:-5}
```

### Build UBI Image Locally

```bash
# From the repository root
podman build --format docker -f Containerfile.ubi \
  -t docker.io/ubermetroid/rustle:0.1.36 \
  -t docker.io/ubermetroid/rustle:latest \
  -t docker.io/ubermetroid/rustle:ubi \
  .
```

## Configuration Options

| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server binds to inside the container. | `4502` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Rustle` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies. | `http://localhost:4502` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). | `*` |
| `RUSTLE_PIN` | Optional 4–64 character PIN to lock access to the interface. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header. | `false` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header. | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header. | `false` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before rate lockout. | `5` |
| `LOCKOUT_TIME_MINUTES` | Lockout duration in minutes for IPs exceeding `MAX_ATTEMPTS`. | `15` |
| `COOKIE_MAX_AGE_HOURS` | Duration in hours that the user's PIN session cookie remains valid. | `24` |
| `SHUTDOWN_DRAIN_SECONDS` | Seconds to wait for active connections to finish before shutting down. | `5` |
| `SHOW_VERSION` | Display the application version number in the footer. | `true` |
| `SHOW_GITHUB` | Display the GitHub repository link in the footer. | `true` |
| `TRUST_PROXY` | Set `true` if backend is hosted behind a reverse proxy. | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated IP/CIDR list of trusted upstream proxies. | None |

## License

Licensed under the [Apache License, Version 2.0](LICENSE). Copyright 2026 UberMetroid.
