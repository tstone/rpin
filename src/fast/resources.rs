use bevy::prelude::Resource;
use serialport::SerialPort;
use std::{
    collections::HashMap,
    hash::Hash,
    sync::{Arc, Mutex},
};

#[derive(Resource, Debug)]
pub struct IoNetPort(pub Arc<Mutex<Box<dyn SerialPort>>>);

#[derive(Resource, Debug)]
pub struct ExpPort(pub Arc<Mutex<Box<dyn SerialPort>>>);

/// Configuration for a single LED
/// See: https://fastpinball.com/programming/exp/#expansion-board-addresses
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct HardwareLed {
    pub expansion_address: &'static str,
    /// Port on expansion board
    pub port: u8,
    /// Index of LED on port
    pub index: u8,
}

#[derive(Resource, Debug)]
pub struct HardwareLedMapping<K: Copy + Eq + Hash + Send + Sync + 'static>(
    pub HashMap<K, Vec<HardwareLed>>,
);
