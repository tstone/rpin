use log;
use serialport::SerialPort;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use super::fsp::{FastExpReq, FastIoReq};
use super::InternalEvent;

fn connect(port_path: &str) -> Box<dyn SerialPort> {
    let baud_rate = 921_600;
    let port = serialport::new(port_path, baud_rate)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open();
    match port {
        Ok(port) => port,
        Err(e) => {
            log::error!("{:?} - {}", e.kind, e.description);
            thread::sleep(Duration::from_millis(300));
            return connect(port_path);
        }
    }
}

pub fn spawn_io(port_path: &'static str, main_tx: Sender<InternalEvent>) -> Sender<FastIoReq> {
    let (tx, io_rx) = mpsc::channel::<FastIoReq>();

    thread::spawn(move || {
        log::debug!("Opening serial port at {port_path}");
        let mut io_net_port = connect(port_path);

        loop {
            // Send outgoing messages (if present from another thread)
            if let Ok(req) = io_rx.try_recv() {
                let msg = req.to_string();
                let outbound = format!("{msg}\r");
                log::trace!("Writing to IO@{port_path}: {outbound}");
                let _ = io_net_port.write(outbound.as_bytes());
            }

            // Read incoming data
            let mut buffer: String = String::new();
            let _ = io_net_port.read_to_string(&mut buffer);
            if buffer.len() > 0 {
                log::trace!("Read {} bytes from {port_path}: {buffer}", buffer.len());
                let _ = main_tx.send(InternalEvent::IncomingIoData { raw: buffer });
            }

            thread::sleep(Duration::from_millis(1));
        }
    });

    tx
}

pub fn spawn_exp(port_path: &'static str, main_tx: Sender<InternalEvent>) -> Sender<FastExpReq> {
    let (tx, io_rx) = mpsc::channel::<FastExpReq>();

    thread::spawn(move || {
        log::debug!("Opening serial port at {port_path}");
        let mut exp_port = connect(port_path);

        loop {
            // Send outgoing messages (if present from another thread)
            if let Ok(req) = io_rx.try_recv() {
                let msg = req.to_string();
                let outbound = format!("{msg}\r");
                log::trace!("Writing to EXP@{port_path}: {outbound}");
                let _ = exp_port.write(outbound.as_bytes());
            }

            // Read incoming data
            let mut buffer: String = String::new();
            let _ = exp_port.read_to_string(&mut buffer);
            if buffer.len() > 0 {
                log::trace!("Read {} bytes from {port_path}: {buffer}", buffer.len());
                let _ = main_tx.send(InternalEvent::IncomingExpData { raw: buffer });
            }

            thread::sleep(Duration::from_millis(1));
        }
    });

    tx
}
