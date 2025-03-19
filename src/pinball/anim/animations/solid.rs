use bevy::prelude::*;

use crate::pinball::anim::anim::*;

/// Animation which shows a single color
#[derive(Debug, Clone, Default)]
pub struct Solid {
    pub color: Color,
}

impl PhasedLedAnimation for Solid {
    fn sample(&self, led_count: u16, _timing: LedAnimationTiming) -> Vec<Color> {
        vec![self.color; led_count as usize]
    }
}
