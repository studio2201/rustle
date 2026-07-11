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

use crate::app_state::{Action, AppState};
use crate::constants::config::{ALERT_TIME_MS, LONG_ALERT_TIME_MS, MAX_CHALLENGES, REVEAL_TIME_MS};
use yew::prelude::*;

/// Create the callback triggered when the player hits the 'Enter' key to submit a guess.
///
/// This handler enforces Wordle/Rustle gameplay rules:
/// 1. Short guess check: Must type a word of matching length.
/// 2. Dictionary check: Must type a valid word from the dictionary.
/// 3. Hard Mode constraint check: Must reuse revealed clues in subsequent guesses.
/// 4. Game state transitions: Updates won/lost flags, streaks, and schedules alerts/modals.
pub fn build_on_enter(
    state: UseReducerHandle<AppState>,
    show_alert: Callback<(String, String, u32)>,
    solution: &'static str,
    is_latest_game: bool,
    i18n: crate::i18n::I18nContext,
) -> Callback<()> {
    Callback::from(move |()| {
        // If the game is already finished, do not accept further guesses.
        if state.is_game_won || state.is_game_lost {
            return;
        }
        let guess_len = state.current_guess.chars().count();
        let sol_len = solution.chars().count();

        // Rule 1: Validate character count matches target word length.
        if guess_len < sol_len {
            show_alert.emit((
                i18n.translations.not_enough_letters.to_string(),
                "error".to_string(),
                ALERT_TIME_MS,
            ));
            // Trigger visual shaking animation on the guess row.
            state.dispatch(Action::SetJiggle("jiggle".to_string()));
            return;
        }

        let word = state.current_guess.clone().to_uppercase();
        // Rule 2: Validate guess is present in the dictionary.
        if !crate::helpers::words::is_word_in_word_list(&word) {
            show_alert.emit((
                i18n.translations.word_not_found.to_string(),
                "error".to_string(),
                ALERT_TIME_MS,
            ));
            state.dispatch(Action::SetJiggle("jiggle".to_string()));
            return;
        }

        // Rule 3: Enforce hard mode restrictions if active.
        // Guesses must contain all previously revealed letters (correct positions and present symbols).
        if state.is_hard_mode {
            if let Some(fail) =
                crate::helpers::words::find_first_unused_reveal(&word, &state.guesses, solution)
            {
                show_alert.emit((fail, "error".to_string(), ALERT_TIME_MS));
                state.dispatch(Action::SetJiggle("jiggle".to_string()));
                return;
            }
        }

        // Lock grid interactions and begin row tile reveal animation sequence.
        state.dispatch(Action::SetRevealing(true));
        let state_rev = state.clone();
        gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * sol_len as u32, move || {
            state_rev.dispatch(Action::SetRevealing(false))
        })
        .forget();

        // Append guess to history list and trigger state update.
        let mut new_guesses = state.guesses.clone();
        new_guesses.push(word.clone());
        state.dispatch(Action::SetGuesses(new_guesses.clone()));

        // Persist guesses to LocalStorage.
        crate::helpers::local_storage::save_game_state_to_local_storage(
            is_latest_game,
            &crate::helpers::local_storage::StoredGameState {
                guesses: new_guesses.clone(),
                solution: solution.to_string(),
            },
        );

        // Reset the typing buffer for the next guess.
        state.dispatch(Action::ClearGuess);

        // Rule 4: Check Win/Loss conditions or offer intermediate match comments.
        if crate::helpers::words::is_winning_word(&word, solution) {
            // Player won! Update state and win statistics.
            state.dispatch(Action::SetWon(true));
            state.dispatch(Action::SetGameStats(
                crate::helpers::stats::add_stats_for_completed_game(
                    state.game_stats.clone(),
                    new_guesses.len() - 1,
                ),
            ));

            // Select a congratulations message (supports seasonal themes).
            let win_messages = i18n.translations.win_messages;
            let win_message = get_seasonal_win_message(&state.theme, win_messages);
            let state_won = state.clone();
            let show_alert_clone = show_alert.clone();
            
            // Wait for flip animation to complete before showing the win summary overlay.
            gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * sol_len as u32, move || {
                show_alert_clone.emit((win_message, "success".to_string(), ALERT_TIME_MS));
                state_won.dispatch(Action::SetStatsOpen(true));
                state_won.dispatch(Action::SetEffectsActive(true));
            })
            .forget();
        } else if new_guesses.len() >= MAX_CHALLENGES {
            // Player lost (out of guesses)! Update stats.
            state.dispatch(Action::SetLost(true));
            state.dispatch(Action::SetGameStats(
                crate::helpers::stats::add_stats_for_completed_game(
                    state.game_stats.clone(),
                    new_guesses.len(),
                ),
            ));

            let state_lost = state.clone();
            let show_alert_clone = show_alert.clone();
            let i18n_clone = i18n.clone();
            
            // Wait for flip animation to complete before showing correct solution and statistics.
            gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * (sol_len as u32 + 1), move || {
                let default_msg =
                    crate::i18n::get_correct_word_message(i18n_clone.language, solution);
                let msg = get_seasonal_loss_message(&state_lost.theme, solution, default_msg);
                show_alert_clone.emit((msg, "error".to_string(), LONG_ALERT_TIME_MS));
                state_lost.dispatch(Action::SetStatsOpen(true));
            })
            .forget();
        } else {
            // Game continues: calculate correct/present matches to emit a hint feedback message.
            let statuses = crate::helpers::statuses::get_guess_statuses(solution, &word);
            let correct = statuses
                .iter()
                .filter(|&&s| s == crate::helpers::statuses::CharStatus::Correct)
                .count();
            let present = statuses
                .iter()
                .filter(|&&s| s == crate::helpers::statuses::CharStatus::Present)
                .count();
            let total_matches = correct + present;

            // Fetch a motivating comment (e.g. "Almost there" or "No match") based on target theme.
            let feedback_msg =
                crate::helpers::feedback::get_intermediate_comment(&state.theme, total_matches);
            let show_alert_clone = show_alert.clone();
            gloo_timers::callback::Timeout::new(REVEAL_TIME_MS * sol_len as u32, move || {
                show_alert_clone.emit((feedback_msg, "info".to_string(), ALERT_TIME_MS));
            })
            .forget();
        }
    })
}

fn get_seasonal_win_message(theme: &str, default_messages: &[&str]) -> String {
    let list: &[&str] = match theme {
        "christmas" => &[
            "Merry Christmas! Santa is proud of you!",
            "Present obtained! Mission complete.",
            "Holiday cheer level critical!",
        ],
        "halloween" => &[
            "Spooky win! No tricks, just treats!",
            "Zebes ghost busted!",
            "Ghoulish intelligence unlocked!",
        ],
        "easter" => &[
            "Easter Egg found! Great job!",
            "Egg-cellent work!",
            "Hop to victory!",
        ],
        "thanksgiving" => &[
            "Thanksgiving feast obtained!",
            "Stuffed with victory!",
            "Feast complete: Zebes is grateful!",
        ],
        "newyear" => &[
            "Happy New Year! Resolution complete.",
            "Starting the year with a win!",
            "Fireworks activated!",
        ],
        "valentine" => &[
            "Heart container obtained!",
            "Cupid Samus strikes again!",
            "Love is in the air on Zebes!",
        ],
        "independence" => &[
            "Liberty and victory for all!",
            "Fireworks sequence initiated!",
            "Independence Day speedrun!",
        ],
        "stpatrick" => &[
            "Lir's treasure obtained! Luck of the Chozo.",
            "Irish eyes are smiling on Zebes!",
            "Pot of gold found!",
        ],
        _ => default_messages,
    };
    let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
    list[idx].to_string()
}

fn get_seasonal_loss_message(theme: &str, solution: &str, default_msg: String) -> String {
    let list: Vec<String> = match theme {
        "christmas" => vec![
            format!("You got coal! The word was {}.", solution),
            format!("Grinch stole the victory! The word was {}.", solution),
        ],
        "halloween" => vec![
            format!("Ridley haunted your dreams! The word was {}.", solution),
            format!("Spooked out of guesses! The word was {}.", solution),
        ],
        "easter" => vec![
            format!("Egg cracked! The word was {}.", solution),
            format!(
                "Ridley stole your Easter basket! The word was {}.",
                solution
            ),
        ],
        "thanksgiving" => vec![
            format!("You got carved! The word was {}.", solution),
            format!("Ridley ate the turkey! The word was {}.", solution),
        ],
        "newyear" => vec![
            format!("Time ran out on the countdown! The word was {}.", solution),
            format!("Ridley crashed the party! The word was {}.", solution),
        ],
        "valentine" => vec![
            format!("Heartbroken! The word was {}.", solution),
            format!("Ridley broke your heart! The word was {}.", solution),
        ],
        "independence" => vec![
            format!("Fireworks fizzled! The word was {}.", solution),
            format!("Metroid independence denied! The word was {}.", solution),
        ],
        "stpatrick" => vec![
            format!("Out of luck! The word was {}.", solution),
            format!("Ridley stole your shamrock! The word was {}.", solution),
        ],
        _ => return default_msg,
    };
    let idx = (js_sys::Math::random() * list.len() as f64).floor() as usize;
    list[idx].clone()
}
