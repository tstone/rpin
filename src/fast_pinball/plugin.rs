use bevy::{prelude::*, utils::hashbrown::HashMap};

use super::{
    events::{ExpPortData, FastIoEvent, IoPortData},
    resources::{Coil, Coils, ExpPort, Indicators, IoNetPort, Switch, Switches},
    serial::*,
    systems, Neutron,
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

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

        app.insert_resource(Indicators {
            by_name: self.indicators.clone(),
        });

        let mut switches_by_id = HashMap::<String, Switch>::new();
        for switch in self.switches.values() {
            switches_by_id.insert(switch.id.clone(), switch.clone());
        }

        app.insert_resource(Switches {
            by_name: self.switches.clone(),
            by_id: switches_by_id,
        });

        let mut coils_by_id = HashMap::<String, Coil>::new();
        for coil in self.coils.values() {
            coils_by_id.insert(coil.id.clone(), coil.clone());
        }

        app.insert_resource(Coils {
            by_name: self.coils.clone(),
            by_id: coils_by_id,
        });

        app.add_systems(Startup, systems::reset_leds);
    }
}
