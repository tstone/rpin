use std::time::Duration;

use bevy::{color::palettes::css::BLACK, prelude::*};

use super::{animations::Solid, LedAnimation, LedAnimationPlayback};

#[derive(Default)]
pub struct LedAnimationSequence {
    pairs: Vec<(Duration, Box<dyn LedAnimation>)>,
    done: bool, // TODO: just return a new type
}

impl LedAnimationSequence {
    pub fn new() -> Self {
        LedAnimationSequence::default()
    }

    pub fn play<T: LedAnimation + 'static>(mut self, duration: Duration, anim: T) -> Self {
        if self.done {
            panic!("LED animation added after repeating sequence was already specified.");
        }
        self.pairs.push((duration, Box::new(anim)));
        self
    }

    pub fn repeating<T: LedAnimation + 'static>(mut self, duration: Duration, anim: T) -> Self {
        self.pairs.push((duration, Box::new(anim)));
        self.done = true;
        self
    }

    pub fn clear(self) -> Self {
        self.play(
            Duration::from_millis(1),
            Solid {
                color: Color::from(BLACK),
            },
        )
    }

    pub fn to_playback(self, entities: Vec<Entity>, fps: u8) -> LedAnimationPlayback {
        Self::link(entities, fps, self.pairs).unwrap()
    }

    fn link(
        entities: Vec<Entity>,
        fps: u8,
        mut rem: Vec<(Duration, Box<dyn LedAnimation>)>,
    ) -> Option<LedAnimationPlayback> {
        info!("link, rem len: {}", rem.len());
        if rem.len() == 0 {
            return None;
        }

        let (dur, anim) = rem.remove(0);
        let mut next = anim.to_repeated_playback(0, dur, entities.clone(), fps);
        next.next = Self::link(entities, fps, rem).map(|n| Box::new(n));
        Some(next)
    }
}
