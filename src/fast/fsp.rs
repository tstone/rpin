use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
pub enum FspResponse {
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
    Unknown {
        command: String,
        address: Option<String>,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FspRequest {
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
}

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

impl FspRequest {
    pub fn to_string(&self) -> String {
        match self {
            Self::GetId => String::from("ID:"),
            Self::GetNodeId => String::from("NI:"),
            Self::GetNodeInfo => String::from("NN:"),
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

// fn fsp_format(command: &str, address: Option<&str>, args: Option<Vec<&str>>) -> String {
//     let left = match address {
//         Some(addr) => format!("{}@{}", command, addr),
//         None => command.to_string(),
//     };
//     // let right = args.join(",");
//     let right = match args {
//         Some(x) => x,
//         None => Vec::new(),
//     }
//     .join(",");
//     return format!("{}:{}", left, right);
// }
