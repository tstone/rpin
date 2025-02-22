use log;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::fsp::FspRequest;
use super::serial;

pub enum InternalEvent {
    IncomingData { raw: String },
}

use super::boot::*;
use InternalEvent::*;

pub struct System;

impl System {
    pub fn start(io_net_port_path: &'static str) {
        let (tx, rx) = mpsc::channel::<InternalEvent>();
        let main_tx = tx;
        let main_rx = rx;

        let (tx, rx) = mpsc::channel::<FspRequest>();
        let io_net_tx = tx;
        let io_net_rx = rx;

        // I/O Net connection
        serial::spawn(io_net_port_path, main_tx.clone(), io_net_rx);

        wait_for_system_boot(&main_rx, &io_net_tx);
        log::info!("System online.");

        loop {
            match main_rx.try_recv() {
                Err(_) => {}
                Ok(event) => match event {
                    IncomingData { raw } => log::info!("Recieved response: {raw}"),
                },
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
