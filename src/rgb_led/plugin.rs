use bevy::prelude::*;

use super::{render_led_gradient, render_led_sequence};

/// Plugin that renders LED sequences, a 1-dimensional listing of LEDs with multiple fill options
pub struct RgbLedPlugin;

impl Plugin for RgbLedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_led_sequence);
        app.add_systems(Update, render_led_gradient);
    }
}
