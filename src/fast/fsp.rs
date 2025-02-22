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
    Unknown {
        command: String,
        address: Option<String>,
        data: Option<String>,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FspRequest {
    GetId,
    GetNodeId,
    GetNodeInfo,
}

impl FspRequest {
    pub fn to_string(&self) -> String {
        match self {
            Self::GetId => String::from("ID:"),
            Self::GetNodeId => String::from("NI:"),
            Self::GetNodeInfo => String::from("NN:"),
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
