use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{bin_to_float_point_one, bin_to_float_point_zero_two, check_payload_length};

const UPLINK_N_BYTES: usize = 2;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.1)]
    k_p: f32,
    #[partial_close(resolution = 0.02)]
    k_i: f32,
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
            k_p: bin_to_float_point_one(input[0]),
            k_i: bin_to_float_point_zero_two(input[1]),
        })
    }

    pub fn k_p(&self) -> f32 {
        self.k_p
    }

    pub fn k_i(&self) -> f32 {
        self.k_i
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
