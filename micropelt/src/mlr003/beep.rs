use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::port::Port;

const DOWNLINK_N_BYTES: usize = 1;

pub struct Downlink {
    pub n_beeps: u8,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Beep {} times", self.n_beeps)
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.n_beeps;

        Ok(PortPayload {
            port: Port::Beep as u8,
            payload,
        })
    }
}
