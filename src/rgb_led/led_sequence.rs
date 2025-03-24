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
    Single,
    Solid,
    Gradient(Srgba),
    Tail(u8),
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
                    LedSequenceFill::Solid => render_filled(seq.position, i, seq.color, &mut led),
                    LedSequenceFill::Gradient(from_color) => render_gradient(
                        seq.position,
                        i,
                        seq.color,
                        from_color,
                        &mut led,
                        seq.names.len(),
                    ),
                    LedSequenceFill::Tail(tail_len) => {
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

fn render_filled(active: f32, current: usize, color: Srgba, led: &mut RgbLed) {
    if current <= active.floor() as usize {
        led.color = color;
    } else {
        led.color = BLACK;
    }
}

fn render_gradient(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    led_count: usize,
) {
    let rounded = active.floor() as usize;
    if rounded == current {
        led.color = to_color;
    } else if current < rounded {
        let curve = EasingCurve::new(from_color, to_color, EaseFunction::Linear);
        led.color = match curve.sample(current as f32 / led_count as f32) {
            Some(color) => color,
            None => from_color,
        };
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
    let rounded = active.floor() as usize;
    let offset_to = (current as i32 - rounded as i32).abs() as u8;
    if rounded == current {
        led.color = to_color;
    } else if current < rounded && offset_to <= tail_len {
        // TODO: this isn't quite correct
        let curve = EasingCurve::new(to_color, from_color, EaseFunction::Linear);
        led.color = match curve.sample(offset_to as f32 / tail_len as f32) {
            Some(color) => color,
            None => from_color,
        };
    } else {
        led.color = BLACK;
    }
}
