// Copyright (C) 2026 UberMetroid
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

//! Centralized state management for the Rustle frontend.
//!
//! Exposes the [`AppState`] struct to hold Yew client state, the [`Action`]
//! enum to represent state mutations, and a [`Reducible`] implementation
//! that reduces actions to state changes.

use crate::helpers::local_storage::GameStats;
use std::rc::Rc;
use yew::prelude::*;

/// Centralized application state for the Rustle gameplay session and modals.
#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    /// List of previously submitted guesses for the current puzzle.
    pub guesses: Vec<String>,
    /// The string representation of the active guess currently being typed.
    pub current_guess: String,
    /// Set to true if the player correctly guessed the target word.
    pub is_game_won: bool,
    /// Set to true if the player used all guesses without matching the target word.
    pub is_game_lost: bool,
    /// Set to true when guess tiles are executing their flip/reveal animations.
    pub is_revealing: bool,
    /// CSS animation class name applied to shake/jiggle the guess row on validation failure.
    pub jiggle_class: String,
    /// Text content currently loaded into the modal alert banner.
    pub alert_msg: String,
    /// Set to true to display the modal alert banner.
    pub alert_visible: bool,
    /// Styling variant class representing the type of alert (e.g. "error", "success").
    pub alert_variant: String,
    /// Set to true if the "How to Play" tutorial modal is open.
    pub is_info_open: bool,
    /// Set to true if the gameplay stats modal is open.
    pub is_stats_open: bool,
    /// Set to true if the calendar archive date picker modal is open.
    pub is_datepicker_open: bool,
    /// Set to true if the stats migration/import modal is open.
    pub is_migrate_open: bool,
    /// Loaded user game statistics (win percentage, streaks, guess distribution).
    pub game_stats: GameStats,
    /// Theme selection identifier (e.g. "light", "dark", "high-contrast").
    pub theme: String,
    /// Set to true if the player has enabled hard mode rules (must use revealed hints).
    pub is_hard_mode: bool,
    /// Set to true to trigger canvas-based weather particle effects (snow, rain).
    pub is_effects_active: bool,
}

/// Reducer action type to update the application state.
pub enum Action {
    /// Append a character to the current guess string.
    AddChar(char),
    /// Remove the trailing character from the current guess string.
    DeleteChar,
    /// Empty the current guess string buffer.
    ClearGuess,
    /// Overwrite the submitted guesses history list.
    SetGuesses(Vec<String>),
    /// Update the gameplay win state.
    SetWon(bool),
    /// Update the gameplay loss state.
    SetLost(bool),
    /// Set the tile reveal animation status lock.
    SetRevealing(bool),
    /// Trigger a jiggle animation class on the current row.
    SetJiggle(String),
    /// Trigger the notification alert banner with a message and style variant.
    ShowAlert(String, String),
    /// Dismiss the active notification alert banner.
    HideAlert,
    /// Toggle visibility of the "How to Play" tutorial modal.
    SetInfoOpen(bool),
    /// Toggle visibility of the stats modal.
    SetStatsOpen(bool),
    /// Toggle visibility of the calendar date selector modal.
    SetDatePickerOpen(bool),
    /// Toggle visibility of the stats import/export migration modal.
    SetMigrateOpen(bool),
    /// Overwrite the active user gameplay statistics profile.
    SetGameStats(GameStats),
    /// Toggle hard mode rules enforcement.
    SetHardMode(bool),
    /// Update the current visual styling theme.
    SetTheme(String),
    /// Enable or disable canvas-based weather effects.
    SetEffectsActive(bool),
}

impl Reducible for AppState {
    type Action = Action;

    /// Reduces an action into a new state instance.
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        match action {
            Action::AddChar(c) => {
                state.current_guess.push(c);
            }
            Action::DeleteChar => {
                let _ = state.current_guess.pop();
            }
            Action::ClearGuess => {
                state.current_guess.clear();
            }
            Action::SetGuesses(g) => {
                state.guesses = g;
            }
            Action::SetWon(w) => {
                state.is_game_won = w;
            }
            Action::SetLost(l) => {
                state.is_game_lost = l;
            }
            Action::SetRevealing(r) => {
                state.is_revealing = r;
            }
            Action::SetJiggle(j) => {
                state.jiggle_class = j;
            }
            Action::ShowAlert(msg, var) => {
                state.alert_msg = msg;
                state.alert_variant = var;
                state.alert_visible = true;
            }
            Action::HideAlert => {
                state.alert_visible = false;
            }
            Action::SetInfoOpen(o) => {
                state.is_info_open = o;
            }
            Action::SetStatsOpen(o) => {
                state.is_stats_open = o;
            }
            Action::SetDatePickerOpen(o) => {
                state.is_datepicker_open = o;
            }
            Action::SetMigrateOpen(o) => {
                state.is_migrate_open = o;
            }
            Action::SetGameStats(s) => {
                state.game_stats = s;
            }
            Action::SetHardMode(val) => {
                state.is_hard_mode = val;
            }
            Action::SetTheme(val) => {
                state.theme = val;
            }
            Action::SetEffectsActive(val) => {
                state.is_effects_active = val;
            }
        }
        Rc::new(state)
    }
}

impl AppState {
    /// Construct a new state instance, loading history and preferences from browser LocalStorage.
    pub fn new(solution: &str, is_latest_game: bool, prefers_dark: bool) -> Self {
        let guesses = if let Some(state) =
            crate::helpers::local_storage::load_game_state_from_local_storage(is_latest_game)
        {
            if state.solution == solution {
                state.guesses
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        let is_game_won = guesses.contains(&solution.to_string());
        let is_game_lost =
            guesses.len() >= crate::constants::config::MAX_CHALLENGES && !is_game_won;

        let prefs =
            crate::helpers::local_storage::load_preferences_from_local_storage(prefers_dark);
        let theme = prefs.theme;
        let is_hard_mode = prefs.is_hard_mode;

        let game_stats = crate::helpers::stats::load_stats();
        let is_effects_active = is_game_won;

        Self {
            guesses,
            current_guess: "".to_string(),
            is_game_won,
            is_game_lost,
            is_revealing: false,
            jiggle_class: "".to_string(),
            alert_msg: "".to_string(),
            alert_visible: false,
            alert_variant: "error".to_string(),
            is_info_open: false,
            is_stats_open: false,
            is_datepicker_open: false,
            is_migrate_open: false,
            game_stats,
            theme,
            is_hard_mode,
            is_effects_active,
        }
    }
}
