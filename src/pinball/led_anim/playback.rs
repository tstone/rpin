use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;

/// A component for playing LED animations
#[derive(Component, Clone, Debug, Default)]
pub struct LedAnimationPlayback {
    pub led_indexes: HashMap<Entity, usize>,
    pub frames: Vec<Vec<Color>>,
    /// None = infinite, Some = repeat given count then stop
    pub repeat: Option<u8>,
    pub timer: Timer,
    pub current_frame: usize,
    pub next: Option<Box<LedAnimationPlayback>>, // sequences are linked lists
}

impl LedAnimationPlayback {
    pub fn new(leds: Vec<Entity>, fps: u8, frames: Vec<Vec<Color>>, repeat: Option<u8>) -> Self {
        Self {
            led_indexes: Self::build_led_index(leds),
            frames,
            repeat,
            timer: Self::timer_from_fps(fps),
            ..Default::default()
        }
    }

    // Each color of the frame index matches the LED index
    // For fast application of the frame, create a lookup table to go from LED to index
    fn build_led_index(leds: Vec<Entity>) -> HashMap<Entity, usize> {
        leds.iter()
            .enumerate()
            .map(|(i, e)| (*e, i)) // swap Entity to be the key
            .collect::<HashMap<_, _>>()
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }

    /// Get the last item in the linked list of animations
    pub(crate) fn last(&self) -> Option<LedAnimationPlayback> {
        match &self.next {
            Some(next) => next.last(),
            None => None,
        }
    }

    /// Get a mutable reference to the last item in the linked list of animations
    pub(crate) fn last_mut(&self) -> Option<&mut LedAnimationPlayback> {
        match &self.next {
            Some(next) => next.last_mut(),
            None => None,
        }
    }
}
