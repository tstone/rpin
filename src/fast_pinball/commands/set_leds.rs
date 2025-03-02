use bevy::ecs::world::Command;
use bevy::log::{error, warn};
use colors_transform::Hsl;

use crate::fast_pinball::{events::ExpPortData, resources::Indicators};

use super::color::HslExt;

pub struct SetLED {
    pub name: String,
    pub color: Hsl,
}

impl Command for SetLED {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        match world.get_resource::<Indicators>() {
            Some(indicators) => match indicators.leds.get(self.name.as_str()) {
                Some(led) => {
                    let msg = format!(
                        "RA@{}{}:{}",
                        led.expansion_address,
                        led.port,
                        self.color.to_hex()
                    );
                    world.send_event(ExpPortData(msg));
                }
                None => warn!("Could not find LED named {}", self.name),
            },
            None => error!("No LED indicators configured"),
        }
    }
}
