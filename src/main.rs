use std::collections::HashMap;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use fast::Neutron;
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

    app.run();
}
