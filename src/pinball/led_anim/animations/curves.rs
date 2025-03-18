use bevy::math::curve::*;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct FrameCount(pub u64);

impl FrameCount {
    pub fn curve(from: u64, to: u64, ease_fn: EaseFunction) -> EasingCurve<Self> {
        EasingCurve::new(FrameCount(from), FrameCount(to), ease_fn)
    }
}

impl Ease for FrameCount {
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::EVERYWHERE, move |t| {
            let range = end.0 - start.0;
            let interpolated = (range as f32 * t).floor();
            FrameCount(start.0 + interpolated as u64)
        })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Lightness(pub f32);

impl Lightness {
    pub fn curve(from: f32, to: f32, ease_fn: EaseFunction) -> EasingCurve<Self> {
        EasingCurve::new(Lightness(from), Lightness(to), ease_fn)
    }
}

impl Ease for Lightness {
    fn interpolating_curve_unbounded(start: Self, end: Self) -> impl Curve<Self> {
        FunctionCurve::new(Interval::EVERYWHERE, move |t| {
            let range = end.0 - start.0;
            let interpolated = range as f32 * t;
            Lightness(start.0 + interpolated)
        })
    }
}
