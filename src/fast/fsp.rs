#[derive(Debug)]
#[allow(dead_code)]
pub enum FspResponse {
    Id {
        identity: String,
    },
    NodeId {
        id: u8,
        serial: String,
    },
    Unknown {
        command: String,
        address: Option<String>,
        data: Option<String>,
    },
    IdFailed,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FspRequest {
    Id,
    NodeId,
}

impl FspRequest {
    pub fn to_string(&self) -> String {
        match self {
            Self::Id => String::from("ID:"),
            Self::NodeId => String::from("NI:"),
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
