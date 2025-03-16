use bevy::prelude::*;

use super::{CabinetButtons, CabinetSwitches, Inputs, LowerThirdsSwitches, MachineState};

/// A plugin to setup all the base events and resources
pub struct PinballBase;

impl Plugin for PinballBase {
    fn build(&self, app: &mut App) {
        app.add_plugins(Inputs(CabinetButtons::default()));
        app.add_plugins(Inputs(CabinetSwitches::default()));
        app.add_plugins(Inputs(LowerThirdsSwitches::default()));

        app.init_state::<MachineState>();
    }
}
