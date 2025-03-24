use std::collections::HashMap;

use bevy::animation::*;
use bevy::log::{Level, LogPlugin};
use bevy::{color::palettes::css::*, prelude::*};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::keyboard::SwitchEmulator;
use pinball::*;
use rgb_led::{LedSequence, LedSequenceFill, LedSequencePlugin};

mod examples;
mod fast;
mod pinball;
mod rgb_led;

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
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin::pinball=trace".to_string(),
        level: Level::TRACE,
        ..Default::default()
    }))
    .add_plugins(PinballBase {
        led_brightness_scale: 0.66,
    })
    .add_plugins(Neutron {
        io_port_path: "COM5",
        exp_port_path: "COM7",
        ..Default::default()
    })
    .add_plugins(ExpansionLeds {
        leds: playfield_leds,
        ..Default::default()
    })
    .add_plugins(LedSequencePlugin);

    #[cfg(debug_assertions)]
    app.add_plugins(SwitchEmulator(HashMap::from([(
        KeyCode::Enter,
        CabinetButtons::StartButton,
    )])))
    .add_plugins(SwitchEmulator(HashMap::from([(
        KeyCode::Comma,
        CabinetSwitches::AddCoin,
    )])));

    app.add_systems(Startup, setup_seq);
    // app.add_systems(Startup, setup_one);
    app.run();
}

fn setup_one(mut query: Query<&mut RgbLed>) {
    let mut led = query.iter_mut().take(1).next().unwrap();
    led.color = RED;
}

fn setup_seq(
    query: Query<&Name, With<RgbLed>>,
    mut commands: Commands,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let led_names = query.iter().take(8).map(|n| n.clone()).collect::<Vec<_>>();
    let seq = LedSequence {
        position: 4.,
        color: YELLOW,
        names: led_names,
        behavior: LedSequenceFill::Split(1),
    };

    let name = Name::new("led_seq_example");
    let target_id = AnimationTargetId::from_name(&name);
    let mut entity_commands = commands.spawn((name, seq));

    let duration = 2.0;

    let position_curve = AnimatableCurve::new(
        animated_field!(LedSequence::position),
        EasingCurve::new(0., 7., EaseFunction::QuadraticOut)
            .ping_pong()
            .unwrap()
            .reparametrize_linear(interval(0., duration).unwrap())
            .unwrap(),
    );

    let color_curve = AnimatableCurve::new(
        animated_field!(LedSequence::color),
        EasingCurve::new(YELLOW, RED, EaseFunction::CircularOut)
            .reparametrize_linear(interval(0., duration).unwrap())
            .unwrap(),
    );

    let mut clip = AnimationClip::default();
    clip.add_curve_to_target(target_id, position_curve);
    // clip.add_curve_to_target(target_id, color_curve);

    let clip_handle = animation_clips.add(clip);
    let (graph, animation_index) = AnimationGraph::from_clip(clip_handle);
    let graph_handle = animation_graphs.add(graph);

    let mut player = AnimationPlayer::default();
    player.play(animation_index).repeat();

    entity_commands.insert((
        player,
        AnimationGraphHandle(graph_handle),
        AnimationTarget {
            id: target_id,
            player: entity_commands.id(),
        },
    ));
}
