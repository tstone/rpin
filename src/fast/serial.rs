use log;
use serialport::SerialPort;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use super::InternalEvent;

fn connect(port_path: &str) -> Box<dyn SerialPort> {
    let baud_rate = 921_600;
    serialport::new(port_path, baud_rate)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open()
        .expect("Failed to open IO port")
}

pub fn spawn(
    io_net_port_path: &'static str,
    main_tx: Sender<InternalEvent>,
    io_rx: Receiver<String>,
) {
    thread::spawn(move || {
        log::debug!("Opening serial port at {io_net_port_path}");
        let mut io_net_port = connect(io_net_port_path);

        loop {
            // Send outgoing messages (if present from another thread)
            if let Ok(str) = io_rx.try_recv() {
                let outbound = format!("{str}\r");
                log::trace!("Writing {} bytes to {io_net_port_path}", outbound.len());
                let _ = io_net_port.write(outbound.as_bytes());
            }

            // Read incoming data
            let mut buffer: String = String::new();
            let _ = io_net_port.read_to_string(&mut buffer);
            if buffer.len() > 0 {
                log::trace!("Read {} bytes from {io_net_port_path}", buffer.len());
                let _ = main_tx.send(InternalEvent::IncomingData { raw: buffer });
            }

            thread::sleep(Duration::from_millis(1));
        }
    });
}
