use bevy::{log::LogPlugin, prelude::*};

mod fast_pinball;
use fast_pinball::prelude::*;

fn main() {
    let neutron = Neutron::new("COM5")
        .add_exp_port("COM7")
        .add_expansion_board(
            ExpansionBoard::Neutron,
            vec![vec!["a", "b", "c", "d", "e", "f", "g", "h"]],
        );

    let mut app = App::new();

    app.add_plugins(MinimalPlugins);

    #[cfg(debug_assertions)]
    app.add_plugins(LogPlugin {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin=debug,bevy_pin::fast_pinball=trace"
            .to_string(),
        level: bevy::log::Level::TRACE,
        custom_layer: |_| None,
    });

    #[cfg(not(debug_assertions))]
    app.add_plugins(LogPlugin {
        filter: "warn".to_string(),
        level: bevy::log::Level::WARN,
        custom_layer: |_| None,
    });

    app.add_plugins(neutron).run();
}
