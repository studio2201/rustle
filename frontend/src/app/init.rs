// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::app_state::{Action, AppState};
use yew::prelude::*;

pub fn fetch_app_config(
    is_pin_required: UseStateHandle<bool>,
    enable_translation: UseStateHandle<bool>,
    enable_themes: UseStateHandle<bool>,
    enable_print: UseStateHandle<bool>,
    state: UseReducerHandle<AppState>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        if let Ok(resp) = gloo_net::http::Request::get("/api/pin-required").send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(req) = json.get("required").and_then(|v| v.as_bool()) {
                    is_pin_required.set(req);
                }
                if let Some(trans) = json.get("enable_translation").and_then(|v| v.as_bool()) {
                    enable_translation.set(trans);
                } else if let Some(trans) = json.get("enableTranslation").and_then(|v| v.as_bool()) {
                    enable_translation.set(trans);
                }

                let mut themes_enabled = true;
                if let Some(themes) = json.get("enable_themes").and_then(|v| v.as_bool()) {
                    themes_enabled = themes;
                    enable_themes.set(themes);
                } else if let Some(themes) = json.get("enableThemes").and_then(|v| v.as_bool()) {
                    themes_enabled = themes;
                    enable_themes.set(themes);
                }

                if !themes_enabled {
                    state.dispatch(Action::SetTheme("tourian".to_string()));
                }

                if let Some(print) = json.get("enable_print").and_then(|v| v.as_bool()) {
                    enable_print.set(print);
                } else if let Some(print) = json.get("enablePrint").and_then(|v| v.as_bool()) {
                    enable_print.set(print);
                }
            }
        }
    });
}
