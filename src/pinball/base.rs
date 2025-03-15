use bevy::prelude::*;

use super::{CabinetButtons, CabinetSwitches, LowerThirdsSwitches, MachineState, SwitchInput};

/// A plugin to setup all the base events and resources
pub struct PinballBase;

impl Plugin for PinballBase {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonInput<CabinetButtons>>();
        app.init_resource::<ButtonInput<CabinetSwitches>>();

        app.add_event::<SwitchInput<CabinetSwitches>>();
        app.add_event::<SwitchInput<CabinetButtons>>();
        app.add_event::<SwitchInput<LowerThirdsSwitches>>();

        app.init_state::<MachineState>();
    }
}
