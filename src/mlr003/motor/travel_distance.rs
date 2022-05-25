use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum TravelDistance {
    Nm1456,
    Nm1664,
    Nm1872,
    Nm2080,
    Nm2288,
    Nm2496,
    Nm2560Point48,
}

impl fmt::Display for TravelDistance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Nm1456 => write!(f, "1.456mm"),
            Self::Nm1664 => write!(f, "1.664mm"),
            Self::Nm1872 => write!(f, "1.872mm"),
            Self::Nm2080 => write!(f, "2.080mm"),
            Self::Nm2288 => write!(f, "2.288mm"),
            Self::Nm2496 => write!(f, "2.496mm"),
            Self::Nm2560Point48 => write!(f, "2.56048mm"),
        }
    }
}

impl TravelDistance {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Nm1456 => 7,
            Self::Nm1664 => 8,
            Self::Nm1872 => 9,
            Self::Nm2080 => 10,
            Self::Nm2288 => 11,
            Self::Nm2496 => 12,
            Self::Nm2560Point48 => 0,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::Nm2560Point48),
            7 => Ok(Self::Nm1456),
            8 => Ok(Self::Nm1664),
            9 => Ok(Self::Nm1872),
            10 => Ok(Self::Nm2080),
            11 => Ok(Self::Nm2288),
            12 => Ok(Self::Nm2496),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{} does not map to a Motor Travel Distance", input),
            )),
        }
    }
}

#[cfg(test)]
#[path = "./test_travel_distance.rs"]
mod test_travel_distance;
