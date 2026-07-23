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

//! Canvas-based particle rendering and physics system.
//!
//! Spawns and simulates cosmetic weather effects (such as snow, rain, embers,
//! bubbles, and confetti) that float across the screen and interactively bounce
//! off DOM layout elements like the guess grid and virtual keyboard.

#![allow(deprecated, dead_code, unused_imports, unused_variables)]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, window};

/// Bounding rectangle in viewport coordinates used for physical particle collisions.
pub struct CollisionRect {
    /// Left horizontal boundary coordinate (pixels).
    pub left: f32,
    /// Right horizontal boundary coordinate (pixels).
    pub right: f32,
    /// Top vertical boundary coordinate (pixels).
    pub top: f32,
    /// Bottom vertical boundary coordinate (pixels).
    pub bottom: f32,
}

/// An individual particle entity simulated on the weather canvas.
pub struct Particle {
    /// Horizontal position coordinate.
    pub x: f32,
    /// Vertical position coordinate.
    pub y: f32,
    /// Horizontal velocity (pixels per frame).
    pub vx: f32,
    /// Vertical velocity (pixels per frame).
    pub vy: f32,
    /// Drawing radius/length.
    pub size: f32,
    /// Color hex or CSS rgba string.
    pub color: &'static str,
    /// Optional character/emoji string drawn instead of a geometric shape.
    pub text: Option<&'static str>,
    /// Remaining lifespan (frames).
    pub life: f32,
    /// Maximum lifespan allocated to the particle at birth. Used to calculate opacity.
    pub max_life: f32,
    /// Set to true if this particle is a splash droplet spawned from a rain collision.
    pub is_splash: bool,
}

/// Query viewport coordinates of a DOM element by ID or selector.
/// Returns a [`CollisionRect`] representing its current bounding client box.
pub fn get_element_rect(id_or_class: &str, is_id: bool) -> Option<CollisionRect> {
    let document = window()?.document()?;
    let el = if is_id {
        document.get_element_by_id(id_or_class)
    } else {
        document.query_selector(id_or_class).ok().flatten()
    }?;
    let rect = el.get_bounding_client_rect();
    Some(CollisionRect {
        left: rect.left() as f32,
        right: rect.right() as f32,
        top: rect.top() as f32,
        bottom: rect.bottom() as f32,
    })
}
