use super::fsp::FspResponse;

/// Convert FAST pinball response string into a Message
pub fn parse(input: String) -> Result<FspResponse, &'static str> {
    match input.split_once(":") {
        None => Err("Invalid message syntax: Missing command."),
        Some((identity, all_args)) => {
            let (command, address) = parse_identity(identity.trim_end_matches("\r"));
            let args = parse_args(all_args.trim_end_matches("\r"));

            match command {
                "ID" => {
                    if args[0] == "F" {
                        Ok(FspResponse::IdFailed)
                    } else {
                        Ok(FspResponse::Id {
                            identity: String::from(args[0]),
                        })
                    }
                }
                "NI" => Ok(FspResponse::NodeId {
                    id: args[0].parse::<u8>().unwrap(), // TODO: is this actually in hex?
                    serial: String::from(args[1]),
                }),
                _ => Ok(FspResponse::Unknown {
                    command: String::from(command),
                    address: address.map(|s| String::from(s)),
                    data: Some(String::from(all_args)),
                }),
            }
        }
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
}
