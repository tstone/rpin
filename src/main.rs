use bevy::{
    input::{keyboard::KeyboardInput, InputPlugin},
    log::LogPlugin,
    prelude::*,
};

mod fast_pinball;
mod pinball;

use colors_transform::Hsl;
use fast_pinball::{prelude::*, FastIoEvent};

#[repr(u16)]
enum CabinetSwitches {
    LeftFlipper,
    RightClipper,
    StartButton,
    AddCoinLeft,
    AddCoinRight,
}

enum PlayfieldSwitches {
    LeftOutlane,
    LeftInlane,
    RightOutlane,
    RightInlane,
    Trough1,
    Trough2,
    Trough3,
    Trough4,
    Trough5,
    Trough6,
    PlungerLane,
}

fn main() {
    App::new();

    // let neutron = Neutron::new("COM5")
    //     .add_io_board(&IoBoard::Fast3208 {
    //         switches: vec![Some("sw1"), Some("sw2")],
    //         coils: vec![],
    //     })
    //     .add_exp_port("COM7")
    //     .add_expansion_board(
    //         ExpansionBoard::Neutron,
    //         vec![vec!["a", "b", "c", "d", "e", "f", "g", "h"]],
    //     );

    // let mut app = App::new();

    // app.add_plugins(DefaultPlugins);

    // #[cfg(debug_assertions)]
    // app.add_plugins(LogPlugin {
    //     filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy_pin=debug,bevy_pin::fast_pinball=trace"
    //         .to_string(),
    //     level: bevy::log::Level::TRACE,
    //     custom_layer: |_| None,
    // });

    // #[cfg(not(debug_assertions))]
    // app.add_plugins(LogPlugin {
    //     filter: "warn".to_string(),
    //     level: bevy::log::Level::WARN,
    //     custom_layer: |_| None,
    // });

    // app.add_plugins(neutron);
    // app.add_systems(Update, keyboard_events);
    // app.add_systems(Update, fake_switch_input);
    // app.add_systems(Update, led_indicator);

    // app.run();
}

// fn led_indicator(mut ev_io: EventReader<FastIoEvent>, mut commands: Commands) {
//     for event in ev_io.read() {
//         println!("{:?}", event);
//         match event {
//             FastIoEvent::SwitchClosed { id } if id == "00" => {
//                 commands.set_led("a", Hsl::from(200., 100., 20.))
//             }
//             FastIoEvent::SwitchOpened { id } if id == "00" => {
//                 commands.set_led("a", Hsl::from(0., 0., 0.))
//             }
//             _ => {}
//         }
//     }
// }

// // TODO: make this some kind of nifty plugin that can be added
// fn fake_switch_input(keys: Res<ButtonInput<KeyCode>>, mut ev_io: EventWriter<FastIoEvent>) {
//     if keys.just_pressed(KeyCode::KeyA) {
//         ev_io.send(FastIoEvent::SwitchClosed {
//             id: "00".to_string(),
//         });
//     }
//     if keys.just_released(KeyCode::KeyA) {
//         ev_io.send(FastIoEvent::SwitchOpened {
//             id: "00".to_string(),
//         });
//     }
// }

// fn keyboard_events(mut evr_kbd: EventReader<KeyboardInput>) {
//     for ev in evr_kbd.read() {
//         println!("{ev:?}");
//     }
// }
