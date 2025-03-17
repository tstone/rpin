use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct LEDAnimation {
    pub led_indexes: HashMap<Entity, usize>,
    pub frames: Vec<Vec<Hsla>>,
    /// None = infinite, Some = repeat given count then stop
    pub repeat: Option<u8>,
    pub timer: Timer,
    pub current_frame: usize,
    pub end_behavior: EndBehavior,
    pub previous_colors: Vec<Hsla>,
}

impl LEDAnimation {
    pub fn new(
        fps: u8,
        repeat: Option<u8>,
        leds: Vec<Entity>,
        frames: Vec<Vec<Hsla>>,
        end_behavior: EndBehavior,
    ) -> Self {
        // Each color of the frame index matches the LED index
        // For fast application of the frame, create a lookup table to go from LED to index
        let led_indexes = leds
            .iter()
            .enumerate()
            .map(|(i, e)| (*e, i)) // swap Entity to be the key
            .collect::<HashMap<_, _>>();

        Self {
            led_indexes,
            frames,
            repeat,
            timer: Self::timer_from_fps(fps, repeat),
            current_frame: 0,
            end_behavior,
            previous_colors: Vec::new(),
        }
    }

    fn timer_from_fps(fps: u8, repeat: Option<u8>) -> Timer {
        let mode = if repeat == Some(1) {
            TimerMode::Once
        } else {
            TimerMode::Repeating
        };
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), mode)
    }
}

impl Default for LEDAnimation {
    fn default() -> Self {
        Self::new(12, None, Vec::new(), Vec::new(), EndBehavior::default())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum EndBehavior {
    Black,
    Previous,
    #[default]
    Nothing,
}
