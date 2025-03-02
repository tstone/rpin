use bevy::prelude::*;

#[derive(Event)]
pub struct IoPortData(pub String);

#[derive(Event)]
pub struct ExpPortData(pub String);

#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub enum FastIoEvent {
    SwitchOpened { id: String },
    SwitchClosed { id: String },
}

pub fn event_listener(
    mut ev_port_data: EventReader<IoPortData>,
    mut ev_io: EventWriter<FastIoEvent>,
) {
    for event in ev_port_data.read() {
        match parse(event.0.clone()) {
            Ok(event) => {
                ev_io.send(event);
            }
            Err(e) => error!("{e}"),
        }
    }
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
