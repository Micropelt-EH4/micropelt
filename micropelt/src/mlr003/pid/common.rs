use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntegralUnwind {
    Zero,
    BackCalculate,
}

impl fmt::Display for IntegralUnwind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "Set error integral to 0"),
            Self::BackCalculate => write!(f, "Back-Calculate"),
        }
    }
}

impl IntegralUnwind {
    pub(super) fn to_bin(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::BackCalculate => 1,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::Zero),
            1 => Ok(Self::BackCalculate),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0 or 1, got {input}"),
            )),
        }
    }
}
