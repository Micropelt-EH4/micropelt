use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{bin_to_float_point_zero_one, check_payload_length};

use super::common::bin_to_float_point_two_five_complement_of_two;

const UPLINK_N_BYTES: usize = 8;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.25)]
    flow_add: f32,
    #[partial_close(resolution = 0.01)]
    flow_multiply: f32,
    n_delay_flow: u8,
    n_moving_average_flow: u8,
    n_moving_average_ambient: u8,
    n_moving_average_all: u8,
    #[partial_close(resolution = 0.01)]
    ratio_offset: f32,
    #[partial_close(resolution = 0.01)]
    power: f32,
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
            flow_add: bin_to_float_point_two_five_complement_of_two(input[0]),
            flow_multiply: bin_to_float_point_zero_one(input[1]),
            n_delay_flow: input[2],
            n_moving_average_flow: input[3],
            n_moving_average_ambient: input[4],
            n_moving_average_all: input[5],
            ratio_offset: bin_to_float_point_zero_one(input[6]),
            power: bin_to_float_point_zero_one(input[7]),
        })
    }

    pub fn flow_add(&self) -> f32 {
        self.flow_add
    }

    pub fn flow_multiply(&self) -> f32 {
        self.flow_multiply
    }

    pub fn n_delay_flow(&self) -> u8 {
        self.n_delay_flow
    }

    pub fn n_moving_average_flow(&self) -> u8 {
        self.n_moving_average_flow
    }

    pub fn n_moving_average_ambient(&self) -> u8 {
        self.n_moving_average_ambient
    }

    pub fn n_moving_average_all(&self) -> u8 {
        self.n_moving_average_all
    }

    pub fn ratio_offset(&self) -> f32 {
        self.ratio_offset
    }

    pub fn power(&self) -> f32 {
        self.power
    }
}

#[cfg(test)]
#[path = "./test_downlink_uplink.rs"]
mod test_downlink_uplink;
