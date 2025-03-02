use bevy::prelude::*;

#[derive(Event)]
pub struct IoPortData(pub String);

#[derive(Event)]
pub struct ExpPortData(pub String);

#[derive(Event)]
pub struct FastSwitchOpened(pub String);
