#![allow(dead_code, deprecated)]

pub mod types;
pub use types::*;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

/// Spawn a new particle configured according to the active weather effect.
///
/// Weather styles customize spawn position, velocity limits, colors, and characters:
/// - `rain`: Falls quickly downwards and slightly sideways.
/// - `ember` / `wisp`: Floats upwards from the bottom with gentle sideways sway.
/// - `bubble`: Rises slowly from the bottom, rendered as hollow strokes.
/// - `snowflake`: Falls slowly, experiencing sideways wind drift.
/// - `confetti`: Multi-colored squares falling from the top.
/// - Emojis (`heart`, `shamrock`, `leaf`, `spooky`, `sparkle`): Rendered as text characters.
pub fn spawn_particle(particles: &mut Vec<Particle>, effect: &str, w: f32, h: f32) {
    let r = js_sys::Math::random() as f32;
    let r2 = js_sys::Math::random() as f32;
    let r3 = js_sys::Math::random() as f32;
    let mut p = Particle {
        x: r * w,
        y: -20.0,
        vx: (r2 - 0.5) * 1.0,
        vy: 1.0 + r3 * 1.0,
        size: 3.0,
        color: "#ffffff",
        text: None,
        life: 220.0,
        max_life: 220.0,
        is_splash: false,
    };
    match effect {
        "rain" => {
            p.y = -50.0;
            p.vx = 1.5 + r2 * 0.5;
            p.vy = 12.0 + r3 * 3.0;
            p.size = 15.0;
            p.color = "rgba(165, 243, 252, 0.4)";
        }
        "ember" | "wisp" => {
            p.y = h + 15.0;
            p.vx = (r2 - 0.5) * 1.5;
            p.vy = -1.2 - r3 * 1.5;
            p.size = 2.0 + r2 * 4.0;
            p.color = match (effect, r > 0.5) {
                ("ember", true) => "#f97316",
                ("ember", false) => "#ef4444",
                (_, true) => "#06b6d4",
                (_, false) => "#d946ef",
            };
        }
        "bubble" => {
            p.y = h + 15.0;
            p.vx = (r2 - 0.5) * 0.8;
            p.vy = -0.8 - r3 * 1.2;
            p.size = 4.0 + r2 * 6.0;
            p.color = "rgba(45, 212, 191, 0.4)";
        }
        "spore" | "pollen" => {
            p.vx = (r2 - 0.5) * 1.0;
            p.vy = 0.7 + r3 * 1.0;
            p.size = 2.0 + r2 * 3.0;
            p.color = if effect == "spore" {
                "rgba(34, 197, 94, 0.4)"
            } else {
                "#a3e635"
            };
        }
        "snowflake" => p.size = 1.5 + r2 * 2.5,
        "confetti" => {
            p.vx = (r2 - 0.5) * 2.0;
            p.vy = 2.0 + r3 * 3.0;
            p.size = 5.0;
            p.color = [
                "#f59e0b", "#ef4444", "#3b82f6", "#10b981", "#ec4899", "#a855f7",
            ][(r * 6.0) as usize];
        }
        "heart" => {
            p.y = h + 15.0;
            p.vy = -1.0 - r3 * 1.2;
            p.text = Some(if r > 0.5 { "💖" } else { "❤️" });
        }
        "shamrock" => p.text = Some(if r > 0.5 { "🍀" } else { "☘️" }),
        "leaf" => p.text = Some(["🍂", "🍁", "🍃"][(r * 3.0) as usize]),
        "spooky" => p.text = Some(["👻", "🦇", "🎃", "🕷️"][(r * 4.0) as usize]),
        "sparkle" => p.text = Some("✨"),
        _ => {}
    }
    particles.push(p);
}

/// Update physics state of active particles and render them onto the canvas.
///
/// Handles:
/// 1. Movement: Adds velocities to positions. Adds gravity force to rain splash.
/// 2. Wind: Applies random horizontal wind drift to snowflakes, shamrocks, and leaves.
/// 3. Collision checking:
///    - Downward moving particles check top edges of target bounding boxes (`grid_rect` / `kb_rect`).
///    - If colliding:
///      - Rain droplets immediately die (life=0) and spawn 3 smaller upward-splashing drops.
///      - Other particles bounce off by reversing vertical velocity (`vy = -vy * 0.3`) and adding slight scattering.
///    - Upward moving particles (bubbles/embers) check bottom edges of bounding boxes.
///    - If colliding, they bounce back downwards and reposition just below the boundary.
/// 4. Drawing: Renders lines (rain), circles (snow/bubbles/embers), or emoji characters.
pub fn update_and_draw(
    particles: &mut Vec<Particle>,
    ctx: &CanvasRenderingContext2d,
    w: f32,
    h: f32,
    effect: &str,
    grid_rect: &Option<CollisionRect>,
    kb_rect: &Option<CollisionRect>,
) {
    ctx.clear_rect(0.0, 0.0, w as f64, h as f64);
    let is_rain = effect == "rain";
    let is_bubble = effect == "bubble";
    let mut splashes = Vec::new();

    for p in particles.iter_mut() {
        // Apply physics modifications.
        if p.is_splash {
            // Apply gravity/downward acceleration to splash droplets.
            p.vy += 0.15;
        } else if effect == "snowflake" || effect == "leaf" || effect == "shamrock" {
            // Simulate random wind currents drifting particles sideways.
            p.vx += (js_sys::Math::random() as f32 - 0.5) * 0.1;
        }
        p.x += p.vx;
        p.y += p.vy;
        p.life -= 1.0;
        if p.life <= 0.0 {
            continue;
        }

        // Collision logic with DOM bounding boxes.
        if !p.is_splash {
            // Iterate over collision targets (guess grid and keyboard).
            for rect in [grid_rect.as_ref(), kb_rect.as_ref()].into_iter().flatten() {
                // Case A: Downward moving particle hitting the top surface of a box.
                if p.vy > 0.0
                    && p.y >= rect.top
                    && p.y - p.vy <= rect.top
                    && p.x >= rect.left
                    && p.x <= rect.right
                {
                    if is_rain {
                        // Rain terminates on impact and explodes into splash droplets.
                        p.life = 0.0;
                        for _ in 0..3 {
                            let vx = (js_sys::Math::random() as f32 - 0.5) * 3.0;
                            let vy = -1.5 - js_sys::Math::random() as f32 * 1.5;
                            splashes.push(Particle {
                                x: p.x,
                                y: rect.top - 1.0,
                                vx,
                                vy,
                                size: 1.5,
                                color: "rgba(165, 243, 252, 0.6)",
                                text: None,
                                life: 15.0,
                                max_life: 15.0,
                                is_splash: true,
                            });
                        }
                    } else {
                        // Non-rain particles bounce upwards with energy dampening (0.3).
                        p.y = rect.top - 2.0;
                        p.vy = -p.vy * 0.3;
                        p.vx += (js_sys::Math::random() as f32 - 0.5) * 1.0;
                    }
                }
                // Case B: Upward moving particle hitting the bottom surface of a box.
                if p.vy < 0.0
                    && p.y <= rect.bottom
                    && p.y - p.vy >= rect.bottom
                    && p.x >= rect.left
                    && p.x <= rect.right
                {
                    p.y = rect.bottom + 2.0;
                    p.vy = -p.vy * 0.3;
                    p.vx += (js_sys::Math::random() as f32 - 0.5) * 1.0;
                }
            }
        }

        // Prune particles that fly too far off-screen.
        if p.y > h + 50.0 || p.y < -50.0 || p.x < -50.0 || p.x > w + 50.0 {
            p.life = 0.0;
            continue;
        }

        // Map particle life percentage directly to canvas opacity/transparency.
        ctx.set_global_alpha((p.life / p.max_life).min(1.0) as f64);

        // Draw particle representation.
        if let Some(txt) = p.text {
            // Render text/emoji character.
            ctx.set_font("16px sans-serif");
            let _ = ctx.fill_text(txt, p.x as f64, p.y as f64);
        } else if is_rain {
            // Render rain line segment indicating velocity direction.
            ctx.set_stroke_style(&JsValue::from_str(p.color));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            ctx.move_to(p.x as f64, p.y as f64);
            ctx.line_to((p.x - p.vx) as f64, (p.y - p.vy) as f64);
            ctx.stroke();
        } else {
            // Render geometric circle (snow/bubbles/embers).
            ctx.set_fill_style(&JsValue::from_str(p.color));
            ctx.begin_path();
            let _ = ctx.arc(
                p.x as f64,
                p.y as f64,
                p.size as f64,
                0.0,
                std::f64::consts::TAU,
            );
            if is_bubble {
                // Bubbles are hollow stroked shapes.
                ctx.set_stroke_style(&JsValue::from_str(p.color));
                ctx.set_line_width(1.0);
                ctx.stroke();
            } else {
                ctx.fill();
            }
        }
    }

    // Cleanup expired particles and append new splashes.
    particles.retain(|p| p.life > 0.0);
    particles.extend(splashes);
}
