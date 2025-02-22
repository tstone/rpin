mod boot;
mod fsp;
mod parser;
mod serial;
mod system;
mod watchdog;

pub use fsp::FastPlatform;
pub use fsp::SwitchReporting;
pub use system::System;
pub use system::SystemConfig;
use system::*;
