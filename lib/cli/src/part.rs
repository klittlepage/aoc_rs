use std::fmt::Display;

use clap::ValueEnum;

#[derive(ValueEnum, Copy, Clone, Debug, Default)]
pub enum Part {
    #[default]
    P1,
    P2,
}

impl Part {
    #[must_use]
    pub fn value(&self) -> u8 {
        match self {
            Self::P1 => 1,
            Self::P2 => 2,
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P1 => write!(f, "P1"),
            Self::P2 => write!(f, "P2"),
        }
    }
}
