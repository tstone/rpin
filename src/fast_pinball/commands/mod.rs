use bevy::ecs::system::Commands;
use colors_transform::Hsl;
use set_all_leds::SetAllLEDs;
use set_leds::SetLED;

mod color;
mod set_all_leds;
mod set_leds;

pub trait FastCommandsExt {
    fn set_all_leds<T: Into<String>>(&mut self, expansion_board: T, color: Hsl);
    fn set_led<T: Into<String>>(&mut self, name: T, color: Hsl);
}

impl<'w, 's> FastCommandsExt for Commands<'w, 's> {
    fn set_all_leds<T: Into<String>>(&mut self, expansion_board: T, color: Hsl) {
        self.queue(SetAllLEDs {
            addr: expansion_board.into(),
            color,
        });
    }

    fn set_led<T: Into<String>>(&mut self, name: T, color: Hsl) {
        self.queue(SetLED {
            name: name.into(),
            color,
        });
    }
}
