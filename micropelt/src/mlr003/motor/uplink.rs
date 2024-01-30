use std::io::Result;

use crate::utils::check_payload_length;

use super::TravelDistance;

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uplink {
    travel_distance: TravelDistance,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        let travel_distance = TravelDistance::from_bin(input[0])?;

        Ok(Self { travel_distance })
    }

    pub fn travel_distance(&self) -> &TravelDistance {
        &self.travel_distance
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
