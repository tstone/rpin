use bevy::prelude::*;

use crate::pinball::PhasedLedAnimation;

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

// TODO: can the thing being animated be part of the curve
// And the animations simply apply it, e.g. "EaseIn" for hue
// Dimension -- What is being animated
// Ease -- The function that is mapping the dimension
// Animation -- Iterator that realizes the dimension+curve

/*




*/

// ---

#[derive(Debug, Clone)]
pub struct BrightnessEaseIn {
    pub color: Color,
    pub from: f32,
    pub ease: EaseFunction,
}

impl PhasedLedAnimation for BrightnessEaseIn {
    fn sample(&self, led_count: u16, timing: crate::pinball::LedAnimationTiming) -> Vec<Color> {
        let hsl = Hsla::from(self.color);
        let curve = Brightness::curve(self.from, hsl.lightness, self.ease);
        let frame_lightness = curve.sample_unchecked(timing.phase).0;
        let hsla = hsl.clone().with_lightness(frame_lightness);

        (0..led_count)
            .map(|_| Color::from(hsla.clone()))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
pub struct BrightnessEaseOut {
    pub color: Color,
    pub to: f32,
    pub ease: EaseFunction,
}

impl PhasedLedAnimation for BrightnessEaseOut {
    fn sample(&self, led_count: u16, timing: crate::pinball::LedAnimationTiming) -> Vec<Color> {
        let hsl = Hsla::from(self.color);
        let curve = Brightness::curve(hsl.lightness, self.to, self.ease);
        let frame_lightness = curve.sample_unchecked(timing.phase).0;
        let hsla = hsl.clone().with_lightness(frame_lightness);

        (0..led_count)
            .map(|_| Color::from(hsla.clone()))
            .collect::<Vec<_>>()
    }
}
