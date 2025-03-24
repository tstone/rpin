use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::pinball::RgbLed;

use super::{math::round_to_nearest, render_tail as render_tail_grad};

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
    /// Illuminates the point an N additional points, fading to black
    Tail(u8),
    /// Illuminates the position, first point, and N points in the middle
    Split(u8),
    // maybe just a regular Noise/NoiseGrad
}

// TODO: can an observer be used here?
pub(crate) fn render_led_sequence(
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
                    LedSequenceFill::Tail(tail_len) => {
                        render_tail(seq.position, i, seq.color, &mut led, tail_len);
                    }
                    LedSequenceFill::Split(n) => {
                        render_split(seq.position, i, seq.color, &mut led, n)
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

fn render_split(active: f32, current: usize, color: Srgba, led: &mut RgbLed, count: u8) {
    if current == 0 {
        led.color = color;
        return;
    }

    let position = round_to_nearest(active);
    // TODO: this doesn't seem to work with count > 1
    let mid = (active / (count + 1) as f32).ceil();
    if current == position as usize || current == mid as usize {
        led.color = color;
    } else {
        led.color = BLACK;
    }
}

fn render_progress(active: f32, current: usize, color: Srgba, led: &mut RgbLed) {
    if current <= round_to_nearest(active) as usize {
        led.color = color;
    } else {
        led.color = BLACK;
    }
}

fn render_tail(active: f32, current: usize, color: Srgba, led: &mut RgbLed, tail_len: u8) {
    // tail_len + 1 since the last color is always black
    render_tail_grad(active, current, color, BLACK, led, tail_len + 1);
}
