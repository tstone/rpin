use bevy::prelude::*;
use std::thread;
use std::time::Duration;

use serialport::SerialPort;

use super::events::{ExpPortData, IoPortData};
use super::resources::{ExpPort, IoNetPort};

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

pub fn io_read(port: ResMut<IoNetPort>, mut ev_io_data: EventWriter<IoPortData>) {
    let mut io_net_port = port.0.lock().unwrap();
    let mut buffer: String = String::new();
    let _ = io_net_port.read_to_string(&mut buffer);
    if buffer.len() > 0 {
        trace!("Read {} bytes from IO/NET: {buffer}", buffer.len());
        ev_io_data.send(IoPortData(buffer));
    }
}

pub fn io_write(port: ResMut<IoNetPort>, mut ev_io_data: EventReader<IoPortData>) {
    if !ev_io_data.is_empty() {
        let mut io_net_port = port.0.lock().unwrap();
        for event in ev_io_data.read() {
            match io_net_port.write(format!("{}\r", event.0).as_bytes()) {
                Ok(_) => trace!("Wrote to IO/NET: {}", event.0),
                Err(e) => error!("{:?}", e),
            }
        }
    }
}

pub fn exp_read(port: ResMut<ExpPort>, mut ev_exp_data: EventWriter<ExpPortData>) {
    let mut exp_port = port.0.lock().unwrap();
    let mut buffer: String = String::new();
    let _ = exp_port.read_to_string(&mut buffer);
    if buffer.len() > 0 {
        trace!("Read {} bytes from EXP: {buffer}", buffer.len());
        ev_exp_data.send(ExpPortData(buffer));
    }
}

pub fn exp_write(port: ResMut<ExpPort>, mut ev_exp_data: EventReader<ExpPortData>) {
    if !ev_exp_data.is_empty() {
        let mut exp_port = port.0.lock().unwrap();
        for event in ev_exp_data.read() {
            match exp_port.write(format!("{}\r", event.0).as_bytes()) {
                Ok(_) => trace!("Wrote to EXP: {}", event.0),
                Err(e) => error!("{:?}", e),
            }
        }
    }
}
