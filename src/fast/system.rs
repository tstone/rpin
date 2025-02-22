use log;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::fast::watchdog;

use super::fsp::FastPlatform;
use super::fsp::FspRequest;
use super::fsp::SwitchReporting;
use super::serial;

pub enum InternalEvent {
    IncomingData { raw: String },
}

use super::boot::*;
use InternalEvent::*;

pub struct SystemConfig {
    pub system: FastPlatform,
    pub switch_reporting: SwitchReporting,
    pub io_net_port_path: &'static str,
}

pub struct System;

impl System {
    pub fn start(config: SystemConfig) {
        let (tx, rx) = mpsc::channel::<InternalEvent>();
        let main_tx = tx;
        let main_rx = rx;

        let (tx, rx) = mpsc::channel::<FspRequest>();
        let io_net_tx = tx;
        let io_net_rx = rx;

        // I/O Net connection
        serial::spawn(config.io_net_port_path, main_tx.clone(), io_net_rx);

        wait_for_system_boot(&main_rx, &io_net_tx);
        configure_hardware(&config, &main_rx, &io_net_tx);
        log::info!("System online.");

        watchdog::spawn(io_net_tx.clone());

        loop {
            match main_rx.try_recv() {
                Err(_) => {}
                Ok(event) => match event {
                    IncomingData { raw } => log::info!("Received response: {raw}"),
                },
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
