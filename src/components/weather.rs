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

use yew::prelude::*;
use web_sys::{window, Element};
use gloo_timers::callback::{Interval, Timeout};

#[derive(Properties, PartialEq, Clone)]
pub struct WeatherContainerProps {
    pub theme: String,
    pub is_active: bool,
}

#[function_component(WeatherContainer)]
pub fn weather_container(props: &WeatherContainerProps) -> Html {
    let container_ref = use_node_ref();
    let theme = props.theme.clone();
    let is_active = props.is_active;

    {
        let container_ref = container_ref.clone();
        use_effect_with((theme, is_active), move |(theme, is_active)| {
            let container_opt = container_ref.cast::<Element>();
            let mut opt_interval = None;

            if let Some(container) = &container_opt {
                // Clear any old particles
                container.set_inner_html("");

                if *is_active {
                    // Determine the effect type based on theme
                    let effect = match theme.as_str() {
                        "crateria" => "rain",
                        "brinstar" => "pollen",
                        "norfair" => "ember",
                        "wrecked_ship" => "wisp",
                        "maridia" => "bubble",
                        "tourian" => "spore",
                        "newyear" => "confetti",
                        "valentine" => "heart",
                        "stpatrick" => "shamrock",
                        "easter" => "easteregg",
                        "independence" => "sparkle",
                        "halloween" => "spooky",
                        "thanksgiving" => "leaf",
                        "christmas" => "snowflake",
                        _ => "rain",
                    };

                    // Spawning closure
                    let spawn_particle = {
                        let container = container.clone();
                        let effect = effect.to_string();
                        move || {
                            let window = match window() {
                                Some(w) => w,
                                None => return,
                            };
                            let document = match window.document() {
                                Some(d) => d,
                                None => return,
                            };
                            let p = match document.create_element("div") {
                                Ok(el) => el,
                                Err(_) => return,
                            };

                            let rand = js_sys::Math::random();
                            let rand2 = js_sys::Math::random();
                            let rand3 = js_sys::Math::random();

                            match effect.as_str() {
                                "rain" => {
                                    p.set_class_name("particle rain-drop");
                                    let left = rand * 110.0 - 5.0;
                                    let duration = 0.6 + rand2 * 0.4;
                                    let style = format!("left: {:.2}%; top: -50px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "ember" => {
                                    p.set_class_name("particle ember");
                                    let diam = 4.0 + rand * 8.0;
                                    let left = rand2 * 100.0;
                                    let duration = 3.0 + rand3 * 3.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; bottom: -20px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "bubble" => {
                                    p.set_class_name("particle bubble");
                                    let diam = 8.0 + rand * 16.0;
                                    let left = rand2 * 100.0;
                                    let duration = 4.0 + rand3 * 4.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; bottom: -30px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "spore" => {
                                    p.set_class_name("particle spore");
                                    let diam = 6.0 + rand * 10.0;
                                    let left = rand2 * 100.0;
                                    let duration = 5.0 + rand3 * 5.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; top: -20px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "snowflake" => {
                                    p.set_class_name("particle snowflake");
                                    let diam = 3.0 + rand * 6.0;
                                    let left = rand2 * 100.0;
                                    let duration = 4.0 + rand3 * 5.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; top: -10px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "heart" => {
                                    p.set_class_name("particle heart-particle");
                                    let emoji = if rand > 0.5 { "💖" } else { "❤️" };
                                    p.set_text_content(Some(emoji));
                                    let left = rand2 * 100.0;
                                    let duration = 3.0 + rand3 * 3.0;
                                    let style = format!("left: {:.2}%; bottom: -20px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "shamrock" => {
                                    p.set_class_name("particle leaf-particle");
                                    let emoji = if rand > 0.5 { "🍀" } else { "☘️" };
                                    p.set_text_content(Some(emoji));
                                    let left = rand2 * 100.0;
                                    let duration = 4.0 + rand3 * 4.0;
                                    let style = format!("left: {:.2}%; top: -30px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "leaf" => {
                                    p.set_class_name("particle leaf-particle");
                                    let leaves = ["🍂", "🍁", "🍃"];
                                    let idx = (rand * leaves.len() as f64).floor() as usize;
                                    p.set_text_content(Some(leaves[idx]));
                                    let left = rand2 * 100.0;
                                    let duration = 5.0 + rand3 * 5.0;
                                    let style = format!("left: {:.2}%; top: -30px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "spooky" => {
                                    p.set_class_name("particle spooky-particle");
                                    let symbols = ["👻", "🦇", "🎃", "🕷️"];
                                    let idx = (rand * symbols.len() as f64).floor() as usize;
                                    p.set_text_content(Some(symbols[idx]));
                                    let left = rand2 * 100.0;
                                    let duration = 6.0 + rand3 * 4.0;
                                    let style = format!("left: {:.2}%; top: -30px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "confetti" => {
                                    p.set_class_name("particle confetti-particle");
                                    let colors = ["#f59e0b", "#ef4444", "#3b82f6", "#10b981", "#ec4899", "#a855f7"];
                                    let idx = (rand * colors.len() as f64).floor() as usize;
                                    let left = rand2 * 100.0;
                                    let duration = 2.0 + rand3 * 3.0;
                                    let style = format!("background-color: {}; left: {:.2}%; top: -15px; animation-duration: {:.2}s;", colors[idx], left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "easteregg" => {
                                    p.set_class_name("particle spooky-particle");
                                    let symbols = ["🥚", "🌸", "🐰", "🌷"];
                                    let idx = (rand * symbols.len() as f64).floor() as usize;
                                    p.set_text_content(Some(symbols[idx]));
                                    let left = rand2 * 100.0;
                                    let duration = 5.0 + rand3 * 4.0;
                                    let style = format!("left: {:.2}%; top: -30px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "sparkle" => {
                                    p.set_class_name("particle sparkle-particle");
                                    p.set_text_content(Some("✨"));
                                    let left = rand2 * 100.0;
                                    let duration = 3.0 + rand3 * 2.0;
                                    let style = format!("left: {:.2}%; top: -20px; animation-duration: {:.2}s;", left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "pollen" => {
                                    p.set_class_name("particle pollen-particle");
                                    let diam = 3.0 + rand * 4.0;
                                    let left = rand2 * 100.0;
                                    let duration = 4.0 + rand3 * 4.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; top: -20px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                "wisp" => {
                                    p.set_class_name("particle wisp-particle");
                                    let diam = 6.0 + rand * 8.0;
                                    let left = rand2 * 100.0;
                                    let duration = 3.0 + rand3 * 3.0;
                                    let style = format!("width: {:.2}px; height: {:.2}px; left: {:.2}%; bottom: -20px; animation-duration: {:.2}s;", diam, diam, left, duration);
                                    let _ = p.set_attribute("style", &style);
                                }
                                _ => {}
                            }

                            let _ = container.append_child(&p);

                            let p_clone = p.clone();
                            Timeout::new(8000, move || {
                                p_clone.remove();
                            })
                            .forget();
                        }
                    };

                    for _ in 0..25 {
                        spawn_particle();
                    }

                    let rate = if effect == "rain" || effect == "snowflake" || effect == "confetti" {
                        40
                    } else {
                        150
                    };
                    opt_interval = Some(Interval::new(rate, spawn_particle));
                }
            }

            let cleanup_container = container_opt.clone();
            move || {
                if let Some(interval) = opt_interval {
                    drop(interval);
                }
                if let Some(container) = cleanup_container {
                    container.set_inner_html("");
                }
            }
        });
    }

    html! {
        <div ref={container_ref} id="weather-container"></div>
    }
}
