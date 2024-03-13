use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Period {
    #[default]
    One,
    Two,
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Activation after at least {}",
            match self {
                Self::One => "one temperature drop",
                Self::Two => "two consecutive temperature drops",
            }
        )
    }
}

impl Period {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::One => 0,
            Self::Two => 1,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0, 1, 2, or 3, got {input}"),
            )),
        }
    }
}
