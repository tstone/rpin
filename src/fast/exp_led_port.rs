use bevy::{color::palettes::css::BLACK, prelude::*, time::common_conditions::on_timer};
use std::{fmt::Debug, time::Duration};

use crate::pinball::{PinballConfig, RgbLed};

use super::{resources::ExpPort, serial::exp_write, ExpansionBoard};

pub struct ExpansionLeds {
    pub leds: Vec<LedDefinition>,
    /// How frequently to send out updates to LEDs; given in Hz/FPS
    pub update_hz: f32,
}

impl Default for ExpansionLeds {
    fn default() -> Self {
        Self {
            leds: Default::default(),
            update_hz: 24.,
        }
    }
}

impl Plugin for ExpansionLeds {
    fn build(&self, app: &mut App) {
        for definition in self.leds.iter() {
            // spawn entities for LEDs
            let mut entity = app.world_mut().spawn((
                RgbLed { color: BLACK },
                FastExpansionDevice {
                    expansion_address: definition.board.as_str(),
                    port: definition.port,
                    index: definition.index,
                },
            ));

            // Name
            if !definition.name.is_empty() {
                entity.insert(Name::new(definition.name));
            }
        }

        let update_led_duration = Duration::from_secs_f32(1. / self.update_hz);
        app.add_systems(
            FixedLast,
            led_change_listener.run_if(on_timer(update_led_duration)),
        );
    }
}

fn led_change_listener(
    query: Query<(&RgbLed, &FastExpansionDevice), Changed<RgbLed>>,
    pinball_config: Res<PinballConfig>,
    port: ResMut<ExpPort>,
) {
    for (indicator, led) in &query {
        let color = if pinball_config.led_luminance_scale != 1.0 {
            // scale brightness if not 1.0
            let hsl = Hsla::from(indicator.color);
            Srgba::from(hsl.with_lightness(hsl.lightness * pinball_config.led_luminance_scale))
        } else {
            indicator.color
        };
        let data = led_color_event(led, color);
        exp_write(data, &port);
    }
}

fn led_color_event(led: &FastExpansionDevice, color: Srgba) -> String {
    format!(
        "RS@{}{}:{}{}",
        led.expansion_address,
        led.port,
        led.index,
        hsl_to_hex(color),
    )
}

/// FastLED -- Hardware attached to a Fast expansion board
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct FastExpansionDevice {
    pub expansion_address: &'static str,
    /// Port on expansion board
    pub port: u8,
    /// Index of LED on port
    pub index: u8,
}

/// Configuration for a single LED
/// See: https://fastpinball.com/programming/exp/#expansion-board-addresses
#[derive(Debug, Default, Clone)]
pub struct LedDefinition {
    pub board: ExpansionBoard,
    pub port: u8,
    pub index: u8,
    pub name: &'static str,
}

fn hsl_to_hex(rgb: Srgba) -> String {
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
        let hex = hsl_to_hex(Srgba::rgb(0., 0., 0.));
        assert_eq!(hex, "000000".to_string());
    }

    #[test]
    fn it_makes_white() {
        let hex = hsl_to_hex(Srgba::rgb(1., 1., 1.));
        assert_eq!(hex, "ffffff".to_string());
    }
}
