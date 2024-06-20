use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::super::{temperature_estimate, version};

use super::operate;

#[derive(Debug)]
pub enum Uplink {
    Operate(operate::Uplink),
    Version(version::Uplink),
    TemperatureEstimate(temperature_estimate::Uplink),
}

impl lorawan::Uplink for Uplink {
    fn deserialise(input: &PortPayload) -> Result<Self> {
        if input.port == Port::Operate as u8 {
            Ok(Self::Operate(operate::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::Version as u8 {
            Ok(Self::Version(version::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::TemperatureEstimate as u8 {
            Ok(Self::TemperatureEstimate(
                temperature_estimate::Uplink::deserialise(&input.payload)?,
            ))
        } else {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("fport {} is not yet used", input.port),
            ))
        }
    }
}
