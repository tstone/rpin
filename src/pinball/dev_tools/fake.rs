// A plugin that configures fake hardware
use bevy::prelude::*;

use crate::pinball::{CabinetButtons, Indicator, RgbIndicator};

pub fn spawn_fake_cabinet_hardware(mut commands: Commands) {
    commands.spawn((
        CabinetButtons::StartButton,
        Indicator {
            id: CabinetButtons::StartButton,
            ..Default::default()
        },
    ));
    commands.spawn((
        CabinetButtons::LeftFlipper,
        Indicator {
            id: CabinetButtons::LeftFlipper,
            ..Default::default()
        },
    ));
    commands.spawn((
        CabinetButtons::RightFlipper,
        Indicator {
            id: CabinetButtons::RightFlipper,
            ..Default::default()
        },
    ));
    commands.spawn((
        CabinetButtons::ActionButton,
        RgbIndicator {
            id: CabinetButtons::ActionButton,
            ..Default::default()
        },
    ));
}

// TODO: spawn_fake_lower_thirds_hardware(mut commands: Commands)
