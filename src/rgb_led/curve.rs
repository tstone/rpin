use bevy::{math::FloatPow, prelude::*};
use std::{f32::consts::PI, fmt::Debug, sync::Arc, time::Duration};

use super::{Animation, AnimationStage};

#[derive(Debug, Default, Clone)]
pub enum Curve {
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    Sinusoid,
    Constant(f32),
    Steps(usize),
    Reverse(Arc<Self>),
    Remap(Arc<Self>, Arc<Self>),
    // Multiply(f32, Arc<Self>),
}

impl Curve {
    pub fn sample(&self, phase: f32) -> f32 {
        match self {
            Self::Linear => phase,
            Self::Constant(c) => *c,
            Self::QuadraticIn => phase.squared(),
            Self::QuadraticOut => 1.0 - (1.0 - phase).squared(),
            Self::QuadraticInOut => sample_quadratic_inout(phase),
            Self::ExponentialIn => ops::powf(2.0, 10.0 * phase - 10.0),
            Self::ExponentialOut => 1.0 - ops::powf(2.0, -10.0 * phase),
            Self::ExponentialInOut => sample_exponential_inout(phase),
            Self::Sinusoid => sample_sinusoid(phase),
            Self::Steps(steps) => sample_steps(*steps, phase),
            Self::Reverse(other) => 1.0 - other.sample(phase),
            Self::Remap(a, b) => a.sample(phase) * b.sample(phase),
            // Self::Multiply(m, curve) =>
        }
    }

    pub fn reverse(self) -> Self {
        Curve::Reverse(Arc::new(self))
    }

    pub fn remap(self, other: Self) -> Self {
        Curve::Remap(Arc::new(self), Arc::new(other))
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

#[inline]
fn sample_sinusoid(phase: f32) -> f32 {
    1.0 - (f32::cos(phase * 2. * PI) + 1.0) / 2.0
}

#[inline]
fn sample_steps(steps: usize, phase: f32) -> f32 {
    (phase * steps as f32).round() / steps.max(1) as f32
}

fn sample_quadratic_inout(phase: f32) -> f32 {
    if phase < 0.5 {
        2.0 * phase.squared()
    } else {
        1.0 - (-2.0 * phase + 2.0).squared() / 2.0
    }
}

fn sample_exponential_inout(phase: f32) -> f32 {
    if phase < 0.5 {
        ops::powf(2.0, 20.0 * phase - 10.0) / 2.0
    } else {
        (2.0 - ops::powf(2.0, -20.0 * phase + 10.0)) / 2.0
    }
}
