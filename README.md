# Rustle

An optimized Rust + WebAssembly (Yew) Wordle clone.

## Local Development (Time-To-First-Run)

### 1. Prerequisites
Ensure you have the Rust toolchain and WASM compilation target installed:
```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk compiler
cargo install --locked trunk
```

### 2. Install Styles Dependencies
```bash
npm install
```

### 3. Run Development Server
```bash
trunk serve
```
Open [http://localhost:4409](http://localhost:4409) to play.

## Docker Container Deployment

### 1. Build Production Image
Build the multi-stage, optimized WASM Docker image:
```bash
docker build -t ubermetroid/rustle:latest -f docker/Dockerfile .
```

### 2. Run Container
Launch the lightweight Nginx server hosting the WASM app under a secure, non-root user:
```bash
docker run -d -p 4409:4409 --name rustle-game ubermetroid/rustle:latest
```
Open [http://localhost:4409](http://localhost:4409) to play.

## File Tree

```text
rustle/
├── Cargo.toml                  # Cargo dependencies & release optimization profile
├── Trunk.toml                  # WebAssembly build tool configuration
├── index.html                  # HTML entry point injecting CSS/WASM target
├── tailwind.config.js          # TailwindCSS config
├── package.json                # Tailwind and build configurations
└── src/
    ├── main.rs                 # Bootstraps the Yew WASM client to the DOM
    ├── app.rs                  # Main view layout coordinator
    ├── app_state.rs            # Centralized Reducer-based game state machine
    ├── app_effects.rs          # Yew custom hook holding game side effects
    ├── constants.rs            # Top-level constants module registration
    ├── index.css               # Core styling overrides (glassmorphism/colors)
    ├── tailwind.css            # Compiled output of tailwind class definitions
    ├── constants/
    │   ├── config.rs           # Core settings, localized text messages, and game rules
    │   └── word_db.rs          # Zero-allocation O(log N) binary search database
    ├── components/
    │   ├── mod.rs              # Exports and mounts modular UI components
    │   ├── alerts.rs           # Toast style event alerts
    │   ├── grid.rs             # Cell tiles container grid
    │   ├── keyboard.rs         # Virtual key inputs listener & styling
    │   ├── navbar.rs           # Navigation header controls
    │   ├── stat_bar.rs         # Top status metrics indicator (Streaks/Tries)
    │   ├── stat_histogram.rs   # Guess distributions chart horizontal bars
    │   ├── app_modals.rs       # Base container coordinating overlay visibility & settings callbacks
    │   ├── modal_base.rs       # Reusable backdrop and overlay layouts
    │   ├── modal_info.rs       # Instructions & Rules modal
    │   ├── modal_settings.rs   # Game difficulty, contrast, theme modal
    │   ├── modal_stats.rs      # Win distributions, count-downs, and sharing trigger
    │   ├── modal_date_picker.rs# Choose historical puzzles date picker
    │   └── modal_migrate.rs    # Exporting/importing user profiles via encrypted code
    └── helpers/
        ├── mod.rs              # Mounts core helpers
        ├── browser.rs          # Social media user agent checkers
        ├── encryption.rs       # Blowfish cryptology encoder/decoder
        ├── local_storage.rs    # Persistence adapters mapping structures to local web storage
        ├── share.rs            # Clipboard string formatting
        ├── stats.rs            # Streaks accumulation math
        ├── statuses.rs         # Correct / Present / Absent validation engine
        └── words.rs            # Epoch calendars, dates, and solution lookups
```
