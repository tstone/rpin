use bevy::prelude::*;

use crate::pinball::{led_anim::anim::LedAnimation, LedFrameSet};

use super::{curves::*, Easing};

/// Animation which applies an easing function to brightness
#[derive(Debug, Clone)]
pub struct EaseBrightness {
    pub color: Color,
    pub easing: Easing<f32>,
}

impl LedAnimation for EaseBrightness {
    fn render(&self, led_count: u16, frame_count: u64) -> LedFrameSet {
        let mut frames: Vec<Vec<Color>> = Vec::new();
        let hsl = Hsla::from(self.color);
        let curve = Lightness::curve(self.easing.from, self.easing.to, self.easing.easefn);

        for i in 0..frame_count {
            let point_on_curve = (i as f32) / (frame_count as f32);
            let frame_lightness = curve.sample_unchecked(point_on_curve).0;
            let hsla = hsl.clone().with_lightness(frame_lightness);

            let mut frame: Vec<Color> = Vec::new();
            for _ in 0..led_count {
                frame.push(Color::from(hsla.clone()));
            }
            frames.push(frame);
        }

        frames
    }
}
