use std::collections::HashMap;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use fast::{ExpansionBoard, ExpansionLeds, LEDDefinition, Neutron};
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
use pinball::frame_builder;
use pinball::*;

mod examples;
mod fast;
mod pinball;

fn main() {
    let playfield_leds = (0..7)
        .map(|i| LEDDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: i,
            id: i,
            row: 0,
            col: 0,
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

    app.add_systems(Startup, linear_led_anim);
    app.run();
}

fn basic_led_anim(mut commands: Commands, query: Query<Entity, With<Colored>>) {
    let entities = query.iter().take(2).collect::<Vec<_>>();
    let frames = vec![
        vec![Hsla::hsl(180., 1.0, 0.35), Hsla::hsl(360., 1.0, 0.35)],
        vec![Hsla::hsl(360., 1.0, 0.35), Hsla::hsl(180., 1.0, 0.35)],
    ];
    commands.spawn(LEDAnimation::new(
        5,
        Some(5),
        entities,
        frames,
        EndBehavior::Previous,
    ));
}

fn linear_led_anim(mut commands: Commands, query: Query<Entity, With<Colored>>) {
    let entities = query.iter().take(5).collect::<Vec<_>>();
    let frames = frame_builder::linear::generate_single(5, Hsla::hsl(150., 1.0, 0.25));
    commands.spawn(LEDAnimation::new(
        10,
        Some(5),
        entities,
        frames,
        EndBehavior::Previous,
    ));
}
