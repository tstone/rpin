use std::collections::HashMap;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use examples::godzilla::playfield::PlayfieldIndicators;
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
use pinball::*;

mod examples;
mod fast;
mod pinball;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin::pinball=debug".to_string(),
        level: Level::DEBUG,
        ..Default::default()
    }))
    .add_plugins(Neutron::new("COM5").add_exp_port("COM7"))
    .add_plugins(ExpansionLeds(vec![
        LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: 0,
            id: PlayfieldIndicators::LeftSpinner,
            row: 2,
            col: 0,
        },
        LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: 1,
            id: PlayfieldIndicators::LeftRamp,
            row: 4,
            col: 0,
        },
    ]))
    .add_plugins(PinballBase)
    .add_plugins(PaymentPlugin::default());

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

    app.add_systems(Update, test_startup);
    app.run();
}

fn test_startup(
    mut query: Query<&mut RgbIndicator<PlayfieldIndicators>>,
    mut ev: EventReader<SwitchInput<CabinetButtons>>,
) {
    for e in ev.read() {
        if e.id == CabinetButtons::StartButton {
            let color = if e.state == SwitchState::Closed {
                Hsla::hsl(0.5, 0.5, 0.5)
            } else {
                Hsla::hsl(0., 0., 0.)
            };

            for mut led in &mut query {
                if led.id == PlayfieldIndicators::LeftSpinner {
                    led.color = color;
                }
            }
        }
    }
}
