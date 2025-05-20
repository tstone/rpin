use bevy::{color::Srgba, ecs::component::Component};

use crate::pinball::RgbLed;

use super::animation::AnimationStage;

pub type AnimationApplicator<T, C: Component> = fn(f32, &AnimationStage<T>, &mut C);

pub const RgbLedColorApplicator: AnimationApplicator<Srgba, RgbLed> =
    |phase: f32, stage: &AnimationStage<Srgba>, component: &mut RgbLed| {
        let stage_phase = stage.curve.sample(phase);
        component.color = blend_srgba(stage.from, stage.to, stage_phase);
    };

pub fn blend_f32(a: f32, b: f32, phase: f32) -> f32 {
    let distance = (b - a) * phase;
    a + distance
}

pub fn blend_srgba(a: Srgba, b: Srgba, phase: f32) -> Srgba {
    let red = blend_f32(a.red, b.red, phase);
    let green = blend_f32(a.green, b.green, phase);
    let blue = blend_f32(a.blue, b.blue, phase);

    Srgba::rgb(
        wrap_color_channel(red),
        wrap_color_channel(green),
        wrap_color_channel(blue),
    )
}

fn wrap_color_channel(v: f32) -> f32 {
    if v > 1.0 {
        wrap_color_channel(v - 1.0)
    } else {
        v
    }
}
