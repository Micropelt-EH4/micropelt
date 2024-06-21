use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TravelDistance {
    Um0624,
    Um0832,
    Um1040,
    Um1248,
    Um1456,
    Um1664,
    Um1872,
    Um2080,
    Um2288,
    Um2496,
    Um2560Point48,
}

impl fmt::Display for TravelDistance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Um0624 => write!(f, "0.624mm"),
            Self::Um0832 => write!(f, "0.832mm"),
            Self::Um1040 => write!(f, "1.040mm"),
            Self::Um1248 => write!(f, "1.248mm"),
            Self::Um1456 => write!(f, "1.456mm"),
            Self::Um1664 => write!(f, "1.664mm"),
            Self::Um1872 => write!(f, "1.872mm"),
            Self::Um2080 => write!(f, "2.080mm"),
            Self::Um2288 => write!(f, "2.288mm"),
            Self::Um2496 => write!(f, "2.496mm"),
            Self::Um2560Point48 => write!(f, "2.56048mm"),
        }
    }
}

impl TravelDistance {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Um0624 => 3,
            Self::Um0832 => 4,
            Self::Um1040 => 5,
            Self::Um1248 => 6,
            Self::Um1456 => 7,
            Self::Um1664 => 8,
            Self::Um1872 => 9,
            Self::Um2080 => 10,
            Self::Um2288 => 11,
            Self::Um2496 => 12,
            Self::Um2560Point48 => 0,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::Um2560Point48),
            3 => Ok(Self::Um0624),
            4 => Ok(Self::Um0832),
            5 => Ok(Self::Um1040),
            6 => Ok(Self::Um1248),
            7 => Ok(Self::Um1456),
            8 => Ok(Self::Um1664),
            9 => Ok(Self::Um1872),
            10 => Ok(Self::Um2080),
            11 => Ok(Self::Um2288),
            12 => Ok(Self::Um2496),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{input} does not map to a Motor Travel Distance"),
            )),
        }
    }
}

#[cfg(test)]
#[path = "./test_travel_distance.rs"]
mod test_travel_distance;
