use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataRate {
    Sf7Bw125,
    Sf8Bw125,
    Sf9Bw125,
    Sf10Bw125,
    Sf11Bw125,
    Sf12Bw125,
}

impl fmt::Display for DataRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sf7Bw125 => write!(f, "SF7BW125"),
            Self::Sf8Bw125 => write!(f, "SF8BW125"),
            Self::Sf9Bw125 => write!(f, "SF9BW125"),
            Self::Sf10Bw125 => write!(f, "SF10BW125"),
            Self::Sf11Bw125 => write!(f, "SF11BW125"),
            Self::Sf12Bw125 => write!(f, "SF12BW125"),
        }
    }
}

impl DataRate {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Sf7Bw125 => 0,
            Self::Sf8Bw125 => 1,
            Self::Sf9Bw125 => 2,
            Self::Sf10Bw125 => 3,
            Self::Sf11Bw125 => 4,
            Self::Sf12Bw125 => 5,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::Sf7Bw125),
            1 => Ok(Self::Sf8Bw125),
            2 => Ok(Self::Sf9Bw125),
            3 => Ok(Self::Sf10Bw125),
            4 => Ok(Self::Sf11Bw125),
            5 => Ok(Self::Sf12Bw125),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0 to 5, got {input}"),
            )),
        }
    }
}

#[cfg(test)]
#[path = "./test_common.rs"]
mod test_common;
