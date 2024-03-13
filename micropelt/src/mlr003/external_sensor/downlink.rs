use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::Expiry;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Downlink {
    pub expiry: Expiry,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "If a Room Temperature is sent by Downlink, continue using it \
            until a new Room Temperature is sent by Downlink \
            or {}",
            self.expiry
        )
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.expiry.to_bin();

        Ok(PortPayload {
            port: Port::ExternalSensor as u8,
            payload,
        })
    }
}

DownlinkStatus! {ExternalSensor, "External Sensor Expiry"}
