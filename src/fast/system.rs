use log;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::fsp::FspResponse::*;
use super::{parser, serial};

pub enum InternalEvent {
    IncomingData { raw: String },
}

use InternalEvent::*;

pub struct System;

impl System {
    pub fn start(io_net_port_path: &'static str) {
        let (tx, rx) = mpsc::channel::<InternalEvent>();
        let main_tx = tx;
        let main_rx = rx;

        let (tx, rx) = mpsc::channel::<String>();
        let io_net_tx = tx;
        let io_net_rx = rx;

        // I/O Net connection
        serial::spawn(io_net_port_path, main_tx.clone(), io_net_rx);

        wait_for_system_boot(&main_rx, &io_net_tx);
        log::info!("System online.");
    }
}

fn wait_for_system_boot(main_rx: &Receiver<InternalEvent>, io_tx: &Sender<String>) {
    loop {
        let _ = io_tx.send(String::from("ID:"));

        match main_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => match event {
                IncomingData { raw } => match parser::parse(raw) {
                    Ok(msg) => match msg {
                        Id { identity } => {
                            log::debug!("Identified board as {identity}");
                            break;
                        }
                        IdFailed => {
                            log::debug!("Startup identification failed. Trying again.");
                        }
                        _ => {}
                    },
                    _ => {}
                },
            },
            _ => {}
        }
        thread::sleep(Duration::from_millis(1));
    }
}
