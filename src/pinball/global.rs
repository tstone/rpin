use bevy::color::Srgba;
use bevy::prelude::*;

// -- States --
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MachineState {
    #[default]
    Idle,
    Attract,
    CreditsDeposited,
    InGame,
}

// -- Lighting --

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct RgbIndicator<T: Clone + Default> {
    pub id: T,
    pub enabled: bool,
    pub color: Srgba,
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
pub struct Indicator<T: Clone> {
    pub id: T,
    pub enabled: bool,
}

// -- Switches --

#[derive(Event, Debug, Clone)]
pub struct SwitchInput<T: Clone> {
    pub id: T,
    pub state: SwitchState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Closed,
    Open,
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CabinetSwitches {
    AddCoin,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LowerThirdsSwitches {
    LeftOutlane,
    LeftInlane,
    RightOutlane,
    RightInlane,
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
