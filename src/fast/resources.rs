use bevy::prelude::*;
use serialport::SerialPort;
use std::sync::{Arc, Mutex};

#[derive(Resource, Debug)]
pub struct IoNetPort(pub Arc<Mutex<Box<dyn SerialPort>>>);

#[derive(Resource, Debug)]
pub struct ExpPort(pub Arc<Mutex<Box<dyn SerialPort>>>);
