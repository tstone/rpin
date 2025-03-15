use std::collections::HashMap;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use pinball::{
    dev_tools::{fake::spawn_fake_cabinet_hardware, keyboard::SwitchEmulator, PinballDebugLogger},
    CabinetButtons, CabinetSwitches, PaymentPlugin, PinballBase,
};

mod fast_pinball;
mod pinball;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin::pinball=debug".to_string(),
        level: Level::DEBUG,
        ..Default::default()
    }))
    .add_systems(Startup, spawn_fake_cabinet_hardware)
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
