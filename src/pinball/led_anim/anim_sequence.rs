use bevy::prelude::*;

use super::LedAnimation;

/// Used to build up an LedAnimationSequence
#[derive(Default, Clone)]
pub struct LedAnimationSequence {
    leds: Vec<Entity>,
    fps: Option<u8>,
    first: Option<LedAnimation>,
}

impl LedAnimationSequence {
    pub fn new(leds: Vec<Entity>) -> Self {
        Self {
            leds: leds,
            ..Default::default()
        }
    }

    pub fn fps(&self, fps: u8) -> Self {
        Self {
            fps: Some(fps),
            ..self.clone()
        }
    }

    pub fn once(&mut self, frames: Vec<Vec<Color>>) -> Self {
        let anim = LedAnimation::new(self.leds.clone()).once(self.fps.unwrap(), frames);
        self.append(anim);
        self.clone()
    }

    pub fn repeating(&mut self, repeat: u8, frames: Vec<Vec<Color>>) -> Self {
        let anim =
            LedAnimation::new(self.leds.clone()).repeating(self.fps.unwrap(), repeat, frames);
        self.append(anim);
        self.clone()
    }

    pub fn infinite(&mut self, frames: Vec<Vec<Color>>) -> Self {
        let anim = LedAnimation::new(self.leds.clone()).infinite(self.fps.unwrap(), frames);
        self.append(anim);
        self.clone()
    }

    fn append(&mut self, anim: LedAnimation) -> Self {
        if let Some(first) = &self.first {
            match first.last_mut() {
                Some(last) => last.next = Some(Box::new(anim)),
                None => {}
            }
        } else {
            self.first = Some(anim);
        }
        self.clone()
    }

    /// Returns the linked list sequence of animations
    pub fn get(&self) -> LedAnimation {
        self.first.clone().unwrap()
    }

    /// Merges all animations down to one. This will use the first FPS encountered
    pub fn merge(&self) -> LedAnimation {
        self.get().merge()
    }
}
