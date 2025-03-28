use std::time::Duration;

use bevy::animation::prelude::*;
use bevy::color::palettes::css::BLACK;
use bevy::color::prelude::*;
use bevy::prelude::*;

struct Flasher;

impl Flasher {
    /// Return an curve which continuously flashes the color at the given frequency
    pub fn new(color: Srgba, hz: f32) -> impl AnimationCompatibleCurve<Srgba> {
        let frequency_interval = interval(0., 1. / hz).unwrap();
        AnimatableKeyframeCurve::new(vec![
            (0.0, BLACK),
            (0.25, color),
            (0.5, color),
            (0.75, BLACK),
            (1.0, BLACK),
        ])
        .unwrap()
        .reparametrize_linear(frequency_interval)
        .unwrap()
    }

    /// Produce a curve which flashes for the given duration then stops. This can be used
    /// to "play" a flasher once for a fixed duration
    pub fn fixed(
        color: Srgba,
        hz: f32,
        duration: Duration,
    ) -> impl AnimationCompatibleCurve<Srgba> {
        let cycles = duration.as_secs_f32() / (1. / hz);
        Self::new(color, hz)
            .repeat(cycles.floor() as usize)
            .unwrap()
    }
}
