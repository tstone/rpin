use bevy::prelude::*;
use std::thread;
use std::time::Duration;

use serialport::SerialPort;

use crate::fast::parser::parse;

use super::resources::{ExpPort, IoNetPort};
use super::FastIoEvent;

pub fn connect(port_path: &str) -> Box<dyn SerialPort> {
    let baud_rate = 921_600;
    let port = serialport::new(port_path, baud_rate)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open();
    match port {
        Ok(port) => port,
        Err(e) => {
            error!("{:?} - {}", e.kind, e.description);
            thread::sleep(Duration::from_millis(300));
            return connect(port_path);
        }
    }
}

pub fn io_read(port: ResMut<IoNetPort>, mut ev_io: EventWriter<FastIoEvent>) {
    let mut io_net_port = port.0.lock().unwrap();
    let mut buffer: String = String::new();
    let _ = io_net_port.read_to_string(&mut buffer);
    if buffer.len() > 0 {
        trace!("Read {} bytes from IO/NET: {buffer}", buffer.len());
        match parse(buffer) {
            Ok(event) => {
                ev_io.send(event);
            }
            Err(e) => error!("{e}"),
        }
    }
}

#[allow(dead_code)]
pub fn io_write(data: String, port: &ResMut<IoNetPort>) {
    let mut io_net_port = port.0.lock().unwrap();
    match io_net_port.write(format!("{}\r", data).as_bytes()) {
        Ok(_) => trace!("Wrote to IO/NET: {}", data),
        Err(e) => error!("{:?}", e),
    }
}

pub fn exp_read(port: ResMut<ExpPort>) {
    let mut exp_port = port.0.lock().unwrap();
    let mut buffer: String = String::new();
    let _ = exp_port.read_to_string(&mut buffer);
    if buffer.len() > 0 {
        trace!("Read {} bytes from EXP: {buffer}", buffer.len());
        // TODO: right now there doesn't seem to be any use for data back from EXP bus
        // so until there is this just logs it out
    }
}

pub fn exp_write(data: String, port: &ResMut<ExpPort>) {
    let mut exp_port = port.0.lock().unwrap();
    match exp_port.write(format!("{}\r", data).as_bytes()) {
        Ok(_) => trace!("Wrote to EXP: {}", data),
        Err(e) => error!("{:?}", e),
    }
}
