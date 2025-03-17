use bevy::color::palettes;
use bevy::color::Color;
use bevy::prelude::*;

static BLACK: Color = Color::hsl(0., 0., 0.);

// pub fn sequential() -> Vec<Vec<Color>> {}

/// A frame generator which moves a list of colors linearly through
/// all LEDs. One frame will be generated for each combination of colors.
pub fn sequential_linear(led_count: u16, colors: Vec<Color>) -> Vec<Vec<Color>> {
    let mut frames: Vec<Vec<Color>> = Vec::new();
    let mut colors = colors.clone();

    // Insert black (blank) if not enough colors were given
    if led_count as usize > colors.len() {
        let diff = led_count - colors.len() as u16;
        (0..diff).for_each(|_| colors.push(BLACK.clone()));
    }

    let max_color_index = colors.len() - 1;
    for offset in (0..colors.len()).rev() {
        let mut frame: Vec<Color> = Vec::new();
        for i in 0..led_count {
            let color_index = wrap(i as usize + offset, max_color_index);
            frame.push(colors[color_index].clone());
        }
        frames.push(frame);
    }

    // move the last frame to the front which just presents more natural
    frames.insert(0, frames[frames.len() - 1].clone());
    frames.pop();

    frames
}

fn wrap(value: usize, max: usize) -> usize {
    if value > max {
        wrap(value - max - 1, max)
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static RED: Color = Color::hsl(360., 1., 1.);

    #[test]
    fn it_generates_single() {
        let frames = sequential_linear(3, vec![RED]);
        println!("frames: {:?}", frames);

        assert_eq!(frames[0][0], RED);
        assert_eq!(frames[0][1], BLACK);
        assert_eq!(frames[0][2], BLACK);

        assert_eq!(frames[1][0], BLACK);
        assert_eq!(frames[1][1], RED);
        assert_eq!(frames[1][2], BLACK);

        assert_eq!(frames[2][0], BLACK);
        assert_eq!(frames[2][1], BLACK);
        assert_eq!(frames[2][2], RED);

        assert_eq!(frames.len(), 3);
    }
}
