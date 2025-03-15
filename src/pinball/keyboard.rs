use std::{collections::HashMap, hash::Hash};

use bevy::prelude::*;

fn switch_emulator<T: Copy + Eq + Hash + Send + Sync + 'static>(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut switches: ResMut<ButtonInput<T>>,
    mapping: Res<EmulatedSwitchMapping<T>>,
) {
    for (key, switch_id) in mapping.0.iter() {
        if keys.just_pressed(*key) {
            switches.press(*switch_id);
        } else if keys.just_released(*key) {
            switches.release(*switch_id);
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct EmulatedSwitchMapping<T: Copy + Eq + Hash + Send + Sync + 'static>(HashMap<KeyCode, T>);
