use bevy::prelude::*;

use super::AnimationDuration;

/// Frame builder that displays a single color for the given duration
pub fn solid(led_count: u16, color: Color, duration: AnimationDuration) -> Vec<Vec<Color>> {
    let mut frames: Vec<Vec<Color>> = Vec::new();

    let frame_count = duration.to_frame_count();
    for _ in 0..frame_count {
        let mut frame: Vec<Color> = Vec::new();
        for _ in 0..led_count {
            frame.push(color.clone());
        }
        frames.push(frame);
    }

    frames
}
