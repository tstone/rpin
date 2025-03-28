// use std::{collections::VecDeque, time::Duration};

// use crate::pinball::RgbLed;
// use bevy::prelude::*;


// trait AnimationApplicator<T: AnimatableValue, C> {
//     fn apply(&self, phase: f32, curve: AnimationStage<T>, component: &mut C);
// }

// struct RgbLedColorApplicator;
// impl AnimationApplicator<Srgba, RgbLed> for RgbLedColorApplicator {
//     fn apply(&self, phase: f32, animation: AnimationStage<Srgba>, component: &mut RgbLed) {
//         component.color = animation.sample(phase);
//     }
// }

// struct AnimationStage<T: AnimatableValue> {
//     phase: f32,
//     duration: Duration,
//     curve: Box<dyn Curve>,
//     from: T,
//     to: T,
// }

// impl<T> AnimationStage<T>
// where
//     T: AnimatableValue,
// {
//     pub fn reset(&mut self) {
//         self.phase = 0.0;
//     }
// }

// impl<T> AnimationStage<T>
// where
//     T: AnimatableValue,
// {
//     pub(crate) fn sample(&self, phase: f32) -> T {
//         let sample = self.curve.sample(phase);
//         self.from.blend(&self.to, sample)
//     }
// }

// struct Animation<T: AnimatableValue> {
//     stages: Vec<AnimationStage<T>>,
//     current_stage: usize,
//     delta: f32,
// }

// struct AnimationPlayback<T: AnimatableValue, C> {
//     phase: f32,
//     state: AnimationState,
//     animation: Animation<T>,
//     applicator: Box<dyn AnimationApplicator<T, C>>,
// }

// #[derive(Component)]
// struct Animatable<T: AnimatableValue, C> {
//     queue: VecDeque<AnimationPlayback<T, C>>,
//     active: Option<AnimationPlayback<T, C>>,
// }
