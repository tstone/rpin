use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::fsp::{
    FastPlatform, FspRequest,
    FspResponse::{self, *},
};
use super::{parser, InternalEvent, SystemConfig};

fn wait_for_response(main_rx: &Receiver<InternalEvent>) -> Option<FspResponse> {
    match main_rx.recv_timeout(Duration::from_millis(100)) {
        Ok(event) => match event {
            InternalEvent::IncomingData { raw } => match parser::parse(raw) {
                Ok(resp) => Some(resp),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

/// Fired at startup once the serial port connection has been successfully opened
pub fn wait_for_system_boot(main_rx: &Receiver<InternalEvent>, io_tx: &Sender<FspRequest>) {
    loop {
        let _ = io_tx.send(FspRequest::GetId);
        match wait_for_response(main_rx) {
            Some(msg) => match msg {
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
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Fired once the system has booted
pub fn configure_hardware(
    config: &SystemConfig,
    main_rx: &Receiver<InternalEvent>,
    io_tx: &Sender<FspRequest>,
) {
    match config.system {
        // Nano board does not required hardware config command
        FastPlatform::Nano => {}
        _ => {
            let _ = io_tx.send(FspRequest::ConfigureHardware {
                platform: config.system,
                switch_reporting: config.switch_reporting,
            });

            match wait_for_response(main_rx) {
                Some(msg) => match msg {
                    HardwareConfigValid => return,
                    HardwareConfigInvalid => {
                        log::error!("Could not configure hardware.");
                        panic!();
                    }
                    _ => {}
                },
                _ => {}
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
