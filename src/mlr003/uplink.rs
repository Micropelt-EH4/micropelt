use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::operate;
use super::utils::Port;

#[derive(Clone, Debug, PartialEq)]
pub enum Uplink {
    Operate(operate::Uplink),
}

impl lorawan::Uplink for Uplink {
    fn deserialize(input: &PortPayload) -> Result<Self> {
        if input.port == Port::Operate as u8 {
            Ok(Self::Operate(operate::Uplink::deserialize(&input.payload)?))
        } else {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("fport {} is not yet used", input.port),
            ))
        }
    }
}
