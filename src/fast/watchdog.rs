use super::fsp::FspRequest;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

// The watchdog feature must ping the board on a regular cadence
// to prove the computer is still alive
//
// Reference: https://fastpinball.com/fast-serial-protocol/net/wd/
pub fn spawn(io_tx: Sender<FspRequest>) {
    log::info!("Starting watchdog.");
    thread::spawn(move || loop {
        let _ = io_tx.send(FspRequest::Watchdog {
            time: Duration::from_millis(750),
        });
        thread::sleep(Duration::from_millis(500));
    });
}
