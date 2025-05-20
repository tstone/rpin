use bevy::prelude::*;

use super::{
    parser::FastIoEvent,
    resources::{ExpPort, IoNetPort},
    serial::*,
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

/// Neutron - Bevy plugin which connects to the Fast Pinball Neutron board
#[derive(Default, Clone)]
pub struct Neutron {
    pub(crate) io_port_path: &'static str,
    pub(crate) exp_port_path: &'static str,
}

impl Plugin for Neutron {
    fn build(&self, app: &mut bevy::app::App) {
        // IO/NET port
        let mut io_port = connect(self.io_port_path);

        // Wait for Neutron to boot up
        loop {
            let _ = io_port.write("ID:\r".as_bytes());
            thread::sleep(Duration::from_millis(50));
            let mut resp = String::new();
            let _ = io_port.read_to_string(&mut resp);
            trace!("Identify board response: {resp}");
            if resp.starts_with("ID:") && resp.trim_end() != "ID:F" {
                debug!("{}", resp.trim_end());
                break;
            }
        }

        // Tell neutron which board it is
        {
            let _ = io_port.write("CH:2000,0\r".as_bytes());
            thread::sleep(Duration::from_millis(50));
            let mut resp = String::new();
            let _ = io_port.read_to_string(&mut resp);
            trace!("Configured Hardware response: {resp}");
            if resp.trim_end() != "CH:P" {
                error!("{}", resp.trim_end());
                panic!("Attempted to configure hardware as Neutron but failed");
            }
        }

        // TODO: watchdog

        let mutex = Mutex::new(io_port);
        app.insert_resource(IoNetPort(Arc::new(mutex)));
        app.add_event::<FastIoEvent>();
        app.add_systems(FixedFirst, io_read);

        // Expansion port
        let exp_path = connect(self.exp_port_path);
        let mutex = Mutex::new(exp_path);
        app.insert_resource(ExpPort(Arc::new(mutex)));
        app.add_systems(FixedFirst, exp_read);
    }
}
