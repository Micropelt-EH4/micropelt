use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::check_payload_length;

use super::common::bin_to_float_point_two_five_complement_of_two;

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.25)]
    flow_offset: f32,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            flow_offset: bin_to_float_point_two_five_complement_of_two(input[0]),
        })
    }

    pub fn flow_offset(&self) -> f32 {
        self.flow_offset
    }
}

#[cfg(test)]
#[path = "./test_downlink_uplink.rs"]
mod test_downlink_uplink;
