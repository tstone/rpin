use std::hash::Hash;

use bevy::prelude::*;

/// Identity - Something that is identifiable, e.g. an LED or servo
#[derive(Component, Debug, Default, Copy, Clone, PartialEq)]
pub struct Identity<T: Copy + Eq + Hash + Send + Sync + 'static> {
    pub id: T,
}

/// Position -- Where smoething is located on the playfield, typically used with indicators
#[derive(Component, Debug, Clone, PartialEq, Default)]
pub struct PlayfieldPosition {
    pub row: u16,
    pub col: u16,
}

/// Colored -- Something which can have it's color set, like an RGB LED
#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
pub struct RgbLed {
    pub color: Srgba,
}
