use std::{collections::HashMap, time::Duration};

use bevy::{
    color::palettes::basic::AQUA,
    log::{Level, LogPlugin},
    prelude::*,
};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::frame_builder::{self, AnimationDuration};
use pinball::*;
use pinball::{
    dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger},
    frame_builder::AnimationSettings,
};

mod examples;
mod fast;
mod pinball;

fn main() {
    let playfield_leds = (0..7)
        .map(|i| LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: i,
            id: i,
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin::pinball=trace".to_string(),
        level: Level::TRACE,
        ..Default::default()
    }))
    .add_plugins(Neutron::new("COM5").add_exp_port("COM7"))
    .add_plugins(ExpansionLeds(playfield_leds))
    .add_plugins(PinballBase)
    .add_plugins(PaymentPlugin::default())
    .add_plugins(LedAnimationPlugin);

    #[cfg(debug_assertions)]
    app.add_plugins(PinballDebugLogger)
        .add_plugins(SwitchEmulator(HashMap::from([(
            KeyCode::Enter,
            CabinetButtons::StartButton,
        )])))
        .add_plugins(SwitchEmulator(HashMap::from([(
            KeyCode::Comma,
            CabinetSwitches::AddCoin,
        )])));

    app.add_systems(Startup, eased_on_off);
    app.run();
}

fn basic_led_anim(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(2).collect::<Vec<_>>();
    let frames = vec![
        vec![Color::hsl(180., 1.0, 0.35), Color::hsl(360., 1.0, 0.35)],
        vec![Color::hsl(360., 1.0, 0.35), Color::hsl(180., 1.0, 0.35)],
    ];
    commands.spawn(LedAnimation::new(entities).repeating(5, 5, frames));
}

fn basic_sequence(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(2).collect::<Vec<_>>();
    let frames = vec![
        vec![Color::hsl(180., 1.0, 0.35), Color::hsl(360., 1.0, 0.35)],
        vec![Color::hsl(360., 1.0, 0.35), Color::hsl(180., 1.0, 0.35)],
    ];
    commands.spawn(LedAnimation::new(entities).repeating(5, 5, frames));
}

fn linear_led_anim(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(5).collect::<Vec<_>>();
    let frames = frame_builder::sequential_linear(5, vec![Color::from(AQUA)]);
    commands.spawn(LedAnimation::new(entities).repeating(10, 5, frames));
}

fn eased_on_off(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();
    let frames = frame_builder::flash(
        1,
        Color::from(AQUA),
        AnimationSettings {
            duration: AnimationDuration::Duration {
                length: Duration::from_secs(1),
                fps: 24,
            },
            // duration: AnimationDuration::Frames(48),
            ease_in: Some(EaseFunction::Elastic(25.)),
            ease_out: Some(EaseFunction::BackOut),
        },
    );

    commands.spawn(LedAnimation::new(entities).infinite(24, frames));
}
