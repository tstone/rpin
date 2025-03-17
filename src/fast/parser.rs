use bevy::prelude::*;

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub enum FastIoEvent {
    SwitchOpened { id: String },
    SwitchClosed { id: String },
}

/// Convert FAST pinball response string into a Message
pub fn parse(input: String) -> Result<FastIoEvent, String> {
    match input.split_once(":") {
        None => Err("Invalid message syntax: Missing command.".to_string()),
        Some((cmd, all_args)) => {
            let args = parse_args(all_args.trim_end_matches("\r"));
            match cmd {
                "-L" => Ok(FastIoEvent::SwitchClosed {
                    id: args[0].to_string(),
                }),
                "/L" => Ok(FastIoEvent::SwitchOpened {
                    id: args[0].to_string(),
                }),
                raw => Err(raw.to_string()),
            }
        }
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
