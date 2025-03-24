use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::pinball::RgbLed;

pub struct LedSequencePlugin;

impl Plugin for LedSequencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_led_seq);
    }
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LedSequence {
    pub position: f32,
    pub color: Srgba,
    pub names: Vec<Name>,
    pub behavior: LedSequenceFill,
}

#[derive(Clone, Debug, Default, Reflect)]
pub enum LedSequenceFill {
    #[default]
    /// Interpolates a single point to render on 1-2 LEDs at position
    Single,
    /// Illuminates all points up to position
    Progress,
    /// Illuminates all points up to position with a gradient of colors (from color)
    ProgressGradient(Srgba),
    /// Illuminates all LEDs from color to color, with the position being the start of the gradient
    Gradient(Srgba),
    /// Illuminates the point an N additional points, fading to black
    TailFade(u8),
    /// Illuminates the point an N additional points, fading to the given color
    TailGradient(u8, Srgba),
}

// TODO: can an observer be used here?
fn render_led_seq(
    sequences: Query<&LedSequence, Changed<LedSequence>>,
    mut leds: Query<(&Name, &mut RgbLed)>,
) {
    // for each space that has changed
    for seq in &sequences {
        // go through the leds to find the ones that are in this space
        for (name, mut led) in &mut leds {
            let indexes = seq
                .names
                .iter()
                .enumerate()
                .filter(|(_, n)| *name == **n)
                .map(|(index, _)| index);

            // then update the color of leds within space
            for i in indexes {
                match seq.behavior {
                    LedSequenceFill::Single => render_single(seq.position, i, seq.color, &mut led),
                    LedSequenceFill::Progress => {
                        render_progress(seq.position, i, seq.color, &mut led)
                    }
                    LedSequenceFill::ProgressGradient(from_color) => render_progress_grad(
                        seq.position,
                        i,
                        seq.color,
                        from_color,
                        &mut led,
                        seq.names.len(),
                    ),
                    LedSequenceFill::TailFade(tail_len) => {
                        render_tail(seq.position, i, seq.color, &mut led, tail_len);
                    }
                    LedSequenceFill::TailGradient(tail_len, from_color) => {
                        render_tail_grad(
                            seq.position,
                            i,
                            seq.color,
                            from_color,
                            &mut led,
                            tail_len,
                        );
                    }
                    LedSequenceFill::Gradient(from_color) => render_grad(
                        seq.position,
                        i,
                        seq.color,
                        from_color,
                        &mut led,
                        seq.names.len(),
                    ),
                }
            }
        }
    }
}

fn render_single(active: f32, current: usize, color: Srgba, led: &mut RgbLed) {
    if active == current as f32 {
        led.color = color;
        return;
    }

    let lower = active.floor() as usize;
    let upper = active.ceil() as usize;

    if current == lower {
        let lum = upper as f32 - active;
        if lum > 0.5 {
            led.color = color.with_luminance(lum);
        } else {
            led.color = color.with_luminance(lum / 4.);
        }
    } else if current == upper {
        let lum = active - lower as f32;
        if lum > 0.5 {
            led.color = color.with_luminance(lum);
        } else {
            led.color = color.with_luminance(lum / 4.);
        }
    } else {
        led.color = BLACK;
    }
}

fn render_progress(active: f32, current: usize, color: Srgba, led: &mut RgbLed) {
    if current <= active.floor() as usize {
        led.color = color;
    } else {
        led.color = BLACK;
    }
}

fn render_progress_grad(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    led_count: usize,
) {
    if current as f32 <= active {
        render_grad(active, current, to_color, from_color, led, led_count)
    } else {
        led.color = BLACK;
    }
}

fn render_tail(active: f32, current: usize, color: Srgba, led: &mut RgbLed, tail_len: u8) {
    // tail_len + 1 since the last color is always black
    render_tail_grad(active, current, color, BLACK, led, tail_len + 1);
}

fn render_tail_grad(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    tail_len: u8,
) {
    if current as f32 <= active {
        render_grad(
            active,
            current,
            to_color,
            from_color,
            led,
            tail_len as usize + 1,
        )
    } else {
        led.color = BLACK;
    }
}

fn render_grad(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    led_count: usize,
) {
    let curve = EasingCurve::new(from_color, to_color, EaseFunction::Linear);
    let ratio = grad_ratio(led_count, current, active);
    led.color = match curve.sample(ratio) {
        Some(color) => color,
        None => BLACK,
    };
}

fn grad_ratio(led_count: usize, current: usize, active: f32) -> f32 {
    let mut ratio = (led_count as f32 - current as f32 + active as f32 + 1.) / led_count as f32;
    if ratio > 1.0 {
        ratio -= 1.0;
    }
    return 1.0 - ratio;
}
