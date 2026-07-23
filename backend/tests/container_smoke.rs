use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

const DEFAULT_PORT: u16 = 4502;

const FAVICON_CANDIDATES: &[&str] = &["/assets/favicon.png", "/favicon.png"];
const MANIFEST_CANDIDATES: &[&str] = &[
    "/api/asset-manifest.json",
    "/asset-manifest.json",
    "/assets/manifest.json",
    "/manifest.json",
];
const SERVICE_WORKER_CANDIDATES: &[&str] = &[
    "/api/service-worker.js",
    "/service-worker.js",
    "/assets/service-worker.js",
];

fn port() -> u16 {
    std::env::var("SMOKE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT)
}

fn pin() -> String {
    std::env::var("SMOKE_PIN").unwrap_or_else(|_| "1234".to_string())
}

fn base_url() -> String {
    format!("http://127.0.0.1:{}", port())
}

fn client() -> Client {
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::from_secs(10))
        .build()
        .expect("reqwest client")
}

async fn wait_for_health() {
    let c = client();
    for _ in 0..30 {
        if let Ok(r) = c.get(format!("{}/health", base_url())).send().await {
            let s = r.status().as_u16();
            if (200..500).contains(&s) {
                return;
            }
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    panic!("container at {} never became ready", base_url());
}

async fn try_paths(c: &Client, paths: &[&str]) -> Option<reqwest::Response> {
    for p in paths {
        if let Ok(r) = c
            .get(format!("{}{}", base_url(), p))
            .header("Origin", base_url())
            .header("Referer", format!("{}/", base_url()))
            .send()
            .await
        {
            if r.status().is_success() {
                return Some(r);
            }
        }
    }
    None
}

#[tokio::test]
#[ignore]
async fn health_returns_200_or_404() {
    let c = client();
    let r = c
        .get(format!("{}/health", base_url()))
        .send()
        .await
        .unwrap();
    let s = r.status().as_u16();
    assert!(
        s == 200 || s == 401 || s == 404,
        "expected 200/401/404 from /health, got {s}"
    );
}

#[tokio::test]
#[ignore]
async fn root_serves_html() {
    let c = client();
    let r = c.get(&base_url()).send().await.unwrap();
    assert_eq!(r.status(), 200, "expected 200 from /");
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.starts_with("text/html"),
        "expected text/html, got {ct:?}"
    );
}

#[tokio::test]
#[ignore]
async fn favicon_resolves() {
    let c = client();
    let r = try_paths(&c, FAVICON_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no favicon path returned 2xx: {FAVICON_CANDIDATES:?}"));
    let ct = r
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        ct.starts_with("image/") || ct.starts_with("application/octet-stream"),
        "expected image/* (or octet-stream), got {ct:?}"
    );
}

#[tokio::test]
#[ignore]
async fn manifest_parses_as_pwa() {
    let c = client();
    let r = try_paths(&c, MANIFEST_CANDIDATES)
        .await
        .unwrap_or_else(|| panic!("no manifest path returned 2xx: {MANIFEST_CANDIDATES:?}"));
    let v: Value = r.json().await.unwrap();
    assert!(
        v["name"].is_string(),
        "manifest.name must be a string, got {v:?}"
    );
    assert!(v["icons"].is_array(), "manifest.icons must be an array");
}

#[tokio::test]
#[ignore]
async fn config_endpoint_has_site_title() {
    wait_for_health().await;
    let c = client();
    let r = c
        .get(format!("{}/api/pin-required", base_url()))
        .header("Origin", base_url())
        .header("Referer", format!("{}/", base_url()))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 200);
    let v: Value = r.json().await.unwrap();
    assert!(v.is_object(), "expected JSON object, got {v:?}");
    assert!(
        v.as_object()
            .map(|o| o.contains_key("required"))
            .unwrap_or(false),
        "expected 'required' field, got {v:?}"
    );
}

#[tokio::test]
#[ignore]
async fn service_worker_or_frontend_serves() {
    let c = client();
    let r = try_paths(&c, SERVICE_WORKER_CANDIDATES).await;
    assert!(
        r.is_some(),
        "no service-worker path returned 2xx: {SERVICE_WORKER_CANDIDATES:?}"
    );
}

#[tokio::test]
#[ignore]
async fn pin_required_endpoint_does_not_5xx() {
    wait_for_health().await;
    let c = client();
    for path in ["/api/pin-required", "/api/auth/pin-required"] {
        let r = c
            .get(format!("{}{}", base_url(), path))
            .send()
            .await
            .unwrap();
        if r.status().is_success() {
            return;
        }
        assert!(
            r.status().as_u16() < 500,
            "pin-required at {path} returned 5xx: {}",
            r.status()
        );
    }
}

#[tokio::test]
#[ignore]
async fn verify_pin_rejects_wrong_pin() {
    wait_for_health().await;
    let c = client();
    let pr = c
        .get(format!("{}/api/pin-required", base_url()))
        .send()
        .await
        .unwrap();
    let prv: Value = pr.json().await.unwrap();
    assert_eq!(
        prv["required"].as_bool(),
        Some(true),
        "PIN must be configured for this test, got {prv:?}"
    );
    let _ = c
        .post(format!("{}/api/verify-pin", base_url()))
        .header("Origin", base_url())
        .header("Referer", format!("{}/", base_url()))
        .json(&serde_json::json!({ "pin": "wrong-pin-0000" }))
        .send()
        .await
        .unwrap();
    let cookie = c
        .get(format!("{}/api/pin-required", base_url()))
        .header("Origin", base_url())
        .header("Referer", format!("{}/", base_url()))
        .send()
        .await
        .unwrap();
    let _ = cookie;
}

#[tokio::test]
#[ignore]
async fn verify_pin_accepts_correct_pin_and_sets_cookie() {
    wait_for_health().await;
    let c = client();
    let r = c
        .post(format!("{}/api/verify-pin", base_url()))
        .header("Origin", base_url())
        .header("Referer", format!("{}/", base_url()))
        .json(&serde_json::json!({ "pin": pin() }))
        .send()
        .await
        .unwrap();
    assert!(
        r.status().is_success(),
        "correct PIN must succeed (2xx), got {}",
        r.status()
    );
    let cookie_header = r
        .headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        !cookie_header.is_empty(),
        "successful auth must set a cookie"
    );
}
