use bevy::{prelude::Resource, utils::HashMap};
use serialport::SerialPort;
use std::sync::{Arc, Mutex};

/// Configuration for a single LED
/// See: https://fastpinball.com/programming/exp/#expansion-board-addresses
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct LED {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    // A short human readable description of this LED, e.g. "left ramp"
    pub name: &'static str,
    pub expansion_address: &'static str,
    /// Port on expansion board
    pub port: u8,
    /// Index of LED on port
    pub index: u8,
}

/// Configuration for a single switch
/// See: https://fastpinball.com/modern/switches/
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Switch {
    pub id: String,
    // A short human readable description of this LED, e.g. "left ramp"
    pub name: &'static str,
}

/// Configuration for a single switch
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Coil {
    pub id: String,
    // A short human readable description of this LED, e.g. "left ramp"
    pub name: &'static str,
}

#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct Indicators {
    pub leds: HashMap<&'static str, LED>,
}

#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct Switches {
    pub switches: HashMap<&'static str, Switch>,
}

#[derive(Resource, Debug)]
#[allow(dead_code)]
pub struct Coils {
    pub coils: HashMap<&'static str, Coil>,
}

#[derive(Resource, Debug)]
pub struct IoNetPort(pub Arc<Mutex<Box<dyn SerialPort>>>);

#[derive(Resource, Debug)]
pub struct ExpPort(pub Arc<Mutex<Box<dyn SerialPort>>>);
