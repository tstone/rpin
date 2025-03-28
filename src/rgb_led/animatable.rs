use std::{collections::VecDeque, time::Duration};

use bevy::prelude::*;

use crate::pinball::RgbLed;

use super::{
    animation::{Animation, AnimationPlayback, PlaybackState, PlaybackType},
    applicator::{AnimationApplicator, RgbLedColorApplicator},
};

#[derive(Component)]
pub struct Animatable<T: Send + Sync, C: Component> {
    queue: VecDeque<AnimationPlayback<T, C>>,
    state: PlaybackState,
    applicator: AnimationApplicator<T, C>,
}

impl<T, C> Animatable<T, C>
where
    T: Send + Sync,
    C: Component,
{
    pub fn new(applicator: AnimationApplicator<T, C>) -> Self {
        Animatable {
            queue: VecDeque::default(),
            state: PlaybackState::Inactive,
            applicator,
        }
    }

    pub fn enqueue(&mut self, anim: Animation<T>) {
        self.queue.push_back(self.anim_to_playback(anim));
    }

    pub fn enqueue_and_play(&mut self, anim: Animation<T>) {
        self.enqueue(anim);
        self.play();
    }

    pub fn interrupt(&mut self, incoming: Animation<T>) {
        let pb = self.anim_to_playback(incoming);
        self.queue.push_front(pb);
        self.play();
    }

    fn anim_to_playback(&self, anim: Animation<T>) -> AnimationPlayback<T, C> {
        let duration = match anim.stages.iter().next() {
            Some(stage) => stage.duration,
            None => Duration::from_millis(0),
        };
        AnimationPlayback {
            timer: Timer::new(duration, TimerMode::Once),
            state: PlaybackState::Inactive,
            animation: anim,
            current_stage: 0,
            play_count: 0,
            applicator: self.applicator,
        }
    }

    // alternate: take every other sample from 2 curves

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn play(&mut self) {
        self.state = PlaybackState::Active;
    }

    pub fn pause(&mut self) {
        self.state = PlaybackState::Paused;
    }
}

impl Animatable<Srgba, RgbLed> {
    pub fn color() -> Self {
        Self::new(RgbLedColorApplicator)
    }
}

pub fn render_all_animatable<T: Send + Sync + 'static, C: Component>(
    mut anims: Query<(&mut Animatable<T, C>, &mut C)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (mut anim, mut component) in &mut anims {
        render_animatable(&mut anim, delta, &mut component);
    }
}

fn render_animatable<T: Send + Sync + 'static, C: Component>(
    anim: &mut Animatable<T, C>,
    delta: Duration,
    component: &mut C,
) {
    if anim.state == PlaybackState::Active {
        if let Some(playback) = anim.queue.front_mut() {
            playback.timer.tick(delta);
            if playback.timer.just_finished() {
                playback.current_stage += 1;

                // End of animation
                if playback.current_stage == playback.animation.stages.len() {
                    match playback.animation.playback {
                        PlaybackType::OneShot => {
                            anim.queue.pop_front();
                            return render_animatable(anim, delta, component);
                        }
                        PlaybackType::Forever => {
                            playback.current_stage = 0;
                        }
                        PlaybackType::Count(c) => {
                            playback.play_count += 1;
                            if playback.play_count == c as usize {
                                anim.queue.pop_front();
                                return render_animatable(anim, delta, component);
                            }
                        }
                    }
                }

                // Carry over any remaining time so that everything lines up
                let rem = playback.timer.remaining().clone();

                playback.timer.reset();
                // Update timer duration to current stage duration
                match playback.animation.stages.get(playback.current_stage) {
                    Some(stage) => playback.timer.set_duration(stage.duration),
                    None => {}
                }

                playback.timer.tick(rem);
            }

            // Render current value
            match playback.animation.stages.get(playback.current_stage) {
                Some(stage) => {
                    let phase =
                        playback.timer.elapsed_secs() / playback.timer.duration().as_secs_f32();
                    (playback.applicator)(phase, stage, component);
                }
                None => {}
            }
        }
    }
}
