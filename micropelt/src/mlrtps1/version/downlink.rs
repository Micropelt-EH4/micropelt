use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::port::Port;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Downlink {}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Get Version")
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        Ok(PortPayload {
            port: Port::Version as u8,
            payload: Vec::with_capacity(0),
        })
    }
}
