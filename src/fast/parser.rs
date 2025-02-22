use std::time::Duration;

use super::fsp::FspResponse;

/// Convert FAST pinball response string into a Message
pub fn parse(input: String) -> Result<FspResponse, &'static str> {
    match input.split_once(":") {
        None => Err("Invalid message syntax: Missing command."),
        Some((identity, all_args)) => {
            let (command, address) = parse_identity(identity.trim_end_matches("\r"));
            let args = parse_args(all_args.trim_end_matches("\r"));
            Ok(parse_to_enum(
                command,
                address.map(|s| String::from(s)),
                args,
            ))
        }
    }
}

fn parse_to_enum(command: &str, address: Option<String>, args: Vec<&str>) -> FspResponse {
    match command {
        "ID" => {
            if args[0] == "F" {
                FspResponse::IdFailed
            } else {
                FspResponse::Id {
                    identity: String::from(args[0]),
                }
            }
        }
        "NI" => FspResponse::NodeId {
            id: args[0].parse::<u8>().unwrap(), // TODO: is this actually in hex?
            serial: String::from(args[1]),
        },
        "NN" => FspResponse::NodeInfo {
            id: args[0].parse::<u8>().unwrap(),
            name: String::from(args[1].trim()),
            firmware: String::from(args[2]),
            driver_count: args[3].parse::<u16>().unwrap(),
            switch_count: args[4].parse::<u16>().unwrap(),
        },
        "WD" => {
            if args[0] == "P" {
                FspResponse::WatchdogValid
            } else if args[0] == "X" || args[0] == "F" {
                FspResponse::WatchdogInvalid
            } else {
                let ms = u64::from_str_radix(args[0], 16).unwrap();
                FspResponse::WatchdogStaus {
                    remaining: Duration::from_millis(ms),
                }
            }
        }
        "CH" => {
            if args[0] == "P" {
                FspResponse::HardwareConfigValid
            } else if args[0] == "X" || args[0] == "F" {
                FspResponse::HardwareConfigInvalid
            } else {
                FspResponse::HardwareConfig {
                    system: args[0].to_string(),
                    data_flags: args[1].to_string(),
                }
            }
        }
        _ => FspResponse::Unknown {
            command: String::from(command),
            address,
        },
    }
}

/// Extract the identity and optional address.
/// e.g. "ID@78" -> ("ID", Some("78"))
fn parse_identity(identity: &str) -> (&str, Option<&str>) {
    match identity.split_once("@") {
        None => (identity, None),
        Some((command, address)) => (command, Some(address)),
    }
}

/// Convert everything after the ":" into a list of arguments
/// Returns [] when there are no arguments
fn parse_args(all_args: &str) -> Vec<&str> {
    if all_args.len() > 0 {
        all_args.split(",").collect()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_id_commands() {
        match parse("ID:NET 12345".to_string()).unwrap() {
            FspResponse::Id { identity } => assert_eq!(identity, "NET 12345"),
            _ => panic!(),
        }
    }

    #[test]
    fn it_parses_id_failed_commands() {
        let msg = parse("ID:F".to_string()).unwrap();
        assert!(matches!(msg, FspResponse::IdFailed));
    }

    #[test]
    fn it_parses_node_id_commands() {
        let raw = "NI:01,A6E616CE514C505136202020FF0E141D";
        match parse(raw.to_string()).unwrap() {
            FspResponse::NodeId { id, serial } => {
                assert_eq!(id, 1);
                assert_eq!(serial, "A6E616CE514C505136202020FF0E141D");
            }
            _ => panic!(),
        }
    }

    #[test]
    fn it_parses_watchdog_valid() {
        let msg = parse("WD:P".to_string()).unwrap();
        assert!(matches!(msg, FspResponse::WatchdogValid));
    }

    #[test]
    fn it_parses_watchdog_invalid() {
        let msg = parse("WD:F".to_string()).unwrap();
        assert!(matches!(msg, FspResponse::WatchdogInvalid));
    }

    #[test]
    fn it_parses_watchdog_status_commands() {
        match parse("WD:000FF839".to_string()).unwrap() {
            FspResponse::WatchdogStaus { remaining } => {
                assert_eq!(remaining.as_millis(), 1046585);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn it_parses_node_info_commands() {
        let raw = "NN:01,FP-I/O-1616-2  ,00.89,04,06,10,10,00,00,00,00";
        let result = parse(raw.to_string()).unwrap();
        match result {
            FspResponse::NodeInfo {
                id,
                name,
                firmware,
                driver_count,
                switch_count,
            } => {
                assert_eq!(id, 1);
                assert_eq!(name, "FP-I/O-1616-2");
                assert_eq!(firmware, "00.89");
                assert_eq!(driver_count, 4);
                assert_eq!(switch_count, 6);
            }
            _ => println!("got here"), // _ => panic!(),
        }
    }
}
