# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.7] - 2026-07-19

### Changed
- Update README, clean file tree, and remove contributing/license files.


## [1.0.6] - 2026-07-19

### Changed
- **Standardized CLI & TUI command interface**: Aligned all admin commands and options with industry standard conventions. Added aliases for starting (`up`, `run`), stopping (`stop`, `down`), restarting (`restart`, `reload`), and diagnosing (`check`, `diagnose`) the application services.


## [1.0.5] - 2026-07-19

### Added
- **TUI & CLI Diagnostic Commands**: Added `doctor`, `start`, and `end`/`close` commands. Added the interactive system health check (doctor report) to the TUI panel menu.


## [1.0.4] - 2026-07-19

### Added
- **CLI Version Flag**: Added support for checking version details in the admin CLI using `version`, `-v`, or `--version` flags.


## [1.0.3] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [1.0.2] - 2026-07-19

### Added
- **Interactive Admin CLI & TUI Console**: Replaced the stub `sh` binary with a fully-featured, zero-dependency command-line interface and terminal user interface (TUI) dashboard for managing settings, checking database/storage file statistics, and viewing database records.


## [1.0.1] - 2026-07-19

### Fixed
- **Favicon cache-bust query** updated version string from `?v=0.1.52` to `?v=1.0.1` in `frontend/index.html` to align with the release version.
- **Backend Tests** removed unused constants `APP_NAME` and `CONFIG_CANDIDATES` in `backend/tests/container_smoke.rs` to clean up compiler warnings.

## [1.0.0] - 2026-07-19

### Changed
- **Rebrand and Align**: Restructured Docker configuration (`Containerfile.ubi`) and GitHub Actions to support Unraid compatibility (runs as UID 99:GID 100, data-dir symlinked to `/config`, listens on all interfaces) across all studio2201 apps.
- **Top-level assets**: Added `LICENSE` and `CONTRIBUTING.md` files.

## [0.1.52] - 2026-07-19

### Changed
- **Rebrand to studio2201**: README, container labels, docker-compose, and Cargo
  metadata now reference `studio2201/rustle`. CI badge URL and GHCR image name
  updated accordingly.
- **Open Graph and Twitter meta tags** replaced placeholder strings
  ("Rustle (Title)", "Rustle (Description)") with real values in
  `frontend/index.html`.
- **Favicon cache-bust query** added `?v=0.1.52` in `frontend/index.html` to
  invalidate stale PWA icon cache and match the Cargo workspace version.
- **Containerfile image description** lowercased to match the other repos
  ("Rustle (UBI9 minimal pilot)" → "rustle (UBI9 minimal pilot)").

## [0.1.0] - 2026-06-22

### Added
- Completed conversion of the application from React/TypeScript to Yew/Rust/WebAssembly.
- Implemented pure Rust Tailwind CSS compiler pipeline (no node_modules or npm dependencies).
- Added unit tests for game logic, local storage, stats persistence, and word lists.
- Dynamically sized the virtual keyboard to occupy exactly 2/3 width and 2/3 of the bottom 2/3 of screen height (`h-[44vh]`).
- Ensured uniform key box sizing across standard and special (`ENTER` / `DELETE`) keys.
- Updated repository workflows and LICENSE file to align with GPL-3.0 copyleft licensing.
