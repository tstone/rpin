use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::pinball::LedAnimationSequence;

use super::{curves::*, AnimationDuration, AnimationSettings, Easing};

/// A frame generator which creates an animation which transitions brightness via the given easing function
pub fn transition_brightness(
    led_count: u16,
    color: Color,
    duration: AnimationDuration,
    easing: Easing<f32>,
) -> Vec<Vec<Color>> {
    let frame_count = duration.to_frame_count();
    let mut frames: Vec<Vec<Color>> = Vec::new();
    let hsl = Hsla::from(color);
    let curve = Lightness::curve(easing.from, easing.to, easing.easefn);

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

// TODO:
// pub fn transition_to_lightness(ease_in_duration, remaining_duration)
// pub fn transition_from_lightness(remaining_duration, ease_out_duration)

/// LED animation builder which turns 1 or more LEDs on or off, applying the given easing functions
/// If easing functions are given, the animation is allocated as quarters: ease in, on, ease out, off.
/// If easing functions are not given, the animation is allocated to halves: on, off.
pub fn flash(led_count: u16, color: Color, settings: AnimationSettings) -> Vec<Vec<Color>> {
    let mut frames: Vec<Vec<Color>> = Vec::new();
    let half_frames = settings.duration.half();

    // "ON" phase
    if let Some(ease_in) = settings.ease_in {
        // fill first half with ease-in
        let ease_in_frames = transition_brightness(
            led_count,
            color,
            half_frames.clone(),
            Easing {
                from: 0.,
                to: Hsla::from(color).lightness,
                easefn: ease_in,
            },
        );
        frames = ease_in_frames;
        info!("frames: {:?}", frames);

        // fill second half with "on" color
        fill_frames(color, &half_frames.half(), led_count, &mut frames);
    } else {
        fill_frames(color, &half_frames, led_count, &mut frames);
    }

    // "OFF" phase
    if let Some(ease_out) = settings.ease_out {
        // fill first half with ease-in
        let mut ease_out_frames = transition_brightness(
            led_count,
            color,
            half_frames.clone(),
            Easing {
                from: Hsla::from(color).lightness,
                to: 0.,
                easefn: ease_out,
            },
        );
        frames.append(&mut ease_out_frames);

        // fill second half with "of" color
        fill_frames(
            Color::from(BLACK),
            &half_frames.half(),
            led_count,
            &mut frames,
        );
    } else {
        fill_frames(Color::from(BLACK), &half_frames, led_count, &mut frames);
    }

    frames
}

/// Fill n frames with the given color
fn fill_frames(
    color: Color,
    frame_count: &AnimationDuration,
    led_count: u16,
    frames: &mut Vec<Vec<Color>>,
) {
    for _ in 0..frame_count.to_frame_count() {
        let mut frame: Vec<Color> = Vec::new();
        for _ in 0..led_count {
            frame.push(color.clone());
        }
        frames.push(frame);
    }
}
