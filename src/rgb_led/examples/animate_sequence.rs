use std::cmp;
use std::collections::HashMap;

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
// This example assumes at least 8 RGB LEDs are connected to `port 1` of the Neutron board

// BEVY
// This example uses the built-in bevy animation feature: https://bevyengine.org/examples/animation/eased-motion/

fn main() {
    let playfield_leds = (0..8)
        .map(|i| LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: i,
            name: format!("LED{i}").leak(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(Neutron::new("COM5").add_exp_port("COM7"))
        .add_plugins(ExpansionLeds(playfield_leds))
        .add_plugins(PinballBase)
        .add_plugins(LedSequencePlugin);

    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    // Create a new LedSequence which names all LEDs in the sequence
    let name = Name::new("example");
    let mut entity = commands.spawn((
        name.clone(),
        LedSequence {
            position: 7.0,
            direction: 1,
            color: BLUE,
            names: (0..8)
                .map(|i| Name::new(format!("LED{i}")))
                .collect::<Vec<_>>(),
            behavior: LinearLedBehavior::Single,
        },
    ));

    // Animate the position of the color along the sequence
    let position_curve = AnimatableCurve::new(
        animated_field!(LedSequence::position),
        EasingCurve::new(0., 7., EaseFunction::Linear)
            .ping_pong()
            .unwrap(),
    );

    let target_id = AnimationTargetId::from_name(&name);
    let mut clip = AnimationClip::default();
    clip.add_curve_to_target(target_id, position_curve);

    let clip_handle = animation_clips.add(clip);
    let (graph, animation_index) = AnimationGraph::from_clip(clip_handle);
    let graph_handle = animation_graphs.add(graph);

    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();

    entity.insert((
        player,
        AnimationGraphHandle(graph_handle),
        AnimationTarget {
            id: target_id,
            player: entity.id(),
        },
    ));
}
