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

//! Per-language UI translations for Rustle.
//!
//! The [`Language`] enum, code/label helpers, and the locale persistence
//! (`get_saved_locale` / `set_saved_locale`) are now provided by the
//! shared `shared_core::i18n` and `shared_frontend::locale` modules —
//! Rustle keeps only its Wordle-specific [`Translations`] struct and
//! the per-language files.

pub use shared_core::i18n::Language;

mod de;
mod en;
mod es;
mod fr;
mod ja;
mod pt;
mod ru;
mod zh;

#[derive(Clone, Debug, PartialEq)]
pub struct Translations {
    pub game_title: &'static str,
    pub win_messages: &'static [&'static str],
    pub game_copied: &'static str,
    pub not_enough_letters: &'static str,
    pub word_not_found: &'static str,
    pub hard_mode_alert: &'static str,
    pub enter: &'static str,
    pub delete: &'static str,
    pub statistics: &'static str,
    pub guess_distribution: &'static str,
    pub new_word: &'static str,
    pub share: &'static str,
    pub share_failure: &'static str,
    pub transfer: &'static str,
    pub transfer_desc: &'static str,
    pub total_tries: &'static str,
    pub success_rate: &'static str,
    pub current_streak: &'static str,
    pub best_streak: &'static str,
    pub discourage_browser: &'static str,
    pub datepicker_title: &'static str,
    pub datepicker_choose: &'static str,
    pub logout: &'static str,
}

pub fn get_translations(lang: Language) -> Translations {
    match lang {
        Language::Chinese => zh::translations(),
        Language::Spanish => es::translations(),
        Language::German => de::translations(),
        Language::Japanese => ja::translations(),
        Language::French => fr::translations(),
        Language::Portuguese => pt::translations(),
        Language::Russian => ru::translations(),
        _ => en::translations(),
    }
}

pub fn get_saved_language() -> Language {
    let raw = shared_frontend::detect_browser_locale();
    Language::from_code(&raw)
}

pub fn save_language(lang: Language) {
    shared_frontend::set_saved_locale(lang.code());
}

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub language: Language,
    pub translations: Translations,
}

impl Default for I18nContext {
    fn default() -> Self {
        let language = Language::English;
        let translations = get_translations(language);
        Self {
            language,
            translations,
        }
    }
}

pub fn get_correct_word_message(lang: Language, solution: &str) -> String {
    match lang {
        Language::Spanish => format!("La palabra era {}", solution),
        Language::Chinese => format!("单词是 {}", solution),
        Language::German => format!("Das Wort war {}", solution),
        Language::Japanese => format!("正解は {} でした", solution),
        Language::French => format!("Le mot était {}", solution),
        Language::Portuguese => format!("A palavra era {}", solution),
        Language::Russian => format!("Слово было {}", solution),
        _ => crate::helpers::feedback::loss::get_loss_taunt(solution),
    }
}
