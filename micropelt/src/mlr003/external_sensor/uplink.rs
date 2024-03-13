use std::fmt;
use std::io::{Error, ErrorKind, Result};

use crate::utils::check_payload_length;

use super::Expiry;

const UPLINK_N_BYTES: usize = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    expiry: Expiry,
    temperature_source: TemperatureSource,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TemperatureSource {
    InternalAlgorithm,
    ExternalSensor,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            expiry: Expiry::from_bin(input[0]),
            temperature_source: TemperatureSource::from_bin(input[1])?,
        })
    }

    pub fn expiry(&self) -> &Expiry {
        &self.expiry
    }

    pub fn temperature_source(&self) -> &TemperatureSource {
        &self.temperature_source
    }
}

impl fmt::Display for TemperatureSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InternalAlgorithm => write!(f, "Internal Algorithm"),
            Self::ExternalSensor => write!(f, "External Sensor"),
        }
    }
}

impl TemperatureSource {
    fn from_bin(input: u8) -> Result<Self> {
        match input {
            0 => Ok(Self::InternalAlgorithm),
            1 => Ok(Self::ExternalSensor),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Expected 0 or 1, got {input}"),
            )),
        }
    }
}
