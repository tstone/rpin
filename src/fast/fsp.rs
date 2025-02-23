use std::time::Duration;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum FastPlatform {
    Neuron,
    Nano,
    RetroSystem11,
    RetroWpc89,
    RetroWpc95,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum SwitchReporting {
    Verbose,
    Read,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FastIoResp {
    Id {
        identity: String,
    },
    IdFailed,
    NodeId {
        id: u8,
        serial: String,
    },
    NodeInfo {
        id: u8,
        name: String,
        firmware: String,
        driver_count: u16,
        switch_count: u16,
    },
    HardwareConfigValid,
    HardwareConfigInvalid,
    HardwareConfig {
        system: String,
        data_flags: String,
    },
    WatchdogValid,
    WatchdogInvalid,
    WatchdogStaus {
        remaining: Duration,
    },
    SwitchOpened {
        id: u32,
    },
    SwitchClosed {
        id: u32,
    },
    Unknown {
        command: String,
        address: Option<String>,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FastIoReq {
    GetId,
    GetNodeId,
    GetNodeInfo,
    Watchdog {
        time: Duration,
    },
    ConfigureHardware {
        platform: FastPlatform,
        switch_reporting: SwitchReporting,
    },
    GetAllSwitchState,
}

impl FastIoReq {
    pub fn to_string(&self) -> String {
        match self {
            Self::GetId => "ID:".to_string(),
            Self::GetNodeId => "NI:".to_string(),
            Self::GetNodeInfo => "NN:".to_string(),
            Self::GetAllSwitchState => "SA:".to_string(),
            Self::Watchdog { time } => format!("WD:{}", time.as_millis()),
            Self::ConfigureHardware {
                platform,
                switch_reporting,
            } => {
                let base = match platform {
                    FastPlatform::Neuron => "CH:2000",
                    FastPlatform::RetroSystem11 => "CH:0011",
                    FastPlatform::RetroWpc89 => "CH:0089",
                    FastPlatform::RetroWpc95 => "CH:0095",
                    _ => panic!(),
                };
                let sw = match switch_reporting {
                    SwitchReporting::Read => "00",
                    SwitchReporting::Verbose => "01",
                };
                return format!("{base},{sw}");
            }
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct LEDState {
    pub index: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FastExpReq {
    GetId {
        address: String,
    },
    ClearAllLEDs {
        address: String,
    },
    SetAllLEDs {
        address: String,
        r: u8,
        g: u8,
        b: u8,
    },
    SetFadeRate {
        address: String,
        rate: u16,
    },
    SetLEDs {
        address: String,
        states: Vec<LEDState>,
    },
}

impl FastExpReq {
    pub fn to_string(&self) -> String {
        match self {
            Self::GetId { address } => fsp_format("ID", address, None),
            Self::ClearAllLEDs { address } => {
                fsp_format("RA", address, Some(vec!["0".to_string()]))
            }
            Self::SetAllLEDs { address, r, g, b } => {
                let rgb_hex = format!("{:#x}{:#x}{:#x}", r, g, b);
                fsp_format("RA", address, Some(vec![rgb_hex]))
            }
            Self::SetFadeRate { address, rate } => {
                let rate_hex = format!("{rate:x}");
                fsp_format("RF", address, Some(vec![rate_hex]))
            }
            Self::SetLEDs { address, states } => {
                let mut args: Vec<String> = Vec::new();
                for state in states {
                    let arg = format!(
                        "{}{}{}{}",
                        u8_to_hex(&state.index),
                        u8_to_hex(&state.r),
                        u8_to_hex(&state.g),
                        u8_to_hex(&state.b)
                    );
                    args.push(arg);
                }
                fsp_format("RS", address, Some(args))
            }
        }
    }
}

fn u8_to_hex(d: &u8) -> String {
    let h = format!("{d:x}");
    match h.len() {
        1 => format!("0{h}"),
        _ => h,
    }
}

fn fsp_format(command: &str, address: &String, args: Option<Vec<String>>) -> String {
    let arg_list = match args {
        Some(x) => x,
        None => Vec::new(),
    }
    .join(",");
    return format!("{command}@{address}:{arg_list}");
}
