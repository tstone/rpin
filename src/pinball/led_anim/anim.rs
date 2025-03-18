use std::cmp;
use std::time::Duration;

use bevy::prelude::*;

use super::LedAnimationPlayback;

pub type LedFrameSet = Vec<Vec<Color>>;

pub trait LedAnimation {
    fn render(&self, led_count: u16, frame_count: u64) -> LedFrameSet;

    fn to_infinite_playback(
        &self,
        entities: Vec<Entity>,
        duration: Duration,
        fps: u8,
    ) -> LedAnimationPlayback {
        let frame_count = calculate_frames(fps, duration);
        let rendered = self.render(entities.len() as u16, frame_count);
        LedAnimationPlayback::new(entities, fps, rendered, None)
    }

    fn to_repeated_playback(
        &self,
        repeat: u8,
        duration: Duration,
        entities: Vec<Entity>,
        fps: u8,
    ) -> LedAnimationPlayback {
        let frame_count = calculate_frames(fps, duration);
        let rendered = self.render(entities.len() as u16, frame_count);
        LedAnimationPlayback::new(entities, fps, rendered, Some(repeat))
    }
}

fn calculate_frames(fps: u8, duration: Duration) -> u64 {
    let millis = duration.as_millis() * fps as u128;
    cmp::max(1, millis / 1000) as u64
}
