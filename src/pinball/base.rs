use bevy::prelude::*;

use super::{CabinetButtons, CabinetSwitches, Inputs, LowerThirdsSwitches, MachineState};

/// A plugin to setup all the base events and resources
pub struct PinballBase {
    /// Scales the LED luminance by the given factor e.g. 0.5 means half as bright
    pub led_brightness_scale: f32,
}

impl Default for PinballBase {
    fn default() -> Self {
        Self {
            led_brightness_scale: 1.0,
        }
    }
}

impl Plugin for PinballBase {
    fn build(&self, app: &mut App) {
        app.add_plugins(Inputs(CabinetButtons::default()));
        app.add_plugins(Inputs(CabinetSwitches::default()));
        app.add_plugins(Inputs(LowerThirdsSwitches::default()));

        app.init_state::<MachineState>();

        app.insert_resource(PinballConfig {
            led_luminance_scale: self.led_brightness_scale,
        });
    }
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct PinballConfig {
    pub led_luminance_scale: f32,
}
