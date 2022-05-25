use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::utils::Port;
use super::TravelDistance;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Downlink {
    pub travel_distance: Option<TravelDistance>,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.travel_distance {
            Some(td) => write!(f, "Set Motor Travel Distance to {}", td),
            None => write!(f, "Get Motor Travel Distance"),
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialize(&self) -> Result<PortPayload> {
        let payload = match &self.travel_distance {
            Some(td) => vec![(1 << 7) | td.to_bin()],
            None => Vec::with_capacity(0),
        };

        Ok(PortPayload {
            port: Port::Motor as u8,
            payload,
        })
    }
}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
