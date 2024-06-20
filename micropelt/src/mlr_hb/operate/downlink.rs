use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::radio_communication_interval::RadioCommunicationInterval;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Downlink {
    pub radio_communication_interval: RadioCommunicationInterval,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Radio Communication Interval {}",
            self.radio_communication_interval,
        )
    }
}

impl Default for Downlink {
    fn default() -> Self {
        Self {
            radio_communication_interval: RadioCommunicationInterval::Minutes5,
        }
    }
}

impl lorawan::Downlink for Downlink {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.radio_communication_interval.to_bin();

        Ok(PortPayload {
            port: Port::Operate as u8,
            payload,
        })
    }
}
