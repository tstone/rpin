#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IoBoard {
    Fast3208 {
        switches: Vec<Option<&'static str>>,
        coils: Vec<Option<&'static str>>,
    },
    Fast1616 {
        switches: Vec<Option<&'static str>>,
        coils: Vec<Option<&'static str>>,
    },
    Fast0804 {
        switches: Vec<Option<&'static str>>,
        coils: Vec<Option<&'static str>>,
    },
    CabinetIO {
        switches: Vec<Option<&'static str>>,
        coils: Vec<Option<&'static str>>,
    },
}

impl IoBoard {
    /// Gets the total number of drivers this board can support
    pub fn coil_port_count(&self) -> u8 {
        match self {
            Self::Fast3208 { .. } => 8,
            Self::Fast1616 { .. } => 16,
            Self::Fast0804 { .. } => 4,
            Self::CabinetIO { .. } => 8,
        }
    }

    /// Gets the total number of switches this board can support
    pub fn switch_port_count(&self) -> u8 {
        match self {
            Self::Fast3208 { .. } => 32,
            Self::Fast1616 { .. } => 16,
            Self::Fast0804 { .. } => 8,
            Self::CabinetIO { .. } => 24,
        }
    }
}
