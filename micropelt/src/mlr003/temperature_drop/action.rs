use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Action {
    #[default]
    None,
    CloseFor30MinutesThenOperate,
    CloseFor60MinutesThenOperate,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Upon detection of a temperature drop, {}",
            match self {
                Self::None => "take no action",
                Self::CloseFor30MinutesThenOperate =>
                    "close to 0% for 30 minutes and then resume normal operation",
                Self::CloseFor60MinutesThenOperate =>
                    "close to 0% for 60 minutes and then resume normal operation",
            }
        )
    }
}

impl Action {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::CloseFor30MinutesThenOperate => 2,
            Self::CloseFor60MinutesThenOperate => 3,
        }
    }

    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::None),
            2 => Ok(Self::CloseFor30MinutesThenOperate),
            3 => Ok(Self::CloseFor60MinutesThenOperate),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0, 2, or 3, got {input}"),
            )),
        }
    }
}
