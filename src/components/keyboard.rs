// Copyright (C) 2026 Jeryd
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

use crate::constants::config::REVEAL_TIME_MS;
use crate::helpers::local_storage::get_stored_is_high_contrast_mode;
use crate::helpers::statuses::{get_statuses, CharStatus};
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct KeyProps {
    #[prop_or_default]
    pub children: Html,
    pub value: String,
    #[prop_or(40)]
    pub width: u32,
    #[prop_or_default]
    pub status: Option<CharStatus>,
    pub on_click: Callback<String>,
    #[prop_or_default]
    pub is_revealing: bool,
    pub solution_len: usize,
}

#[function_component(Key)]
pub fn key_btn(props: &KeyProps) -> Html {
    let value = props.value.clone();
    let on_click = props.on_click.clone();
    let key_delay_ms = REVEAL_TIME_MS as usize * props.solution_len;
    let is_high_contrast = get_stored_is_high_contrast_mode();
    let status = props.status;

    let is_special = value == "ENTER" || value == "DELETE";
    let text_size_class = if is_special {
        "text-[10px] sm:text-xs px-0.5"
    } else {
        "text-sm sm:text-base"
    };

    let key_classes = classes!(
        "xxshort:h-9",
        "xshort:h-11",
        "short:h-12",
        "h-14",
        "sm:h-16",
        "flex",
        "items-center",
        "justify-center",
        "rounded",
        "mx-0.5",
        text_size_class,
        "font-bold",
        "cursor-pointer",
        "select-none",
        "text-slate-900",
        "dark:text-white",
        "shadow-sm",
        "active:scale-95",
        "transition-transform",
        if props.is_revealing {
            "transition ease-in-out"
        } else {
            ""
        },
        if status.is_none() {
            "bg-slate-200 dark:bg-slate-600 hover:bg-slate-300 active:bg-slate-400"
        } else {
            ""
        },
        if status == Some(CharStatus::Absent) {
            "bg-slate-400 dark:bg-slate-800 text-white"
        } else {
            ""
        },
        if status == Some(CharStatus::Correct) && is_high_contrast {
            "bg-orange-500 hover:bg-orange-600 active:bg-orange-700 text-white"
        } else {
            ""
        },
        if status == Some(CharStatus::Present) && is_high_contrast {
            "bg-cyan-500 hover:bg-cyan-600 active:bg-cyan-700 text-white"
        } else {
            ""
        },
        if status == Some(CharStatus::Correct) && !is_high_contrast {
            "bg-green-500 hover:bg-green-600 active:bg-green-700 text-white"
        } else {
            ""
        },
        if status == Some(CharStatus::Present) && !is_high_contrast {
            "bg-yellow-500 hover:bg-yellow-600 active:bg-yellow-700 text-white"
        } else {
            ""
        }
    );

    let transition_delay = if props.is_revealing {
        format!("transition-delay: {}ms;", key_delay_ms)
    } else {
        "unset".to_string()
    };
    let flex_style = format!("flex: {} 1 0%; min-width: 0;", props.width);
    let styles = format!("{} {}", transition_delay, flex_style);

    let val = value.clone();
    let click_handler = Callback::from(move |e: MouseEvent| {
        if let Some(target) = e.target() {
            if let Ok(btn) = target.dyn_into::<web_sys::HtmlButtonElement>() {
                let _ = btn.blur();
            }
        }
        on_click.emit(val.clone());
    });

    html! {
        <button
            style={styles}
            aria-label={format!("{}{}", value, status.map(|s| format!(" {:?}", s)).unwrap_or_default())}
            class={key_classes}
            onclick={click_handler}
        >
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct KeyboardProps {
    pub on_char: Callback<char>,
    pub on_delete: Callback<()>,
    pub on_enter: Callback<()>,
    pub solution: String,
    pub guesses: Vec<String>,
    #[prop_or_default]
    pub is_revealing: bool,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let solution = props.solution.clone();
    let guesses = props.guesses.clone();
    let on_char = props.on_char.clone();
    let on_delete = props.on_delete.clone();
    let on_enter = props.on_enter.clone();

    {
        let on_char = on_char.clone();
        let on_delete = on_delete.clone();
        let on_enter = on_enter.clone();

        use_effect_with((), move |_| {
            let listener =
                gloo_events::EventListener::new(&gloo_utils::window(), "keyup", move |e| {
                    if let Some(ke) = e.dyn_ref::<web_sys::KeyboardEvent>() {
                        let code = ke.code();
                        if code == "Enter" {
                            on_enter.emit(());
                        } else if code == "Backspace" {
                            on_delete.emit(());
                        } else {
                            let key = ke.key();
                            if key.len() == 1 {
                                if let Some(c) = key.chars().next() {
                                    if c.is_ascii_alphabetic() {
                                        on_char.emit(c.to_ascii_uppercase());
                                    }
                                }
                            }
                        }
                    }
                });

            move || drop(listener)
        });
    }

    let char_statuses = get_statuses(&solution, &guesses);
    let solution_len = solution.chars().count();

    let click_key = {
        let on_char = on_char.clone();
        let on_delete = on_delete.clone();
        let on_enter = on_enter.clone();
        Callback::from(move |value: String| {
            if value == "ENTER" {
                on_enter.emit(());
            } else if value == "DELETE" {
                on_delete.emit(());
            } else {
                if let Some(c) = value.chars().next() {
                    on_char.emit(c);
                }
            }
        })
    };

    let row1 = ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
    let row2 = ["A", "S", "D", "F", "G", "H", "J", "K", "L"];
    let row3 = ["Z", "X", "C", "V", "B", "N", "M"];

    html! {
        <div class="keyboard-container mx-auto w-full max-w-[520px] px-1 select-none">
            <div class="mb-1 flex justify-center w-full">
                {for row1.into_iter().map(|key| {
                    let c = key.chars().next().unwrap_or('\0');
                    let status = char_statuses.get(&c).copied();
                    html! {
                        <Key
                            value={key.to_string()}
                            status={status}
                            on_click={click_key.clone()}
                            is_revealing={props.is_revealing}
                            solution_len={solution_len}
                        >
                            {html! { {key} }}
                        </Key>
                    }
                })}
            </div>
            <div class="mb-1 flex justify-center w-full">
                <div class="flex mx-0.5" style="flex: 20 1 0%; pointer-events: none;"></div>
                {for row2.into_iter().map(|key| {
                    let c = key.chars().next().unwrap_or('\0');
                    let status = char_statuses.get(&c).copied();
                    html! {
                        <Key
                            value={key.to_string()}
                            status={status}
                            on_click={click_key.clone()}
                            is_revealing={props.is_revealing}
                            solution_len={solution_len}
                        >
                            {html! { {key} }}
                        </Key>
                    }
                })}
                <div class="flex mx-0.5" style="flex: 20 1 0%; pointer-events: none;"></div>
            </div>
            <div class="flex justify-center w-full">
                <Key
                    width={60}
                    value="ENTER"
                    on_click={click_key.clone()}
                    solution_len={solution_len}
                >
                    {html! { {crate::constants::config::ENTER_TEXT} }}
                </Key>
                {for row3.into_iter().map(|key| {
                    let c = key.chars().next().unwrap_or('\0');
                    let status = char_statuses.get(&c).copied();
                    html! {
                        <Key
                            value={key.to_string()}
                            status={status}
                            on_click={click_key.clone()}
                            is_revealing={props.is_revealing}
                            solution_len={solution_len}
                        >
                            {html! { {key} }}
                        </Key>
                    }
                })}
                <Key
                    width={60}
                    value="DELETE"
                    on_click={click_key.clone()}
                    solution_len={solution_len}
                >
                    {html! { {crate::constants::config::DELETE_TEXT} }}
                </Key>
            </div>
        </div>
    }
}
