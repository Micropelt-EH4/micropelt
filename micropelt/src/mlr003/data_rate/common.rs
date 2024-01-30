use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataRate {
    Sf7Bw125,
    Sf8Bw125,
}

impl fmt::Display for DataRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sf7Bw125 => write!(f, "SF7BW125"),
            Self::Sf8Bw125 => write!(f, "SF8BW125"),
        }
    }
}

impl DataRate {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Sf7Bw125 => 0,
            Self::Sf8Bw125 => 1,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::Sf7Bw125),
            1 => Ok(Self::Sf8Bw125),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0 or 1, got {input}"),
            )),
        }
    }
}

#[cfg(test)]
#[path = "./test_common.rs"]
mod test_common;
