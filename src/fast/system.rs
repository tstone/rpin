use colorous::Color;
use log;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use super::boot::*;
use super::fsp::FastPlatform;
use super::fsp::SwitchReporting;
use super::serial;
use crate::fast::led;
use crate::fast::led::id_builder;
use crate::fast::watchdog;

pub enum InternalEvent {
    IncomingIoData { raw: String },
    IncomingExpData { raw: String },
}
use InternalEvent::*;

pub struct ExpansionBoard {
    pub id: &'static str,
    pub leds: Vec<u8>,
}

pub struct SystemConfig {
    pub system: FastPlatform,
    pub switch_reporting: SwitchReporting,
    pub expansion_boards: Vec<ExpansionBoard>,
    pub io_port_path: &'static str,
    pub exp_port_path: &'static str,
}

pub struct System;

impl System {
    pub fn start(config: SystemConfig) {
        let (tx, rx) = mpsc::channel::<InternalEvent>();
        let main_tx = tx;
        let main_rx = rx;

        let io_tx = serial::spawn_io(config.io_port_path, main_tx.clone());
        let exp_tx = serial::spawn_exp(config.exp_port_path, main_tx.clone());

        wait_for_system_boot(&main_rx, &io_tx);
        configure_hardware(&config, &main_rx, &io_tx);
        watchdog::spawn(io_tx.clone());
        clear_leds(&config.expansion_boards, exp_tx.clone());

        log::info!("System online.");

        led::anim::run(
            exp_tx,
            led::linear_gradient::generate(
                colorous::PLASMA,
                // Color {
                //     r: 89,
                //     g: 50,
                //     b: 180,
                // },
                id_builder::linear("48".to_string(), 0, 8),
                10,
                0,
            ),
        );

        loop {
            match main_rx.try_recv() {
                Err(_) => {}
                Ok(event) => match event {
                    IncomingIoData { raw } => log::info!("Received IO response: {raw}"),
                    IncomingExpData { raw } => log::info!("Received EXP response: {raw}"),
                },
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
}
