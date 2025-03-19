use std::cmp;
use std::time::Duration;

use bevy::prelude::*;

use super::LedAnimationPlayback;

pub type LedFrameSet = Vec<Vec<Color>>;

pub trait LedAnimation {
    // TODO: is it possible to make this an iterator instead of a realized set?
    fn render(&self, led_count: u16, frame_count: u64, fps: u8) -> LedFrameSet;

    fn to_infinite_playback(
        &self,
        duration: Duration,
        entities: Vec<Entity>,
        fps: u8,
    ) -> LedAnimationPlayback {
        let frame_count = calculate_frames(fps, duration);
        let rendered = self.render(entities.len() as u16, frame_count, fps);
        LedAnimationPlayback::new(entities, fps, rendered, None)
    }

    fn to_fixed_playback(
        &self,
        play_count: u8,
        duration: Duration,
        entities: Vec<Entity>,
        fps: u8,
    ) -> LedAnimationPlayback {
        let frame_count = calculate_frames(fps, duration);
        let rendered = self.render(entities.len() as u16, frame_count, fps);
        LedAnimationPlayback::new(entities, fps, rendered, Some(play_count - 1))
    }

    fn to_one_shot(
        &self,
        duration: Duration,
        entities: Vec<Entity>,
        fps: u8,
    ) -> LedAnimationPlayback {
        let frame_count = calculate_frames(fps, duration);
        let rendered = self.render(entities.len() as u16, frame_count, fps);
        LedAnimationPlayback::new(entities, fps, rendered, Some(0))
    }
}

pub(crate) fn calculate_frames(fps: u8, duration: Duration) -> u64 {
    let millis = duration.as_millis() * fps as u128;
    cmp::max(1, millis / 1000) as u64
}

// ---

pub trait PhasedLedAnimation {
    fn sample(&self, led_count: u16, timing: LedAnimationTiming) -> Vec<Color>;
}

pub struct LedAnimationTiming {
    pub phase: f32,
    pub cycle_length: Duration,
    pub fps: u8,
}

impl LedAnimationTiming {
    pub fn total_frame_count_f32(&self) -> f32 {
        self.cycle_length.as_secs_f32() * self.fps as f32
    }

    pub fn total_frame_count(&self) -> u64 {
        cmp::max(1, self.total_frame_count_f32() as u64)
    }

    pub fn current_frame_f32(&self) -> f32 {
        self.total_frame_count_f32() * self.phase
    }

    pub fn current_frame(&self) -> u64 {
        let curr = self.total_frame_count() as f32 * self.phase;
        cmp::max(1, curr.floor() as u64)
    }
}

// TODO: make an animation with Ease + ColorDimension

pub trait ColorDimension<T> {
    fn apply(&self, ease: EasingCurve<T>) -> Color;
}

// -- Iterator --

pub struct LedAnimationIterator {
    led_count: u16,
    phase: f32,
    phase_increment_per_frame: f32,
    cycle_length: Duration,
    fps: u8,
    animation: dyn PhasedLedAnimation,
}

impl Iterator for LedAnimationIterator {
    type Item = Vec<Color>;

    fn next(&mut self) -> Option<Self::Item> {
        self.phase += self.phase_increment_per_frame;
        if self.phase > 1.0 {
            None
        } else {
            Some(self.animation.sample(
                self.led_count,
                LedAnimationTiming {
                    phase: self.phase,
                    cycle_length: self.cycle_length,
                    fps: self.fps,
                },
            ))
        }
    }
}
