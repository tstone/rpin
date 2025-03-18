use bevy::{color::palettes::css::BLACK, prelude::*};
use std::cmp;

use crate::pinball::{anim::anim::LedAnimation, LedFrameSet};

use super::{BrightnessEaseIn, BrightnessEaseOut, Solid};

// -- OnOff --

/// OnOff - A simple animation that turns an LED on for a color, then off again
/// Each phase (on/off) is half the time, with each phase being half each of ease then state
/// E.g. an OnOff animatino spends 250ms easing in, 250ms on, 250ms easing out, and 250ms off
#[derive(Debug, Clone)]
pub struct OnOff {
    pub color: Color,
    pub ease_in: EaseFunction,
    pub ease_out: EaseFunction,
}

impl Default for OnOff {
    fn default() -> Self {
        Self {
            color: Default::default(),
            ease_in: EaseFunction::QuadraticIn,
            ease_out: EaseFunction::QuadraticOut,
        }
    }
}

impl LedAnimation for OnOff {
    fn render(&self, led_count: u16, frame_count: u64, fps: u8) -> LedFrameSet {
        let quarter_frame_count = cmp::max(frame_count / 4, 1);

        // ease in
        let mut frames: LedFrameSet = BrightnessEaseIn {
            color: self.color,
            from: 0.,
            ease: self.ease_in,
        }
        .render(led_count, quarter_frame_count, fps);

        // "on" state
        frames.append(&mut Solid { color: self.color }.render(led_count, quarter_frame_count, fps));

        // ease out
        frames.append(
            &mut BrightnessEaseOut {
                color: self.color,
                to: 0.,
                ease: self.ease_out,
            }
            .render(led_count, quarter_frame_count, fps),
        );

        // "off" state
        frames.append(
            &mut Solid {
                color: Color::from(BLACK),
            }
            .render(led_count, quarter_frame_count, fps),
        );

        frames
    }
}

// -- Flash --

/// Flash - An animation that flashes an LED at the given frequency
///
/// This animation is primarily intended to be used within a sequence to
/// flash for a given period of time then stop. If using this animation on it's own,
/// consider OnOff which can be repeated.
#[derive(Debug, Clone)]
pub struct Flash {
    pub color: Color,
    pub hz: f32,
    pub ease_in: EaseFunction,
    pub ease_out: EaseFunction,
}

impl Default for Flash {
    fn default() -> Self {
        Self {
            color: Default::default(),
            hz: 1.,
            ease_in: EaseFunction::QuadraticIn,
            ease_out: EaseFunction::QuadraticOut,
        }
    }
}

impl LedAnimation for Flash {
    fn render(&self, led_count: u16, frame_count: u64, fps: u8) -> LedFrameSet {
        let secs = (frame_count as f32) / (fps as f32);
        let cycle_duration = 1. / self.hz;
        let cycle_frame_count = cmp::max(1, (cycle_duration * (fps as f32)) as u64);
        let cycles = (secs / cycle_duration).floor() as u16;

        // TODO: larger hz means longer runtimes not faster flashes

        let mut frames: LedFrameSet = Vec::new();
        for _ in 0..cycles {
            frames.append(
                &mut OnOff {
                    color: self.color,
                    ease_in: self.ease_in,
                    ease_out: self.ease_out,
                }
                .render(led_count, cycle_frame_count, fps),
            );
        }
        frames
    }
}
