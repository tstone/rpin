use bevy::prelude::*;

use crate::pinball::MachineState;

use super::{
    events::{event_listener, ExpPortData, FastIoEvent, IoPortData},
    resources::{ExpPort, IoNetPort},
    serial::*,
    ExpansionBoard,
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
    pub(crate) exp_port_path: Option<&'static str>,
    pub(crate) default_led_brightness: f32,
}

impl Neutron {
    pub fn new(io_port_path: &'static str) -> Self {
        Neutron {
            io_port_path,
            default_led_brightness: 50.,
            ..Neutron::default()
        }
    }

    #[allow(dead_code)]
    pub fn add_exp_port(mut self, path: &'static str) -> Self {
        self.exp_port_path = Some(path);
        self
    }

    #[allow(dead_code)]
    pub fn default_led_brightness(mut self, value: f32) -> Self {
        self.default_led_brightness = value;
        self
    }
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
        app.add_event::<IoPortData>();
        app.add_event::<FastIoEvent>();
        app.add_systems(FixedUpdate, io_read);
        app.add_systems(FixedUpdate, io_write);

        // Expansion port
        if let Some(port_path) = self.exp_port_path {
            let exp_path = connect(port_path);
            let mutex = Mutex::new(exp_path);
            app.insert_resource(ExpPort(Arc::new(mutex)));
            app.add_event::<ExpPortData>();
            app.add_systems(FixedUpdate, exp_read);
            app.add_systems(FixedUpdate, exp_write);
        }

        app.insert_resource(NeutronConfig {
            default_led_brightness: self.default_led_brightness,
            expansion_boards: Vec::new(),
            ..Default::default()
        });

        app.add_systems(Startup, event_listener);
        app.add_systems(OnEnter(MachineState::Waiting), reset_leds);
    }
}

#[derive(Resource, Debug, Clone, Default)]
#[allow(dead_code)]
struct NeutronConfig {
    pub default_led_brightness: f32,
    pub expansion_boards: Vec<ExpansionBoard>,
}

fn reset_leds() {
    todo!()
}
