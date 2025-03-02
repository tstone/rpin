use bevy::{
    ecs::system::{Commands, Res},
    utils::hashbrown::HashSet,
};
use colors_transform::Hsl;

use super::resources::Indicators;
use super::FastCommandsExt;

/// Set all LEDs to off to clear any prior state
pub fn reset_leds(indicators: Res<Indicators>, mut commands: Commands) {
    let mut expansion_boards_with_leds = HashSet::<&str>::new();
    for led in indicators.leds.values() {
        expansion_boards_with_leds.insert(&led.expansion_address);
    }
    for addr in expansion_boards_with_leds.iter() {
        // TODO: make LED brightness configurable
        commands.set_all_leds(*addr, Hsl::from(200., 100., 20.));
    }
}
