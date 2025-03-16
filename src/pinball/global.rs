use std::hash::Hash;

use bevy::color::Hsla;
use bevy::prelude::*;

// -- States --
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineState {
    #[default]
    Waiting,
    InGame,
}

// -- Lighting --

#[derive(Component, Debug, Clone, PartialEq, Default)]
pub struct RgbIndicator<T: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: T,
    pub color: Hsla,
    pub row: u16,
    pub col: u16,
}

#[derive(Component, Debug, Clone, PartialEq, Default)]
pub struct Indicator<T: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: T,
    pub enabled: bool,
    pub row: u16,
    pub col: u16,
}

// -- Switches --

#[derive(Event, Debug, Clone)]
pub struct SwitchInput<T: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: T,
    pub state: SwitchState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Closed,
    Open,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum CabinetButtons {
    LeftFlipper,
    LeftMagnasave,
    RightFlipper,
    RightMagnasave,
    ActionButton,
    #[default]
    StartButton,
    ContinueButton,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CabinetSwitches {
    #[default]
    AddCoin,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum LowerThirdsSwitches {
    LeftOutlane,
    LeftInlane,
    RightOutlane,
    RightInlane,
    #[default]
    Trough1,
    Trough2,
    Trough3,
    Trough4,
    Trough5,
    Trough6,
    Trough7,
    Trough8,
    PlungerLane,
    LeftSlingUpper,
    LeftSlingLower,
    RightSlingUpper,
    RightSlingLower,
}
