use super::anim::{LedAnimation, LedId};
use colorous::{Color, Gradient};

/// An animation that moves a single LED linearly along a list of LEDs
pub fn generate(gradient: Gradient, leds: Vec<LedId>, fps: u8, repeat: u16) -> LedAnimation {
    let mut frames: Vec<Vec<Color>> = Vec::new();

    for i in 0..leds.len() {
        let mut frame: Vec<Color> = Vec::new();
        for j in 0..leds.len() {
            let offset = wrap(
                i16::try_from(j).unwrap() - i16::try_from(i).unwrap(),
                0,
                i16::try_from(leds.len()).unwrap(),
            );
            let color = gradient.eval_rational(usize::from(offset), leds.len());
            frame.push(color);
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

fn wrap(value: i16, min: i16, max: i16) -> u8 {
    if value > max {
        wrap(value - max, min, max)
    } else if value < min {
        wrap(value + max, min, max)
    } else {
        u8::try_from(value).unwrap()
    }
}
