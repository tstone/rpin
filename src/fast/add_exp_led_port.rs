use bevy::prelude::*;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::pinball::RgbIndicator;

use super::{
    events::ExpPortData,
    resources::{HardwareLed, HardwareLedMapping},
    ExpansionBoard,
};

pub struct ExpansionLedPort<K: Copy + Eq + Hash + Send + Sync + 'static> {
    board: ExpansionBoard,
    port: u8,
    leds: Vec<LedDefinition<K>>,
}

impl<K: Component + Debug + Copy + Eq + Hash + Send + Sync + 'static> Plugin
    for ExpansionLedPort<K>
{
    fn build(&self, app: &mut App) {
        let mut mapping: HashMap<K, Vec<HardwareLed>> = HashMap::new();
        let addr = self.board.as_str();

        for (index, def) in self.leds.iter().enumerate() {
            let led = HardwareLed {
                expansion_address: addr,
                port: self.port,
                index: index as u8,
            };
            match mapping.get_mut(&def.key) {
                Some(vec) => vec.push(led),
                None => {
                    mapping.insert(def.key, vec![led]);
                }
            }

            // spawn indicator entities
            app.world_mut().spawn((
                def.key,
                RgbIndicator {
                    color: Hsla::hsl(0., 0., 0.),
                    id: def.key,
                    row: def.row,
                    col: def.col,
                },
            ));
        }

        app.insert_resource(HardwareLedMapping(mapping));
        app.add_systems(Update, led_change_listener::<K>);
    }
}

fn led_change_listener<K: Component + Debug + Copy + Eq + Hash + Send + Sync + 'static>(
    query: Query<&RgbIndicator<K>, Changed<RgbIndicator<K>>>,
    mapping: Res<HardwareLedMapping<K>>,
    mut ev: EventWriter<ExpPortData>,
) {
    for indicator in &query {
        match mapping.0.get(&indicator.id) {
            Some(leds) => {
                for led in leds {
                    let msg = format!(
                        "RS@{}{}:{}{}",
                        led.expansion_address,
                        led.port,
                        led.index,
                        hsl_to_hex(indicator.color),
                    );
                    ev.send(ExpPortData(msg));
                }
            }
            None => error!("Indicator {:?} is not mapped to hardware", indicator.id),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct LedDefinition<K: Copy + Eq + Hash + Send + Sync + 'static> {
    key: K,
    row: u16,
    col: u16,
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
