use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::utils::Port;
use super::{data_rate, motor, operate, version};

#[derive(Clone, Debug, PartialEq)]
pub enum Uplink {
    Operate(operate::Uplink),
    Version(version::Uplink),
    DataRate(data_rate::Uplink),
    Motor(motor::Uplink),
}

impl lorawan::Uplink for Uplink {
    fn deserialize(input: &PortPayload) -> Result<Self> {
        if input.port == Port::Operate as u8 {
            Ok(Self::Operate(operate::Uplink::deserialize(&input.payload)?))
        } else if input.port == Port::Version as u8 {
            Ok(Self::Version(version::Uplink::deserialize(&input.payload)?))
        } else if input.port == Port::Motor as u8 {
            Ok(Self::Motor(motor::Uplink::deserialize(&input.payload)?))
        } else if input.port == Port::DataRate as u8 {
            Ok(Self::DataRate(data_rate::Uplink::deserialize(
                &input.payload,
            )?))
        } else {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("fport {} is not yet used", input.port),
            ))
        }
    }
}
