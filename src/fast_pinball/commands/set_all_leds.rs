use bevy::ecs::world::Command;
use colors_transform::Hsl;

use crate::fast_pinball::events::ExpPortData;

use super::color::HslExt;

pub struct SetAllLEDs {
    pub addr: String,
    pub color: Hsl,
}

impl Command for SetAllLEDs {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let msg = format!("RA@{}:{}", self.addr, self.color.to_hex());
        world.send_event(ExpPortData(msg));
    }
}
