use bevy::prelude::*;
use std::{f32::consts::PI, fmt::Debug, rc::Rc, sync::Arc, time::Duration};

use super::{Animation, AnimationStage};

/// This maps phase to another point
pub trait CurveSampler: Debug {
    fn sample(&self, phase: f32) -> f32;
}

#[derive(Debug)]
pub struct Linear;
impl CurveSampler for Linear {
    fn sample(&self, phase: f32) -> f32 {
        phase
    }
}

#[derive(Debug)]
pub struct Constant(f32);
impl CurveSampler for Constant {
    fn sample(&self, _phase: f32) -> f32 {
        self.0
    }
}

#[derive(Debug)]
pub struct Sinusoid;
impl CurveSampler for Sinusoid {
    fn sample(&self, phase: f32) -> f32 {
        1.0 - (f32::cos(phase * 2. * PI) + 1.0) / 2.0
    }
}

#[derive(Debug)]
pub struct Steps(usize);
impl CurveSampler for Steps {
    fn sample(&self, phase: f32) -> f32 {
        // TODO: should this add one?
        let step_size = 1.0 / self.0 as f32;
        let current_step = (phase / step_size).floor();
        let x = 1.0 / (self.0 as f32 - current_step);
        info!("step_size {step_size}, current_step {current_step}, value {x}");
        x
    }
}

#[derive(Debug)]
pub struct Reverse(pub Arc<Curve>);
impl CurveSampler for Reverse {
    fn sample(&self, phase: f32) -> f32 {
        1.0 - self.0.sample(phase)
    }
}

#[derive(Debug, Default, Clone)]
pub enum Curve {
    #[default]
    Linear,
    Constant(f32),
    Sinusoid,
    Reverse(Arc<Curve>),
    Steps(usize),
}

impl Curve {
    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Self::Linear => Linear.sample(phase),
            Self::Constant(c) => Constant(*c).sample(phase),
            Self::Sinusoid => Sinusoid.sample(phase),
            Self::Reverse(other) => Reverse(other.clone()).sample(phase),
            Self::Steps(s) => Steps(*s).sample(phase),
        }
    }

    pub fn reverse(self) -> Curve {
        Curve::Reverse(Arc::new(self))
    }

    pub fn stage<T: Default + Send + Sync>(
        self,
        from: T,
        to: T,
        duration: Duration,
    ) -> AnimationStage<T> {
        AnimationStage {
            from,
            to,
            duration,
            curve: self,
        }
    }

    pub fn animate<T: Default + Send + Sync>(
        self,
        from: T,
        to: T,
        duration: Duration,
    ) -> Animation<T> {
        let stage = self.stage(from, to, duration);
        Animation {
            stages: vec![stage],
            ..Default::default()
        }
    }
}
