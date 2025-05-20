use std::{fmt::Debug, slice::Iter, time::Duration};

use bevy::prelude::*;

use super::{applicator::AnimationApplicator, curve::Curve};

#[derive(Debug, Default, Clone)]
pub struct AnimationStage<T: Send + Sync> {
    pub duration: Duration,
    pub from: T,
    pub to: T,
    pub curve: Curve,
}

impl<T> AnimationStage<T>
where
    T: Send + Sync,
{
    pub fn to_animation(self) -> Animation<T> {
        Animation {
            stages: vec![self],
            playback: PlaybackType::OneShot,
        }
    }

    pub fn to(mut self, t: T) -> Self {
        self.to = t;
        self
    }

    pub fn duration(mut self, d: Duration) -> Self {
        self.duration = d;
        self
    }
}

impl<T> AnimationStage<T>
where
    T: Clone + Send + Sync,
{
    pub fn repeat(self, n: u16) -> Animation<T> {
        let stages = (0..n).map(|_| self.clone()).collect::<Vec<_>>();
        Animation {
            stages,
            playback: PlaybackType::OneShot,
        }
    }

    pub fn repeat_for(self, duration: Duration) -> Animation<T> {
        let count = (duration.as_secs_f32() / self.duration.as_secs_f32()).floor() as usize;
        let stages = (0..count).map(|_| self.clone()).collect::<Vec<_>>();
        Animation {
            stages,
            playback: PlaybackType::OneShot,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Animation<T: Send + Sync> {
    pub stages: Vec<AnimationStage<T>>,
    pub playback: PlaybackType,
}

impl<T> Animation<T>
where
    T: Send + Sync,
{
    pub fn continuous() -> Self {
        Animation {
            stages: Vec::new(),
            playback: PlaybackType::Forever,
        }
    }

    pub fn as_continuous(mut self) -> Self {
        self.playback = PlaybackType::Forever;
        self
    }

    pub fn repeat(n: u16) -> Self {
        Animation {
            stages: Vec::new(),
            playback: PlaybackType::Count(n),
        }
    }

    pub fn as_repeated(mut self, n: u16) -> Self {
        self.playback = PlaybackType::Count(n);
        self
    }

    pub fn then(mut self, stage: AnimationStage<T>) -> Self {
        self.stages.push(stage);
        self
    }
}

impl<T> Animation<T>
where
    T: Clone + Send + Sync,
{
    pub fn chain(mut self, other: Animation<T>) -> Self {
        for stage in other.stages {
            self.stages.push(stage.clone());
        }
        self
    }
}

impl<T> Animation<T>
where
    T: Default + Clone + Send + Sync,
{
    pub fn tween(vec: Vec<(T, Duration, Curve)>) -> Self {
        let mut iter = vec.iter();
        let stages = match iter.next() {
            Some(first) => Self::tween_rec(first.clone(), iter),
            None => Vec::new(),
        };
        Animation {
            stages,
            ..Default::default()
        }
    }

    fn tween_rec(
        prev: (T, Duration, Curve),
        mut points: Iter<(T, Duration, Curve)>,
    ) -> Vec<AnimationStage<T>> {
        match points.next() {
            None => Vec::new(),
            Some(next) => {
                let mut rem = Self::tween_rec(next.clone(), points);
                rem.insert(
                    0,
                    AnimationStage {
                        from: prev.0,
                        to: next.0.clone(),
                        curve: prev.2,
                        duration: prev.1,
                    },
                );
                rem
            }
        }
    }

    pub fn keyframes(vec: Vec<(T, Duration)>) -> Self {
        let mut iter = vec.iter();
        let stages = match iter.next() {
            Some(first) => Self::keyframes_rec(first.clone(), iter),
            None => Vec::new(),
        };
        Animation {
            stages,
            ..Default::default()
        }
    }

    fn keyframes_rec(
        prev: (T, Duration),
        mut points: Iter<(T, Duration)>,
    ) -> Vec<AnimationStage<T>> {
        match points.next() {
            None => Vec::new(),
            Some(next) => {
                let mut rem = Self::keyframes_rec(next.clone(), points);
                rem.insert(
                    0,
                    AnimationStage {
                        from: prev.0,
                        to: next.0.clone(),
                        curve: Curve::Linear,
                        duration: prev.1,
                    },
                );
                rem
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnimationPlayback<T: Send + Sync, C: Component> {
    // pub delta: f32,
    pub timer: Timer,
    pub state: PlaybackState,
    pub animation: Animation<T>,
    pub current_stage: usize,
    pub play_count: usize,
    pub applicator: AnimationApplicator<T, C>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PlaybackState {
    #[default]
    Inactive,
    Active,
    Paused,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PlaybackType {
    #[default]
    OneShot,
    Count(u16),
    Forever,
}
