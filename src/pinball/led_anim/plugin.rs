use bevy::prelude::*;

use crate::pinball::RgbLed;

use super::anim::LedAnimation;

/// LedAnimationPlugin -- A plugin to run LED
pub struct LedAnimationPlugin;

impl Plugin for LedAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_anim);
        app.add_observer(on_remove_anim);
        app.add_systems(Update, anim_frame_handler);
    }
}

fn on_add_anim(
    trigger: Trigger<OnAdd, LedAnimation>,
    anim_query: Query<&LedAnimation>,
    mut leds: Query<(Entity, &mut RgbLed)>,
) {
    match anim_query.get(trigger.entity()) {
        Ok(anim) => {
            // apply first frame to start animation instantly
            apply_current_frame(&anim, &mut leds);
        }
        _ => {}
    }
}

fn on_remove_anim(
    trigger: Trigger<OnRemove, LedAnimation>,
    anim_query: Query<&LedAnimation>,
    mut commands: Commands,
) {
    let anim = anim_query.get(trigger.entity()).unwrap();
    match &anim.next {
        None => {}
        Some(next) => {
            // start the next animation in the sequence
            commands.spawn(*next.clone());
        }
    }
}

fn anim_frame_handler(
    time: ResMut<Time>,
    mut anim_query: Query<(Entity, &mut LedAnimation)>,
    mut led_query: Query<(Entity, &mut RgbLed)>,
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

fn apply_current_frame(anim: &LedAnimation, led_query: &mut Query<(Entity, &mut RgbLed)>) {
    let frame = &anim.frames[anim.current_frame];
    apply_colors(frame, anim, led_query);
}

fn apply_colors(
    colors: &Vec<Color>,
    anim: &LedAnimation,
    led_query: &mut Query<(Entity, &mut RgbLed)>,
) {
    for (led_entity, mut colored) in led_query {
        match anim.led_indexes.get(&led_entity) {
            Some(index) => colored.color = colors[*index].clone(),
            _ => (),
        }
    }
}
