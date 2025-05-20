use bevy::prelude::*;

use crate::pinball::RgbLed;

use super::{
    animatable::{render_all_animatable, Animatable},
    render_led_gradient, render_led_sequence,
};

/// Plugin that renders LED sequences, a 1-dimensional listing of LEDs with multiple fill options
pub struct RgbLedPlugin;

impl Plugin for RgbLedPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_rgbled);
        app.add_systems(Startup, bootstrap_animatable);
        app.add_systems(Update, render_led_sequence);
        app.add_systems(Update, render_led_gradient);
        app.add_systems(Update, render_all_animatable::<Srgba, RgbLed>);
    }
}

fn bootstrap_animatable(query: Query<Entity, With<RgbLed>>, mut commands: Commands) {
    for entity in &query {
        commands.entity(entity).insert(Animatable::color());
    }
}

fn on_add_rgbled(
    trigger: Trigger<OnAdd, RgbLed>,
    query: Query<Entity, With<RgbLed>>,
    mut commands: Commands,
) {
    let led = query.get(trigger.entity()).unwrap();
    commands.entity(led).insert(Animatable::color());
}
