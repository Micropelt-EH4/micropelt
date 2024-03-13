use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::port::Port;
use super::{operate, version};

#[derive(Debug)]
pub enum Uplink {
    Operate(operate::Uplink),
    Version(version::Uplink),
}

impl lorawan::Uplink for Uplink {
    fn deserialise(input: &PortPayload) -> Result<Self> {
        if input.port == Port::Operate as u8 {
            Ok(Self::Operate(operate::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::Version as u8 {
            Ok(Self::Version(version::Uplink::deserialise(&input.payload)?))
        } else {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("fport {} is not yet used", input.port),
            ))
        }
    }
}
