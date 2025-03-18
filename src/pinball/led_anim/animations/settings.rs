use std::time::Duration;

use bevy::math::curve::*;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnimationDuration {
    Frames(u64),
    Duration { length: Duration, fps: u8 },
}

impl AnimationDuration {
    pub fn to_frame_count(&self) -> u64 {
        match self {
            Self::Frames(c) => *c as u64,
            Self::Duration { length, fps } => {
                let millis = length.as_millis() * *fps as u128;
                (millis / 1000) as u64
            }
        }
    }

    pub fn half(&self) -> AnimationDuration {
        match self {
            Self::Frames(f) => Self::Frames(f / 2),
            Self::Duration { length, fps } => Self::Duration {
                length: Duration::from_secs_f64(length.as_secs_f64() / 2.),
                fps: *fps,
            },
        }
    }

    pub fn secs(ms: u64, fps: u8) -> Self {
        Self::Duration {
            length: Duration::from_secs(ms),
            fps,
        }
    }

    pub fn millis(ms: u64, fps: u8) -> Self {
        Self::Duration {
            length: Duration::from_millis(ms),
            fps,
        }
    }

    pub fn frames(count: u64) -> Self {
        Self::Frames(count)
    }
}

impl Default for AnimationDuration {
    fn default() -> Self {
        AnimationDuration::Duration {
            length: Duration::from_secs(1),
            fps: 12,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AnimationSettings {
    pub duration: AnimationDuration,
    pub ease_in: Option<EaseFunction>,
    pub ease_out: Option<EaseFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Easing<T> {
    pub from: T,
    pub to: T,
    pub easefn: EaseFunction,
}
