use colorous::Color;

use super::anim::{LedAnimation, LedId};

/// An animation that moves a single LED linearly along a list of LEDs
pub fn generate(color: Color, leds: Vec<LedId>, fps: u8, repeat: u16) -> LedAnimation {
    let mut frames: Vec<Vec<Color>> = Vec::new();
    for i in 0..leds.len() {
        let mut frame: Vec<Color> = Vec::new();
        for j in 0..leds.len() {
            if i == j {
                frame.push(color);
            } else {
                frame.push(Color { r: 0, b: 0, g: 0 });
            }
        }
        frames.push(frame);
    }

    return LedAnimation {
        leds,
        frames,
        fps,
        repeat,
    };
}
