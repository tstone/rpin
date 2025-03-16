use std::hash::Hash;

use bevy::prelude::*;

use super::SwitchInput;

/// A plugin to setup an input type (switch, button, etc.)
pub struct Inputs<T: Copy + Eq + Hash + Send + Sync + 'static>(pub T);

impl<T: Copy + Eq + Hash + Send + Sync + 'static> Plugin for Inputs<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonInput<T>>();
        app.add_event::<SwitchInput<T>>();
    }
}
