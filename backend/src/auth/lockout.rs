// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Lockout and login attempt tracking utility logic.

use axum::http::HeaderMap;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

const LOCKOUT_DURATION: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Clone)]
pub struct Attempt {
    pub count: u32,
    pub last_attempt: Instant,
}

pub fn login_attempts() -> &'static Mutex<HashMap<String, Attempt>> {
    static ATTEMPTS: OnceLock<Mutex<HashMap<String, Attempt>>> = OnceLock::new();
    ATTEMPTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_max_attempts() -> u32 {
    std::env::var("MAX_ATTEMPTS")
        .ok()
        .and_then(|val| val.parse().ok())
        .unwrap_or(5)
}

pub fn is_locked_out(ip: &str) -> bool {
    if let Ok(mut attempts) = login_attempts().lock() {
        if let Some(attempt) = attempts.get(ip) {
            if attempt.count >= get_max_attempts() {
                if attempt.last_attempt.elapsed() < LOCKOUT_DURATION {
                    return true;
                }
                attempts.remove(ip);
            }
        }
    }
    false
}

pub fn record_attempt(ip: &str) {
    if let Ok(mut attempts) = login_attempts().lock() {
        let now = Instant::now();
        let attempt = attempts.entry(ip.to_string()).or_insert(Attempt {
            count: 0,
            last_attempt: now,
        });
        attempt.count += 1;
        attempt.last_attempt = now;
    }
}

pub fn reset_attempts(ip: &str) {
    if let Ok(mut attempts) = login_attempts().lock() {
        attempts.remove(ip);
    }
}

pub fn get_lockout_time_remaining(ip: &str) -> u64 {
    if let Ok(attempts) = login_attempts().lock() {
        if let Some(attempt) = attempts.get(ip) {
            let elapsed = attempt.last_attempt.elapsed();
            if elapsed < LOCKOUT_DURATION {
                let remaining = LOCKOUT_DURATION - elapsed;
                return remaining.as_secs();
            }
        }
    }
    0
}

pub fn get_client_ip(headers: &HeaderMap, addr: SocketAddr) -> String {
    if let Some(cf_connecting_ip) = headers.get("cf-connecting-ip") {
        if let Ok(ip) = cf_connecting_ip.to_str() {
            return ip.to_string();
        }
    }
    if let Some(x_forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(ip_list) = x_forwarded_for.to_str() {
            if let Some(ip) = ip_list.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    if let Some(x_real_ip) = headers.get("x-real-ip") {
        if let Ok(ip) = x_real_ip.to_str() {
            return ip.to_string();
        }
    }
    addr.ip().to_string()
}
