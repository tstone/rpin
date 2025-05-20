use std::collections::HashMap;
use std::time::Duration;

use bevy::animation::*;
use bevy::color::palettes::tailwind::{TEAL_200, TEAL_400};
use bevy::log::{Level, LogPlugin};
use bevy::{color::palettes::css::*, prelude::*};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::keyboard::SwitchEmulator;
use pinball::*;
use rgb_led::{
    Animatable, Animation, AnimationStage, Curve, LedGradient, LedGradientFill, LedSequence,
    LedSequenceFill, RgbLedPlugin,
};

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
    .add_plugins(PinballBase {
        led_brightness_scale: 0.66,
    })
    .add_plugins(Neutron {
        io_port_path: "COM5",
        exp_port_path: "COM7",
        ..Default::default()
    })
    .add_plugins(ExpansionLeds {
        leds: playfield_leds,
        ..Default::default()
    })
    .add_plugins(RgbLedPlugin);

    #[cfg(debug_assertions)]
    app.add_plugins(SwitchEmulator(HashMap::from([(
        KeyCode::Enter,
        CabinetButtons::StartButton,
    )])))
    .add_plugins(SwitchEmulator(HashMap::from([(
        KeyCode::Comma,
        CabinetSwitches::AddCoin,
    )])));

    app.add_systems(PostStartup, setup_one);
    app.run();
}

fn setup_one(mut query: Query<&mut Animatable<Srgba, RgbLed>>) {
    for (i, mut animatable) in query.iter_mut().enumerate() {
        if i == 0 {
            let anim = Curve::Sinusoid.animate(BLACK, ORANGE, Duration::from_secs(3));
            animatable.enqueue_and_play(anim.as_continuous());
        } else if i == 2 {
            let anim = Curve::Steps(3).animate(BLACK, ORANGE_RED, Duration::from_secs(6));
            animatable.enqueue_and_play(anim.as_continuous());
        } else if i == 4 {
            let anim = Animation::keyframes(vec![
                (BLACK, Duration::from_millis(500)),
                (ORANGE, Duration::from_millis(500)),
                (BLUE_VIOLET, Duration::from_millis(500)),
                (BLACK, Duration::from_millis(500)),
            ]);
            animatable.enqueue_and_play(anim.as_continuous());
        } else if i == 6 {
            let anim = Curve::Sinusoid
                .stage(BLACK, ORANGE, Duration::from_secs(1))
                .repeat(5)
                .chain(
                    Curve::Sinusoid
                        .stage(BLACK, ORANGE, Duration::from_millis(750))
                        .repeat(2),
                )
                .chain(
                    Curve::Sinusoid
                        .stage(BLACK, ORANGE, Duration::from_millis(500))
                        .repeat(4),
                )
                .chain(
                    Curve::Sinusoid
                        .stage(BLACK, ORANGE, Duration::from_millis(250))
                        .repeat(6),
                )
                .chain(
                    Curve::Sinusoid
                        .stage(BLACK, ORANGE, Duration::from_millis(125))
                        .repeat(8),
                );
            animatable.enqueue_and_play(anim.as_continuous());
        }
    }

    // let mut animatable = query.iter_mut().take(1).next().unwrap();
    // let anim = Animation::tween(vec![
    //     (BLACK, Duration::from_millis(1500), Curve::Linear),
    //     (RED, Duration::from_millis(1500), Curve::Sinusoid),
    //     (BLUE, Duration::from_millis(1500), Curve::Linear),
    //     (BLACK, Duration::from_millis(1500), Curve::Linear),
    // ]);
    // info!("stages: {:?}", anim.stages);

    // let anim = Curve::Sinusoid
    //     .stage(BLACK, RED, Duration::from_secs(1))
    //     .repeat(5)
    //     .chain(
    //         Curve::Sinusoid
    //             .stage(BLACK, RED, Duration::from_millis(750))
    //             .repeat(2),
    //     )
    //     .chain(
    //         Curve::Sinusoid
    //             .stage(BLACK, RED, Duration::from_millis(500))
    //             .repeat(4),
    //     )
    //     .chain(
    //         Curve::Sinusoid
    //             .stage(BLACK, RED, Duration::from_millis(250))
    //             .repeat(6),
    //     )
    //     .chain(
    //         Curve::Sinusoid
    //             .stage(BLACK, RED, Duration::from_millis(125))
    //             .repeat(8),
    //     );
}
