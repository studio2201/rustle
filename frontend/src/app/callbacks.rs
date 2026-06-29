// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.
//
// Rustle is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use crate::app_state::{Action, AppState};
use crate::constants::config::{ALERT_TIME_MS, HARD_MODE_ALERT_MESSAGE};
use yew::prelude::*;

pub fn build_on_theme_click(
    state: UseReducerHandle<AppState>,
    enable_themes: UseStateHandle<bool>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        if !*enable_themes {
            return;
        }
        let date = crate::helpers::words::get_game_date();
        if crate::helpers::holidays::get_holiday_for_date(date).is_some() {
            return;
        }
        let next_theme = match state.theme.as_str() {
            "crateria" => "brinstar".to_string(),
            "brinstar" => "norfair".to_string(),
            "norfair" => "wrecked_ship".to_string(),
            "wrecked_ship" => "maridia".to_string(),
            "maridia" => "tourian".to_string(),
            _ => "crateria".to_string(),
        };
        state.dispatch(Action::SetTheme(next_theme.clone()));
        crate::helpers::local_storage::save_preferences_to_local_storage(
            &crate::helpers::local_storage::StoredPreferences {
                theme: next_theme,
                is_hard_mode: state.is_hard_mode,
            },
        );
    })
}

pub fn build_on_hard_mode_click(
    state: UseReducerHandle<AppState>,
    show_alert: Callback<(String, String, u32)>,
) -> Callback<MouseEvent> {
    Callback::from(move |_| {
        let next_val = !state.is_hard_mode;
        if next_val {
            if state.guesses.is_empty() {
                state.dispatch(Action::SetHardMode(true));
                crate::helpers::local_storage::save_preferences_to_local_storage(
                    &crate::helpers::local_storage::StoredPreferences {
                        theme: state.theme.clone(),
                        is_hard_mode: true,
                    },
                );
            } else {
                show_alert.emit((
                    HARD_MODE_ALERT_MESSAGE.to_string(),
                    "error".to_string(),
                    ALERT_TIME_MS,
                ));
            }
        } else {
            state.dispatch(Action::SetHardMode(false));
            crate::helpers::local_storage::save_preferences_to_local_storage(
                &crate::helpers::local_storage::StoredPreferences {
                    theme: state.theme.clone(),
                    is_hard_mode: false,
                },
            );
        }
    })
}
