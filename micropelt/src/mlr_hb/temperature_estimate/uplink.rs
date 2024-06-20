use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{bin_to_float_point_zero_one, check_payload_length};

const UPLINK_N_BYTES: usize = 3;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.00001)]
    p2: f32,
    #[partial_close(resolution = 0.01)]
    p1: f32,
    #[partial_close(resolution = 0.01)]
    p0: f32,
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
            p0: bin_to_float_plus_minus_point_zero_one(input[0]),
            p1: bin_to_float_point_zero_one(input[1]),
            p2: bin_to_float_point_zero_zero_zero_zero_one(input[2]),
        })
    }

    pub fn p2(&self) -> f32 {
        self.p2
    }

    pub fn p1(&self) -> f32 {
        self.p1
    }

    pub fn p0(&self) -> f32 {
        self.p0
    }
}

fn bin_to_float_plus_minus_point_zero_one(input: u8) -> f32 {
    if input > 127 {
        (input as i16 - 256) as f32 * 0.01
    } else {
        input as f32 * 0.01
    }
}

fn bin_to_float_point_zero_zero_zero_zero_one(input: u8) -> f32 {
    input as f32 * 0.00001
}

#[cfg(test)]
#[path = "./test_downlink_uplink.rs"]
mod test_downlink_uplink;
