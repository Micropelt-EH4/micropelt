use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::TravelDistance;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Downlink {
    pub travel_distance: TravelDistance,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Set Motor Travel Distance to {}", self.travel_distance)
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = (1 << 7) | self.travel_distance.to_bin();

        Ok(PortPayload {
            port: Port::Motor as u8,
            payload,
        })
    }
}

DownlinkStatus! {Motor, "Motor Travel Distance"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
