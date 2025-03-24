use std::cmp;
use std::collections::HashMap;

use bevy::color::palettes::tailwind::{PINK_950, PURPLE_950, SKY_800};
use bevy::log::{Level, LogPlugin};
use bevy::{color::palettes::css::*, prelude::*};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
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
    .add_plugins(Neutron {
        io_port_path: "COM5",
        exp_port_path: "COM7",
        ..Default::default()
    })
    .add_plugins(ExpansionLeds {
        leds: playfield_leds,
        ..Default::default()
    })
    .add_plugins(PinballBase)
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

fn setup_seq(query: Query<&Name, With<RgbLed>>, mut commands: Commands) {
    let names = query.iter().take(8).map(|n| n.clone()).collect::<Vec<_>>();
    let seq = LedSequence {
        position: 5.,
        color: RED,
        names,
        behavior: LedSequenceFill::Tail(2),
    };
    commands.spawn(seq);
}
