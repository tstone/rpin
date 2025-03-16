use bevy::ecs::component::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayfieldIndicators {
    LeftSpinner,
    LeftRamp,
    CenterSpinner,
    RightRamp,
    RightLane,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayfieldSwitches {
    MaserCanon,
    LeftSpinnerGate,
    LeftSpinnerEntrance,
    LeftSpinner,
    LeftRamp,
    MiniWallop,
    LeftTeslaTarget,
    BuildingEntrance,
    MiddleTeslaTarget,
    CenterSpinner,
    RightRamp,
    LoopEntrance,
    RightLane,
    Saucer,
    RightSpinner,
}
