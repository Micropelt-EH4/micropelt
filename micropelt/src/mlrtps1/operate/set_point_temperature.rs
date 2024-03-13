use std::fmt;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum SetPointTemperature {
    FreezeProtect,
    Offset(i8),
}

impl fmt::Display for SetPointTemperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FreezeProtect => write!(f, "FP"),
            Self::Offset(i) => {
                if *i != 0 {
                    write!(f, "{i:+}")
                } else {
                    write!(f, " {i}")
                }
            }
        }
    }
}

impl SetPointTemperature {
    pub(super) fn from_bin(input: u8) -> Result<Self> {
        match input {
            0x00 => Ok(SetPointTemperature::Offset(0)),
            0x01 => Ok(SetPointTemperature::Offset(1)),
            0x02 => Ok(SetPointTemperature::Offset(2)),
            0x03 => Ok(SetPointTemperature::Offset(3)),
            0x04 => Ok(SetPointTemperature::Offset(4)),
            0x05 => Ok(SetPointTemperature::Offset(5)),
            0x0C => Ok(SetPointTemperature::Offset(-4)),
            0x0D => Ok(SetPointTemperature::Offset(-3)),
            0x0E => Ok(SetPointTemperature::Offset(-2)),
            0x0F => Ok(SetPointTemperature::Offset(-1)),
            0xFF => Ok(SetPointTemperature::FreezeProtect),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unexpected input for Set Point Temperature, got {input}"),
            )),
        }
    }
}

#[cfg(test)]
#[path = "./test_set_point_temperature.rs"]
mod test_set_point_temperature;
