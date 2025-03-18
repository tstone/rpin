use std::time::Duration;

use bevy::{color::palettes::css::BLACK, prelude::*};

use super::{animations::Solid, calculate_frames, LedAnimation, LedAnimationPlayback, LedFrameSet};

#[derive(Default)]
pub struct LedAnimationSequence {
    pairs: Vec<(Duration, Box<dyn LedAnimation>)>,
}

impl LedAnimationSequence {
    pub fn new() -> Self {
        LedAnimationSequence::default()
    }

    pub fn once<T: LedAnimation + 'static>(mut self, duration: Duration, anim: T) -> Self {
        self.pairs.push((duration, Box::new(anim)));
        self
    }

    pub fn forever<T: LedAnimation + 'static>(
        mut self,
        duration: Duration,
        anim: T,
    ) -> LedAnimationSequenceLinked {
        self.pairs.push((duration, Box::new(anim)));
        LedAnimationSequenceLinked { pairs: self.pairs }
    }

    pub fn clear(self) -> Self {
        self.once(
            Duration::from_millis(1),
            Solid {
                color: Color::from(BLACK),
            },
        )
    }

    pub fn to_one_shot(self, entities: Vec<Entity>, fps: u8) -> LedAnimationPlayback {
        let mut frames: LedFrameSet = Vec::new();
        for (dur, anim) in self.pairs {
            let frame_count = calculate_frames(fps, dur);
            frames.append(&mut anim.render(entities.len() as u16, frame_count));
        }
        LedAnimationPlayback::new(entities, fps, frames, Some(0))
    }

    pub fn to_infinite_playback(self, entities: Vec<Entity>, fps: u8) -> LedAnimationPlayback {
        let mut frames: LedFrameSet = Vec::new();
        for (dur, anim) in self.pairs {
            let frame_count = calculate_frames(fps, dur);
            frames.append(&mut anim.render(entities.len() as u16, frame_count));
        }
        LedAnimationPlayback::new(entities, fps, frames, None)
    }

    pub fn to_fixed_playback(
        self,
        entities: Vec<Entity>,
        fps: u8,
        repeat: Option<u8>,
    ) -> LedAnimationPlayback {
        let mut frames: LedFrameSet = Vec::new();
        for (dur, anim) in self.pairs {
            let frame_count = calculate_frames(fps, dur);
            frames.append(&mut anim.render(entities.len() as u16, frame_count));
        }
        LedAnimationPlayback::new(entities, fps, frames, repeat)
    }
}

#[derive(Default)]
pub struct LedAnimationSequenceLinked {
    pairs: Vec<(Duration, Box<dyn LedAnimation>)>,
}

impl LedAnimationSequenceLinked {
    /// Combine sequence to a linked list of playbacks, where the last one repeats
    pub fn to_infinite_playback(self, entities: Vec<Entity>, fps: u8) -> LedAnimationPlayback {
        to_playback_rec(entities, fps, self.pairs, true).unwrap()
    }
}

fn to_playback_rec(
    entities: Vec<Entity>,
    fps: u8,
    mut rem: Vec<(Duration, Box<dyn LedAnimation>)>,
    loop_last: bool,
) -> Option<LedAnimationPlayback> {
    info!("link, rem len: {}", rem.len());
    if rem.len() == 0 {
        return None;
    }

    let (dur, anim) = rem.remove(0);
    let mut next = if loop_last && rem.len() == 1 {
        anim.to_infinite_playback(dur, entities.clone(), fps)
    } else {
        anim.to_fixed_playback(0, dur, entities.clone(), fps)
    };

    next.next = to_playback_rec(entities, fps, rem, loop_last).map(|n| Box::new(n));
    Some(next)
}
