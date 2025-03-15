use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;

use crate::pinball::{SwitchInput, SwitchState};

pub struct SwitchEmulator<T: Copy + Eq + Hash + Send + Sync + 'static>(pub HashMap<KeyCode, T>);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Plugin for SwitchEmulator<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(SwitchEmulatorMapping(self.0.clone()));
        app.add_systems(Update, switch_emulator::<T>);
    }
}

fn switch_emulator<T: Copy + Eq + Hash + Send + Sync + 'static>(
    mapping: Res<SwitchEmulatorMapping<T>>,
    keys: ResMut<ButtonInput<KeyCode>>,
    mut switches: ResMut<ButtonInput<T>>,
    mut ev: EventWriter<SwitchInput<T>>,
) {
    for (key, switch_id) in mapping.0.iter() {
        if keys.just_pressed(*key) {
            switches.press(*switch_id);
            ev.send(SwitchInput {
                id: *switch_id,
                state: SwitchState::Closed,
            });
        } else if keys.just_released(*key) {
            switches.release(*switch_id);
            ev.send(SwitchInput {
                id: *switch_id,
                state: SwitchState::Open,
            });
        }
    }
}

#[derive(Resource)]
pub struct SwitchEmulatorMapping<T: Copy + Eq + Hash + Send + Sync + 'static>(HashMap<KeyCode, T>);
