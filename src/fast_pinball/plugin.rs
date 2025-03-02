use bevy::{prelude::*, utils::hashbrown::HashSet};
use colors_transform::Hsl;

use super::{
    events::{ExpPortData, IoPortData},
    resources::{ExpPort, Indicators, IoNetPort},
    serial::*,
    FastCommandsExt, Neutron,
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

impl Plugin for Neutron {
    fn build(&self, app: &mut bevy::app::App) {
        // IO/NET port
        let mut io_port = connect(self.io_port_path.unwrap());

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

        // TODO: confgiure drivers
        // TODO: configure switches

        let mutex = Mutex::new(io_port);
        app.insert_resource(IoNetPort(Arc::new(mutex)));
        app.add_event::<IoPortData>();
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

        app.insert_resource(Indicators {
            leds: self.indicators.clone(),
        });

        app.add_systems(Startup, reset_leds);
    }
}

/// Set all LEDs to black (off) to clear any prior state
fn reset_leds(indicators: Res<Indicators>, mut commands: Commands) {
    let mut expansion_boards_with_leds = HashSet::<&str>::new();
    for led in &indicators.leds {
        expansion_boards_with_leds.insert(&led.expansion_address);
    }
    for addr in expansion_boards_with_leds.iter() {
        commands.set_all_leds(*addr, Hsl::from(0., 0., 0.));
    }
}
