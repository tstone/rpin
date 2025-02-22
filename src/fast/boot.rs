use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::fsp::{FspRequest, FspResponse::*};
use super::{parser, InternalEvent};

pub fn wait_for_system_boot(main_rx: &Receiver<InternalEvent>, io_tx: &Sender<FspRequest>) {
    loop {
        let _ = io_tx.send(FspRequest::GetId);

        match main_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => match event {
                InternalEvent::IncomingData { raw } => match parser::parse(raw) {
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
