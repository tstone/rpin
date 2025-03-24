use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::pinball::RgbLed;

use super::math::round_to_nearest;

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LedGradient {
    pub position: f32,
    pub from_color: Srgba,
    pub to_color: Srgba,
    pub names: Vec<Name>,
    pub behavior: LedGradientFill,
}

#[derive(Clone, Debug, Default, Reflect)]
pub enum LedGradientFill {
    #[default]
    /// Illuminates all LEDs from color to color, with the position being the start of the gradient
    Continuous,
    /// Illuminates a gradienton points up to and including the position
    Progress,
    /// Illuminates the point and N additional points gradiented downward
    Tail(u8),
}

// TODO: can an observer be used here?
pub(crate) fn render_led_gradient(
    gradients: Query<&LedGradient, Changed<LedGradient>>,
    mut leds: Query<(&Name, &mut RgbLed)>,
) {
    // for each space that has changed
    for grad in &gradients {
        // go through the leds to find the ones that are in this space
        for (name, mut led) in &mut leds {
            let indexes = grad
                .names
                .iter()
                .enumerate()
                .filter(|(_, n)| *name == **n)
                .map(|(index, _)| index);

            // then update the color of leds within space
            for i in indexes {
                match grad.behavior {
                    LedGradientFill::Progress => {
                        render_progress(grad.position, i, grad.to_color, grad.from_color, &mut led)
                    }
                    LedGradientFill::Tail(tail_len) => {
                        render_tail(
                            grad.position,
                            i,
                            grad.to_color,
                            grad.from_color,
                            &mut led,
                            tail_len,
                        );
                    }
                    LedGradientFill::Continuous => render_continuous(
                        grad.position,
                        i,
                        grad.to_color,
                        grad.from_color,
                        &mut led,
                        grad.names.len(),
                    ),
                }
            }
        }
    }
}

pub(crate) fn render_progress(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
) {
    let position = round_to_nearest(active);
    if current as f32 == position {
        led.color = to_color;
    } else if current as f32 <= active {
        let curve = EasingCurve::new(from_color, to_color, EaseFunction::Linear);
        let ratio = grad_ratio(position as usize, current, position);
        led.color = match curve.sample(1.0 - ratio) {
            Some(color) => color,
            None => BLACK,
        };
    } else {
        led.color = BLACK;
    }
}

pub(crate) fn render_tail(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    tail_len: u8,
) {
    let position = round_to_nearest(active);
    if current as f32 == position {
        led.color = to_color;
    } else if current as f32 <= active {
        let curve = EasingCurve::new(from_color, to_color, EaseFunction::Linear);
        let ratio = grad_ratio(tail_len as usize, current, position);
        led.color = match curve.sample(1.0 - ratio) {
            Some(color) => color,
            None => BLACK,
        };
    } else {
        led.color = BLACK;
    }
}

pub(crate) fn render_continuous(
    active: f32,
    current: usize,
    to_color: Srgba,
    from_color: Srgba,
    led: &mut RgbLed,
    led_count: usize,
) {
    let position = round_to_nearest(active);
    let curve = EasingCurve::new(to_color, from_color, EaseFunction::Linear);
    let ratio = grad_ratio(led_count, current, position);
    led.color = match curve.sample(ratio) {
        Some(color) => color,
        None => BLACK,
    };
}

fn grad_ratio(led_count: usize, current: usize, position: f32) -> f32 {
    let mut ratio = (led_count as f32 - current as f32 + position) / led_count as f32;
    if ratio > 1.0 {
        ratio -= 1.0;
    }
    return ratio;
}
