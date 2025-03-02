use bevy::utils::HashMap;

use super::{
    expansion_board::ExpansionBoard,
    resources::{Coil, Switch, LED},
    IoBoard,
};

#[derive(Default, Clone)]
pub struct Neutron {
    pub(crate) io_port_path: &'static str,
    pub(crate) exp_port_path: Option<&'static str>,
    pub(crate) indicators: HashMap<&'static str, LED>,
    pub(crate) switch_count: u32,
    pub(crate) coil_count: u32,
    pub(crate) switches: HashMap<&'static str, Switch>,
    pub(crate) coils: HashMap<&'static str, Coil>,
    pub(crate) default_led_brightness: f32,
}

impl Neutron {
    pub fn new(io_port_path: &'static str) -> Self {
        Neutron {
            io_port_path,
            switch_count: 0,
            coil_count: 0,
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

    pub fn add_io_board(mut self, board: &IoBoard) -> Self {
        let (switches, coils) = match board {
            IoBoard::Fast3208 { switches, coils } => (switches, coils),
            IoBoard::Fast1616 { switches, coils } => (switches, coils),
            IoBoard::Fast0804 { switches, coils } => (switches, coils),
            IoBoard::CabinetIO { switches, coils } => (switches, coils),
        };

        // add switches
        for (index, switch) in switches.iter().enumerate() {
            match switch {
                Some(name) => {
                    let id = format!("{:0>x}", self.switch_count + index as u32);
                    self.switches.insert(name, Switch { id, name });
                }
                None => {}
            }
        }

        // add coils
        for (index, switch) in coils.iter().enumerate() {
            match switch {
                Some(name) => {
                    let id = format!("{:0>x}", self.coil_count + index as u32);
                    self.coils.insert(name, Coil { id, name });
                }
                None => {}
            }
        }

        self.switch_count += board.switch_port_count() as u32;
        self.coil_count += board.coil_port_count() as u32;
        self
    }
}
