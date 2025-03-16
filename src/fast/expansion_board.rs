use std::fmt;

/// See: https://fastpinball.com/programming/exp/#expansion-board-addresses
#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum ExpansionBoard {
    Neutron,
    FpExp0071 { jumper_0: bool, jumper_1: bool },
    FpExp0081 { jumper_0: bool, jumper_1: bool },
    FpExp0091 { jumper_0: bool, jumper_1: bool },
}

impl ExpansionBoard {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Neutron { .. } => "48",
            Self::FpExp0071 {
                jumper_0: false,
                jumper_1: false,
            } => "B4",
            Self::FpExp0071 {
                jumper_0: true,
                jumper_1: false,
            } => "B5",
            Self::FpExp0071 {
                jumper_0: false,
                jumper_1: true,
            } => "B6",
            Self::FpExp0071 {
                jumper_0: true,
                jumper_1: true,
            } => "B7",
            Self::FpExp0081 {
                jumper_0: false,
                jumper_1: false,
            } => "84",
            Self::FpExp0081 {
                jumper_0: true,
                jumper_1: false,
            } => "85",
            Self::FpExp0081 {
                jumper_0: false,
                jumper_1: true,
            } => "86",
            Self::FpExp0081 {
                jumper_0: true,
                jumper_1: true,
            } => "87",
            Self::FpExp0091 {
                jumper_0: false,
                jumper_1: false,
            } => "88",
            Self::FpExp0091 {
                jumper_0: true,
                jumper_1: false,
            } => "89",
            Self::FpExp0091 {
                jumper_0: false,
                jumper_1: true,
            } => "8A",
            Self::FpExp0091 {
                jumper_0: true,
                jumper_1: true,
            } => "8B",
        }
    }
}

impl std::default::Default for ExpansionBoard {
    fn default() -> Self {
        ExpansionBoard::Neutron
    }
}

impl fmt::Display for ExpansionBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<String> for ExpansionBoard {
    fn into(self) -> String {
        self.to_string()
    }
}
