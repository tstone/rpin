mod boot;
mod fsp;
mod led;
mod parser;
mod serial;
mod system;
mod watchdog;

pub use fsp::FastPlatform;
pub use fsp::SwitchReporting;
use system::*;
pub use system::{ExpansionBoard, System, SystemConfig};
