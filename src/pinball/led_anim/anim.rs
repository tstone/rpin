use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;

#[derive(Component, Clone, Debug, Default)]
pub struct LedAnimation {
    pub led_indexes: HashMap<Entity, usize>,
    pub fps: u8,
    pub frames: Vec<Vec<Color>>,
    /// None = infinite, Some = repeat given count then stop
    pub repeat: Option<u8>,
    pub timer: Timer,
    pub current_frame: usize,
    pub next: Option<Box<LedAnimation>>, // sequences are linked lists
}

impl LedAnimation {
    pub fn new(leds: Vec<Entity>) -> Self {
        Self {
            led_indexes: Self::build_led_index(leds),
            ..Default::default()
        }
    }

    pub fn infinite(&self, fps: u8, frames: Vec<Vec<Color>>) -> Self {
        Self {
            fps,
            frames,
            repeat: None,
            timer: Self::timer_from_fps(fps, None),
            ..self.clone()
        }
    }

    pub fn once(&self, fps: u8, frames: Vec<Vec<Color>>) -> Self {
        Self {
            fps,
            frames,
            repeat: Some(1),
            timer: Self::timer_from_fps(fps, Some(1)),
            ..self.clone()
        }
    }

    pub fn repeating(&self, fps: u8, repeat: u8, frames: Vec<Vec<Color>>) -> Self {
        Self {
            fps,
            frames,
            repeat: Some(repeat),
            timer: Self::timer_from_fps(fps, Some(repeat)),
            ..self.clone()
        }
    }

    pub fn append(&mut self, frames: &mut Vec<Vec<Color>>) {
        self.frames.append(frames);
    }

    // Each color of the frame index matches the LED index
    // For fast application of the frame, create a lookup table to go from LED to index
    fn build_led_index(leds: Vec<Entity>) -> HashMap<Entity, usize> {
        leds.iter()
            .enumerate()
            .map(|(i, e)| (*e, i)) // swap Entity to be the key
            .collect::<HashMap<_, _>>()
    }

    fn timer_from_fps(fps: u8, repeat: Option<u8>) -> Timer {
        let mode = if repeat == Some(1) {
            TimerMode::Once
        } else {
            TimerMode::Repeating
        };
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), mode)
    }

    pub(crate) fn last(&self) -> Option<LedAnimation> {
        match &self.next {
            Some(next) => next.last(),
            None => None,
        }
    }

    pub(crate) fn last_mut(&self) -> Option<&mut LedAnimation> {
        match &self.next {
            Some(next) => next.last_mut(),
            None => None,
        }
    }

    pub(crate) fn merge(&self) -> LedAnimation {
        let mut result = self.clone();
        match &self.next {
            Some(next) => {
                let mut next_merged = next.merge();
                result.frames.append(&mut next_merged.frames);
            }
            None => {}
        }

        result
    }
}
