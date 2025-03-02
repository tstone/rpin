use bevy::ecs::system::Commands;
use colors_transform::Hsl;
use set_all_leds::SetAllLEDs;

mod color;
mod set_all_leds;

pub trait FastCommandsExt {
    fn set_all_leds<T: Into<String>>(&mut self, expansion_board: T, color: Hsl);
}

// implement our trait for Bevy's `Commands`
impl<'w, 's> FastCommandsExt for Commands<'w, 's> {
    fn set_all_leds<T: Into<String>>(&mut self, expansion_board: T, color: Hsl) {
        self.queue(SetAllLEDs {
            addr: expansion_board.into(),
            color,
        });
    }
}
