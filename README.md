<h1 align="center">
  <img src="https://raw.githubusercontent.com/studio2201/.github/master/profile/assets/rustle.png" width="48" height="48" valign="middle"> Rustle
</h1>

<p align="center">
  <b>Self-hosted Wordle clone with multi-language support, custom themes, and zero tracking written in Rust.</b>
</p>

---

### Instant One-Line Install (Docker Container)

Run the official zero-dependency container on port 4503:

```bash
docker run -d --name rustle -p 4503:4503 -v /mnt/user/appdata/rustle:/config ghcr.io/studio2201/rustle:latest
```

Open your browser to `http://localhost:4503` to start guessing the daily word.

---

### One-Line Install (Native Package Manager)

On Debian, Ubuntu, Fedora, or RHEL:

```bash
curl -fsSL https://studio2201.github.io/packages/install.sh | sudo bash
```

---

### Unraid NAS Deployment

Deploy via the official Unraid Template:

1. Copy [`rustle.xml`](rustle.xml) to your Unraid flash drive under `/boot/config/plugins/dockerMan/templates-user/`.
2. Open **Docker** -> **Add Container** -> Select **rustle** from the template dropdown.
3. Click **Apply**.

---

### Environment Configuration

The backend service can be customized using the following environment variables:

| Variable | Description | Default |
| :--- | :--- | :---: |
| `PORT` | Network port the web server binds to | `4503` |
| `RUSTLE_PIN` | Security PIN required for application access | *(Disabled)* |
| `RUSTLE_DATA_DIR` | Directory path for persistent data and statistics | `/config` |
| `RUSTLE_ALLOWED_ORIGINS` | CORS allowed origins list (comma-separated) | `*` |
| `TRUST_PROXY` | Honor reverse proxy headers (`X-Forwarded-For`) | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated CIDR list of trusted reverse proxies | *(None)* |
| `LOG_LEVEL` | Tracing filter (`error`, `warn`, `info`, `debug`) | `info` |

---

### Administration CLI & TUI Dashboard

Every container and package includes a built-in administration utility (`rustle`).

Launch interactive TUI dashboard:
```bash
docker exec -it rustle rustle tui
```

System diagnostics and self-healing check:
```bash
docker exec -it rustle rustle doctor
```

CLI Command Reference:
- `rustle tui` — Interactive terminal user interface.
- `rustle doctor` — Diagnoses storage permissions, ports, and database health.
- `rustle status` — Displays network configuration and security parameters.
- `rustle data stats` — Shows storage utilization and entry metrics.

---

### Architecture & Security

- **Axum Web Backend**: High-concurrency async HTTP runtime built on Tokio.
- **Yew WebAssembly Frontend**: Type-safe client bundle running natively in browser WASM runtime.
- **Strict Input & Path Sanitization**: Path canonicalization guards preventing directory traversal escapes.
- **Fail-Closed Security PIN Authentication**: Rate-limited brute force protection with automatic lockout timers.

---

### License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <a href="https://github.com/studio2201/rustle">
    <img src="assets/rustle-header.jpg" alt="studio2201 banner" width="100%">
  </a>
</p>
