use bevy::color::Srgba;
use bevy::prelude::*;

// -- General Components --

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct RgbIndicator {
    pub id: u16,
    pub enabled: bool,
    pub color: Srgba,
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct Switch {
    pub id: u16,
    pub closed: bool,
}

// -- Specific Components --

#[derive(Component)]
#[require(Switch)]
pub struct StartButton {}

#[derive(Component)]
#[require(Switch)]
pub struct AddCreditSwitch {}

// -- Events --

#[derive(Event)]
pub struct SwitchClosed(u16);

#[derive(Event)]
pub struct SwitchOpened(u16);

#[derive(Event)]
pub struct PinballButtonDown(u16);

#[derive(Event)]
pub struct PinballButtonUp(u16);
