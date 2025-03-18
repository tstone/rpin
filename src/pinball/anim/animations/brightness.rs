use bevy::prelude::*;

use crate::pinball::{anim::anim::LedAnimation, LedFrameSet, PhasedLedAnimation};

use super::Easing;

// -- BrightnessEaseIn --

#[derive(Debug, Clone)]
pub struct BrightnessEaseIn {
    pub color: Color,
    pub from: f32,
    pub ease: EaseFunction,
}

impl Default for BrightnessEaseIn {
    fn default() -> Self {
        Self {
            color: Default::default(),
            from: 0.,
            ease: EaseFunction::Linear,
        }
    }
}

impl LedAnimation for BrightnessEaseIn {
    fn render(&self, led_count: u16, frame_count: u64, _fps: u8) -> LedFrameSet {
        ease_brightness(
            self.color,
            Easing {
                from: self.from,
                to: Hsla::from(self.color).lightness,
                easefn: self.ease,
            },
            led_count,
            frame_count,
        )
    }
}

// -- BrightnessEaseOut --

#[derive(Debug, Clone)]
pub struct BrightnessEaseOut {
    pub color: Color,
    pub to: f32,
    pub ease: EaseFunction,
}

impl Default for BrightnessEaseOut {
    fn default() -> Self {
        Self {
            color: Default::default(),
            to: 0.,
            ease: EaseFunction::Linear,
        }
    }
}

impl LedAnimation for BrightnessEaseOut {
    fn render(&self, led_count: u16, frame_count: u64, _fps: u8) -> LedFrameSet {
        ease_brightness(
            self.color,
            Easing {
                from: Hsla::from(self.color).lightness,
                to: self.to,
                easefn: self.ease,
            },
            led_count,
            frame_count,
        )
    }
}

// -- Brightness Curve --

#[derive(Debug, Clone)]
pub(crate) struct Brightness(pub f32);

impl Brightness {
    pub fn curve(from: f32, to: f32, ease_fn: EaseFunction) -> EasingCurve<Self> {
        EasingCurve::new(Brightness(from), Brightness(to), ease_fn)
    }
}

impl Ease for Brightness {
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::EVERYWHERE, move |t| {
            let range = end.0 - start.0;
            let interpolated = range as f32 * t;
            Brightness(start.0 + interpolated)
        })
    }
}

fn ease_brightness(
    color: Color,
    easing: Easing<f32>,
    led_count: u16,
    frame_count: u64,
) -> LedFrameSet {
    let mut frames: Vec<Vec<Color>> = Vec::new();
    let hsl = Hsla::from(color);
    let curve = Brightness::curve(easing.from, easing.to, easing.easefn);

    for i in 0..frame_count {
        let point_on_curve = (i as f32) / (frame_count as f32);
        let frame_lightness = curve.sample_unchecked(point_on_curve).0;
        let hsla = hsl.clone().with_lightness(frame_lightness);

        let mut frame: Vec<Color> = Vec::new();
        for _ in 0..led_count {
            frame.push(Color::from(hsla.clone()));
        }
        frames.push(frame);
    }

    frames
}

// ---

#[derive(Debug, Clone)]
pub struct PhasedBrightnessEaseIn {
    pub color: Color,
    pub from: f32,
    pub ease: EaseFunction,
}

impl PhasedLedAnimation for PhasedBrightnessEaseIn {
    fn render(&self, led_count: u16, timing: crate::pinball::LedAnimationTiming) -> Vec<Color> {
        let hsl = Hsla::from(self.color);
        let curve = Brightness::curve(self.from, hsl.lightness, self.ease);
        let frame_lightness = curve.sample_unchecked(timing.phase).0;
        let hsla = hsl.clone().with_lightness(frame_lightness);

        (0..led_count)
            .map(|_| Color::from(hsla.clone()))
            .collect::<Vec<_>>()
    }
}
