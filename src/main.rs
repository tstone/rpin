use std::{collections::HashMap, time::Duration};

use bevy::{
    color::palettes::{
        basic::AQUA,
        css::{BLACK, BLUE, PURPLE, RED},
    },
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

    app.add_systems(Startup, fancy_sequence);
    app.run();
}

fn low_level_anim(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(2).collect::<Vec<_>>();
    let frames = vec![
        vec![Color::hsl(180., 1.0, 0.35), Color::hsl(360., 1.0, 0.35)],
        vec![Color::hsl(360., 1.0, 0.35), Color::hsl(180., 1.0, 0.35)],
    ];
    commands.spawn(LedAnimationPlayback::new(entities, 5, frames, None));
}

fn single_color(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(7).collect::<Vec<_>>();
    let anim = Solid {
        color: Color::from(AQUA),
    };
    commands.spawn(anim.to_repeated_playback(1, Duration::from_secs(3), entities, 1));
}

fn single_color_for_time(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(7).collect::<Vec<_>>();
    let anim = LedAnimationSequence::new()
        .play(
            Duration::from_secs(1),
            Solid {
                color: Color::from(PURPLE),
            },
        )
        .clear()
        .to_playback(entities, 2);

    commands.spawn(anim);
}

fn fancy_sequence(mut commands: Commands, query: Query<Entity, With<RgbLed>>) {
    let entities = query.iter().take(1).collect::<Vec<_>>();
    let color = Color::from(AQUA);
    let anim = LedAnimationSequence::new()
        .play(
            Duration::from_secs(2),
            Solid {
                color: Color::from(BLUE),
            },
        )
        .play(
            Duration::from_secs(2),
            Solid {
                color: Color::from(PURPLE),
            },
        )
        .play(
            Duration::from_secs(2),
            Solid {
                color: Color::from(RED),
            },
        )
        .clear()
        .to_playback(entities, 24);

    commands.spawn(anim);
}
