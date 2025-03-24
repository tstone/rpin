use std::cmp;
use std::collections::HashMap;

use bevy::color::palettes::tailwind::{PINK_950, PURPLE_950, SKY_800};
use bevy::log::{Level, LogPlugin};
use bevy::{animation::*, color::palettes::css::*, prelude::*};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
use pinball::*;

mod examples;
mod fast;
mod pinball;
mod rgb_animations;

// HARDWARE SETUP:
// This example assumes at least 1 RGB LED is connected to `port 1` of the Neutron board

// BEVY
// This example uses the built-in bevy animation feature: https://bevyengine.org/examples/animation/eased-motion/

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(Neutron::new("COM5").add_exp_port("COM7"))
        .add_plugins(ExpansionLeds(vec![LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: i,
            name: "LED0",
            ..Default::default()
        }]))
        .add_plugins(PinballBase);

    app.add_systems(Startup, basic_anim_setup);
    app.run();
}

fn basic_anim_setup(
    mut commands: Commands,
    query: Query<(Entity, &Name), With<RgbLed>>,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
    animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let (entity, name) = query.iter().take(1).next().unwrap();

    // The "curve" represents the values and duration
    // Black -> Red -> Blue -> Black
    let curve = EasingCurve::new(BLACK, RED, EaseFunction::CubicInOut)
        .chain(EasingCurve::new(BLUE, BLACK, EaseFunction::CircularInOut))
        .unwrap()
        // `internval` represents the time, in this case 2.0 seconds
        .reparametrize_linear(interval(0., 2.).unwrap())
        .unwrap();

    // The curve is combined with which field will be animated
    // to create the "animatable curve"
    let anim_curve = AnimatableCurve::new(animated_field!(RgbLed::color), curve);

    // Animatable curves can be added to a "clip", which is a set of curves
    let target_id = AnimationTargetId::from_name(name);
    let mut clip = AnimationClip::default();
    clip.add_curve_to_target(target_id, anim_curve);

    // TODO: how to get an event when an animation is done
    // TODO: how to remove animation components when it's done

    let clip_handle = animation_clips.add(clip);
    let (graph, node_index) = AnimationGraph::from_clip(clip_handle);
    let graph = animation_graphs.add(graph);

    // The animation doesn't have to be started right away, but here it is
    let player = AnimationPlayer::default();
    player.play(anim.animation_index).repeat();

    // Once the components are added to the entity they will begin to animate
    commands.entity(entity).insert((
        player,
        graph_handle,
        AnimationTarget {
            id: target_id,
            player: entity,
        },
    ));
}
