use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash};

use crate::pinball::{Colored, Identity, Position};

use super::{resources::ExpPort, serial::exp_write, ExpansionBoard};

pub struct ExpansionLeds<K: Copy + Eq + Hash + Send + Sync + 'static>(pub Vec<LEDDefinition<K>>);

impl<K: Debug + Copy + Eq + Hash + Send + Sync + 'static> Plugin for ExpansionLeds<K> {
    fn build(&self, app: &mut App) {
        for definition in self.0.iter() {
            // spawn entities for LEDs
            app.world_mut().spawn((
                Identity { id: definition.id },
                Colored {
                    color: Hsla::hsl(0., 0., 0.),
                },
                Position {
                    row: definition.row,
                    col: definition.col,
                },
                FastLED {
                    expansion_address: definition.board.as_str(),
                    port: definition.port,
                    index: definition.index,
                },
            ));
        }

        app.add_systems(Update, led_change_listener);
    }
}

fn led_change_listener(
    query: Query<(&Colored, &FastLED), Changed<Colored>>,
    port: ResMut<ExpPort>,
) {
    for (indicator, led) in &query {
        let data = led_color_event(led, indicator.color);
        exp_write(data, &port);
    }
}

fn led_color_event(led: &FastLED, color: Hsla) -> String {
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
pub struct LEDDefinition<K: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: K,
    pub board: ExpansionBoard,
    pub port: u8,
    pub index: u8,
    pub row: u16,
    pub col: u16,
}

fn hsl_to_hex(color: Hsla) -> String {
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
        let hex = hsl_to_hex(Hsla::hsl(1., 1., 0.1));
        assert_eq!(hex, "320000".to_string());
    }

    #[test]
    fn it_makes_white() {
        let hex = hsl_to_hex(Hsla::hsl(1., 1., 1.));
        assert_eq!(hex, "ffffff".to_string());
    }
}
