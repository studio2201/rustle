# Rustle - Wordle Clone in Rust & WebAssembly

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/Rustle/master/public/favicon.png" alt="Rustle Logo" width="128" height="128">
</p>

Rustle is an optimized, responsive, and secure clone of the popular Wordle game built using Rust, Yew, and WebAssembly, served by a native Axum backend.

---

## 🐳 Container Installation

### Option 1: Docker Compose (Recommended)

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  rustle:
    image: ubermetroid/rustle:latest
    container_name: rustle
    restart: unless-stopped
    ports:
      - 4409:4409
    environment:
      - PORT=4409
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4409`.

### Option 2: Docker CLI

Run the following command to start the container:

```bash
docker run -d \
  --name rustle \
  --restart unless-stopped \
  -p 4409:4409 \
  ubermetroid/rustle:latest
```

---

## 📋 Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4409` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. *(Supports fallback `RUSTRUSTLE_TITLE`)* | `Rustle` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4409` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `RUSTLE_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |

## 📂 Repository Structure

```
.
└── src/
    ├── app
    │   ├── enter.rs
    │   └── mod.rs
    ├── app_effects.rs
    ├── app_state.rs
    ├── bin
    │   └── server
    │       ├── auth.rs
    │       ├── handlers.rs
    │       ├── login.html
    │       ├── main.rs
    │       └── utils.rs
    ├── components
    │   ├── alerts.rs
    │   ├── app_modals.rs
    │   ├── grid.rs
    │   ├── keyboard.rs
    │   ├── mod.rs
    │   ├── modal_base.rs
    │   ├── modal_date_picker.rs
    │   ├── modal_info.rs
    │   ├── modal_migrate.rs
    │   ├── modal_stats.rs
    │   ├── navbar.rs
    │   ├── stat_bar.rs
    │   ├── stat_histogram.rs
    │   ├── weather.rs
    │   └── weather_engine.rs
    ├── constants
    │   ├── config.rs
    │   ├── valid_guesses.txt
    │   ├── word_db.rs
    │   └── wordlist.txt
    ├── constants.rs
    ├── helpers
    │   ├── browser.rs
    │   ├── encryption.rs
    │   ├── holidays.rs
    │   ├── local_storage.rs
    │   ├── mod.rs
    │   ├── share.rs
    │   ├── stats.rs
    │   ├── statuses.rs
    │   ├── tests.rs
    │   └── words.rs
    ├── index.css
    ├── main.rs
    └── tailwind.css
```
