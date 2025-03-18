use std::{collections::HashMap, time::Duration};

use bevy::{
    color::palettes::css::*,
    log::{Level, LogPlugin},
    prelude::*,
};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::animations::*;
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
use pinball::*;

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

    app.add_systems(Startup, flashing);
    app.run();
}

fn flashing(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();
    let color = Color::from(BLUE_VIOLET);

    let anim = Flash {
        color,
        hz: 100.,
        ..Default::default()
    };

    commands.spawn(anim.to_one_shot(Duration::from_secs(1), entities, 24));
}
