use std::io::{Error, ErrorKind, Result};

use super::TravelDistance;

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    travel_distance: TravelDistance,
}

impl Uplink {
    pub(crate) fn deserialize(input: &[u8]) -> Result<Self> {
        if input.len() != UPLINK_N_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{:?} is of length {}", input, input.len()),
            ));
        }

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
