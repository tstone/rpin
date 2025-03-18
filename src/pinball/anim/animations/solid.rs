use bevy::prelude::*;

use crate::pinball::anim::anim::*;

/// Animation which shows a single color
#[derive(Debug, Clone, Default)]
pub struct Solid {
    pub color: Color,
}

impl LedAnimation for Solid {
    fn render(&self, led_count: u16, frame_count: u64, _fps: u8) -> LedFrameSet {
        vec![vec![self.color; led_count as usize]; frame_count as usize]
    }
}
