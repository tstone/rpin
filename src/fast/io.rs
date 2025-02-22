#[path = "fsp.rs"]
mod fsp;
use fsp::FastMessage;
use serialport::SerialPort;

pub fn format(command: &str, address: Option<&str>, args: Option<Vec<&str>>) -> String {
    let left = match address {
        Some(addr) => format!("{}@{}", command, addr),
        None => command.to_string(),
    };
    // let right = args.join(",");
    let right = match args {
        Some(x) => x,
        None => Vec::new(),
    }
    .join(",");
    return format!("{}:{}", left, right);
}

pub fn format_message(msg: FastMessage) -> String {
    return format(msg.command, msg.address, Some(msg.args));
}

// TODO: this should probably be something like
// fast::neutron::open(...)
// neutron.io.send(fast::id())
pub fn send(
    mut port: Box<dyn SerialPort>,
    command: &str,
    address: Option<&str>,
    args: Option<Vec<&str>>,
) {
    let body = format(command, address, args);
    let outbound = format!("{}\r", body);
    let _ = port.write(outbound.as_bytes());
}

pub fn verse(n: u32) -> String {
    return match n {
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take one down and pass it around, no more bottles of beer on the wall.\n\n",
        ),
        0 => format!(
            "No more bottle of beer on the wall, no more bottle of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n\n",
        ),
        _ => format!(
            "{n} bottles of beer on the wall, {n} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n\n",
            n - 1
        ),
    };
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let steps = start - end;
    for i in 0..=steps {
        let bottle_count = start - i;
        result.push_str(&verse(bottle_count));
    }
    return result;
}
