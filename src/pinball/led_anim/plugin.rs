use bevy::prelude::*;

use crate::pinball::{Colored, EndBehavior};

use super::anim::LEDAnimation;

/// LedAnimationPlugin -- A plugin to run LED
pub struct LedAnimationPlugin;

impl Plugin for LedAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_anim);
        app.add_observer(on_remove_anim);
        app.add_systems(Update, anim_frame_handler);
    }
}

/// Runs the first time an LEDAnimation is added to the world
fn on_add_anim(
    trigger: Trigger<OnAdd, LEDAnimation>,
    mut anim_query: Query<&mut LEDAnimation>,
    mut leds: Query<(Entity, &mut Colored)>,
) {
    match anim_query.get_mut(trigger.entity()) {
        Ok(mut anim) => {
            // Record previous colors if feature is enabled
            if anim.end_behavior == EndBehavior::Previous {
                anim.previous_colors = anim
                    .led_indexes
                    .keys()
                    .map(|led| match leds.get(*led) {
                        Ok((_, colored)) => colored.color.clone(),
                        _ => Hsla::hsl(0., 0., 0.),
                    })
                    .collect::<Vec<_>>();
            }

            // apply first frame to start animation instantly
            apply_current_frame(&anim, &mut leds);
        }
        _ => {}
    }
}

/// Runs when an animation is removed/stopped and applies previous behavior
fn on_remove_anim(
    trigger: Trigger<OnRemove, LEDAnimation>,
    anim_query: Query<&LEDAnimation>,
    mut leds: Query<(Entity, &mut Colored)>,
) {
    let anim = anim_query.get(trigger.entity()).unwrap();
    match anim.end_behavior {
        EndBehavior::Black => {
            let blacks = anim
                .led_indexes
                .iter()
                .map(|_| Hsla::hsl(0., 0., 0.))
                .collect::<Vec<_>>();
            apply_colors(&blacks, anim, &mut leds);
        }
        EndBehavior::Previous => apply_colors(&anim.previous_colors, anim, &mut leds),
        EndBehavior::Nothing => {}
    }
}

fn anim_frame_handler(
    time: ResMut<Time>,
    mut anim_query: Query<(Entity, &mut LEDAnimation)>,
    mut led_query: Query<(Entity, &mut Colored)>,
    mut commands: Commands,
) {
    for (anim_entity, mut anim) in &mut anim_query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            // advance frame
            anim.current_frame += 1;

            // handle frame repeat or reset
            if anim.current_frame == anim.frames.len() {
                match anim.repeat {
                    None => anim.current_frame = 0,
                    Some(0) => {
                        // If repeat is enabled but there are none left
                        // then drop the animation
                        commands.entity(anim_entity).despawn();
                        continue;
                    }
                    Some(r) => {
                        anim.repeat = Some(r - 1);
                        anim.current_frame = 0;
                    }
                }
            }

            // apply next frame
            apply_current_frame(&anim, &mut led_query);
        }
    }
}

fn apply_current_frame(anim: &LEDAnimation, led_query: &mut Query<(Entity, &mut Colored)>) {
    let frame = &anim.frames[anim.current_frame];
    apply_colors(frame, anim, led_query);
}

fn apply_colors(
    colors: &Vec<Hsla>,
    anim: &LEDAnimation,
    led_query: &mut Query<(Entity, &mut Colored)>,
) {
    for (led_entity, mut colored) in led_query {
        match anim.led_indexes.get(&led_entity) {
            Some(index) => colored.color = colors[*index].clone(),
            _ => (),
        }
    }
}
