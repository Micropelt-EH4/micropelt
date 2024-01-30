use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::DataRate;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Downlink {
    pub data_rate: DataRate,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set Data Rate to {}", self.data_rate)
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.data_rate.to_bin();

        Ok(PortPayload {
            port: Port::DataRate as u8,
            payload,
        })
    }
}

DownlinkStatus! {DataRate, "Data Rate"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
