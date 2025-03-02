use bevy::utils::HashMap;

use super::{expansion_board::ExpansionBoard, resources::LED};

#[derive(Default, Clone)]
pub struct Neutron {
    pub(crate) io_port_path: &'static str,
    pub(crate) exp_port_path: Option<&'static str>,
    pub(crate) indicators: HashMap<&'static str, LED>,
    pub(crate) default_led_brightness: f32,
}

impl Neutron {
    pub fn new(io_port_path: &'static str) -> Self {
        Neutron {
            io_port_path,
            default_led_brightness: 50.,
            ..Neutron::default()
        }
    }

    pub fn add_exp_port(mut self, path: &'static str) -> Self {
        self.exp_port_path = Some(path);
        self
    }

    pub fn default_led_brightness(mut self, value: f32) -> Self {
        self.default_led_brightness = value;
        self
    }

    /// Defines an expansion board
    /// - `leds` - List of ports, where each port is a list of nicknames for the LED e.g. "right ramp", "center spinner"
    pub fn add_expansion_board(
        mut self,
        board: ExpansionBoard,
        leds: Vec<Vec<&'static str>>,
    ) -> Self {
        for (port_index, port) in leds.iter().enumerate() {
            for (index, name) in port.iter().enumerate() {
                if self.indicators.contains_key(name) {
                    panic!("LED names must be unique. Found duplicate for '{}'", name);
                }

                self.indicators.insert(
                    name,
                    LED {
                        r: 0,
                        g: 0,
                        b: 0,
                        expansion_address: board.as_str(),
                        port: port_index as u8,
                        index: index as u8,
                        name,
                    },
                );
            }
        }
        self
    }
}
