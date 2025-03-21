use std::cmp;
use std::collections::HashMap;

use bevy::log::{Level, LogPlugin};
use bevy::{animation::*, color::palettes::css::*, prelude::*};
use fast::{ExpansionBoard, ExpansionLeds, LedDefinition, Neutron};
use pinball::dev_tools::{keyboard::SwitchEmulator, PinballDebugLogger};
use pinball::*;

mod examples;
mod fast;
mod pinball;

fn main() {
    let playfield_leds = (0..8)
        .map(|i| LedDefinition {
            board: ExpansionBoard::Neutron,
            port: 0,
            index: i,
            name: format!("LED{i}").leak(),
            // TODO: make it so that a list of components can be inserted?
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

    // TODO: need an LedUpdate schedule
    app.add_systems(Update, render_linear_space);
    app.add_systems(Startup, setup_linear_space);
    // app.add_systems(Startup, color_anim_setup);
    app.add_observer(setup_linear_anim);

    app.run();
}

fn color_anim_setup(
    mut commands: Commands,
    query: Query<(Entity, &Name), With<RgbLed>>,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
    animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let (entity, name) = query.iter().take(1).next().unwrap();

    let curve = AnimatableCurve::new(
        animated_field!(RgbLed::color),
        EasingCurve::new(BLACK, RED, EaseFunction::CubicInOut)
            .chain(EasingCurve::new(RED, BLUE_VIOLET, EaseFunction::Linear))
            .unwrap()
            .chain(EasingCurve::new(BLUE_VIOLET, BLUE, EaseFunction::Linear))
            .unwrap()
            .chain(EasingCurve::new(BLUE, BLACK, EaseFunction::CircularInOut))
            .unwrap()
            .reparametrize_linear(interval(0., 0.75).unwrap())
            .unwrap(),
    );

    let mut anim = SimpleAnimation::new(name, entity, curve, animation_graphs, animation_clips);
    anim.player
        .play(anim.animation_index)
        .set_repeat(RepeatAnimation::Count(5));

    commands.entity(entity).insert(anim.to_bundle());
}

fn setup_linear_space(
    mut commands: Commands,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
    animation_clips: ResMut<Assets<AnimationClip>>,
) {
    let name = Name::new("test_animation");
    let mut entity = commands.spawn((
        name.clone(),
        LinearLedSpace {
            position: 7.0,
            direction: 1,
            color: BLUE,
            names: vec![
                Name::new("LED0"),
                Name::new("LED1"),
                Name::new("LED2"),
                Name::new("LED3"),
                Name::new("LED4"),
                Name::new("LED5"),
                Name::new("LED6"),
                Name::new("LED7"),
            ],
            behavior: LinearLedBehavior::Single,
        },
    ));

    let pos_curve = AnimatableCurve::new(
        animated_field!(LinearLedSpace::position),
        EasingCurve::new(0., 7., EaseFunction::BounceOut)
            .reparametrize_linear(interval(0.0, 3.).unwrap())
            .unwrap()
            .ping_pong()
            .unwrap(),
    );

    let mut anim = SimpleAnimation::new(
        &name,
        entity.id(),
        pos_curve,
        animation_graphs,
        animation_clips,
    );
    anim.player.play(anim.animation_index).repeat();
    entity.insert(anim.to_bundle());
}

fn setup_linear_anim(
    trigger: Trigger<OnAdd, LinearLedSpace>,
    mut commands: Commands,
    animation_graphs: ResMut<Assets<AnimationGraph>>,
    animation_clips: ResMut<Assets<AnimationClip>>,
) {
    info!("added led linear space");
    let name = Name::new("test_animation");
    let curve = AnimatableCurve::new(
        animated_field!(LinearLedSpace::color),
        EasingCurve::new(BLACK, RED, EaseFunction::CubicInOut)
            .chain(EasingCurve::new(RED, BLUE_VIOLET, EaseFunction::Linear))
            .unwrap()
            .chain(EasingCurve::new(BLUE_VIOLET, BLUE, EaseFunction::Linear))
            .unwrap()
            .chain(EasingCurve::new(BLUE, BLACK, EaseFunction::CircularInOut))
            .unwrap()
            .reparametrize_linear(interval(0., 0.75).unwrap())
            .unwrap(),
    );

    let entity = trigger.entity();
    let mut anim = SimpleAnimation::new(&name, entity, curve, animation_graphs, animation_clips);
    anim.player.play(anim.animation_index).repeat();

    commands.entity(entity).insert(anim.to_bundle());
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LinearLedSpace {
    position: f32,
    direction: i8,
    color: Srgba,
    names: Vec<Name>,
    behavior: LinearLedBehavior,
}

#[derive(Clone, Debug, Default, Reflect)]
pub enum LinearLedBehavior {
    #[default]
    Single,
    Fill,
    FillGradient(Srgba),
    Tail(u8),
    TailGradient(u8, Srgba),
}

fn render_linear_space(
    spaces: Query<&LinearLedSpace, Changed<LinearLedSpace>>,
    mut leds: Query<(&Name, &mut RgbLed)>,
) {
    // for each space that has changed
    for space in &spaces {
        // go through the leds to find the ones that are in this space
        for (name, mut led) in &mut leds {
            let indexes = space
                .names
                .iter()
                .enumerate()
                .filter(|(_, n)| *name == **n)
                .map(|(index, _)| index);

            // then update the color of leds within space
            for i in indexes {
                match space.behavior {
                    LinearLedBehavior::Single => {
                        render_single(space.position, i, space.color, &mut led)
                    }
                    _ => todo!(),
                }
            }
        }
    }
}

fn render_single(active: f32, current: usize, color: Srgba, led: &mut RgbLed) {
    if active == current as f32 {
        led.color = color;
        return;
    }

    let lower = active.floor() as usize;
    let upper = active.ceil() as usize;

    if current == lower {
        let lum = upper as f32 - active;
        if lum > 0.5 {
            led.color = color.with_luminance(lum);
        } else {
            led.color = color.with_luminance(lum / 4.);
        }
    } else if current == upper {
        let lum = active - lower as f32;
        if lum > 0.5 {
            led.color = color.with_luminance(lum);
        } else {
            led.color = color.with_luminance(lum / 4.);
        }
    } else {
        led.color = BLACK;
    }
}

// ---

// Define "linear space" (ie. list of LEDs)
// Define "cartesian space" (ie. Position components on LEDs)
