use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

use crate::pinball::{Identity, PlayfieldPosition, RgbLed};

use super::{resources::ExpPort, serial::exp_write, ExpansionBoard};

pub struct ExpansionLeds<K: Copy + Eq + Hash + Send + Sync + 'static>(pub Vec<LedDefinition<K>>);

impl<K: Debug + Copy + Eq + Hash + Send + Sync + 'static> Plugin for ExpansionLeds<K> {
    fn build(&self, app: &mut App) {
        for definition in self.0.iter() {
            // spawn entities for LEDs
            let mut entity = app.world_mut().spawn((
                Identity { id: definition.id },
                RgbLed {
                    color: Color::hsl(0., 0., 0.),
                },
                FastLED {
                    expansion_address: definition.board.as_str(),
                    port: definition.port,
                    index: definition.index,
                },
            ));

            if let Some(pos) = &definition.playfield_position {
                entity.insert(pos.clone());
            }
        }

        app.add_systems(Update, led_change_listener);
    }
}

fn led_change_listener(query: Query<(&RgbLed, &FastLED), Changed<RgbLed>>, port: ResMut<ExpPort>) {
    for (indicator, led) in &query {
        let data = led_color_event(led, indicator.color);
        exp_write(data, &port);
    }
}

fn led_color_event(led: &FastLED, color: Color) -> String {
    format!(
        "RS@{}{}:{}{}",
        led.expansion_address,
        led.port,
        led.index,
        hsl_to_hex(color),
    )
}

/// FastLED -- Component which adds FAST EXP address information
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct FastLED {
    pub expansion_address: &'static str,
    /// Port on expansion board
    pub port: u8,
    /// Index of LED on port
    pub index: u8,
}

/// Configuration for a single LED
/// See: https://fastpinball.com/programming/exp/#expansion-board-addresses
#[derive(Debug, Default, Clone)]
pub struct LedDefinition<K: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: K,
    pub board: ExpansionBoard,
    pub port: u8,
    pub index: u8,
    pub playfield_position: Option<PlayfieldPosition>,
}

fn hsl_to_hex(color: Color) -> String {
    let rgb = Srgba::from(color);
    format!(
        "{:0>2x}{:0>2x}{:0>2x}",
        (rgb.red * 255.) as u16,
        (rgb.green * 255.) as u16,
        (rgb.blue * 255.) as u16
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_single_digits() {
        let hex = hsl_to_hex(Color::hsl(1., 1., 0.1));
        assert_eq!(hex, "320000".to_string());
    }

    #[test]
    fn it_makes_white() {
        let hex = hsl_to_hex(Color::hsl(1., 1., 1.));
        assert_eq!(hex, "ffffff".to_string());
    }
}
