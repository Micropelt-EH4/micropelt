use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{
    bin_to_float_point_two, bin_to_float_point_zero, bin_to_float_point_zero_two, bin_to_percent,
    check_payload_length,
};
const UPLINK_N_BYTES: usize = 7;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.1)]
    k_p: f32,
    #[partial_close(resolution = 0.02)]
    k_i: f32,
    #[partial_close(resolution = 0.2)]
    k_d: f32,
    closed_percent: u8,
    #[partial_close(resolution = 0.2)]
    k_d_when_closed: f32,
    offset_percent: u8,
    pid_inverse: bool,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        let uplink = Self {
            k_p: bin_to_float_point_zero(input[0]),
            k_i: bin_to_float_point_zero_two(input[1]),
            k_d: bin_to_float_point_two(input[2]),
            pid_inverse: if((input[3] >> 6) & 0x01) == 0 {true} else {false},
            closed_percent: bin_to_percent(input[4])?,
            k_d_when_closed: bin_to_float_point_two(input[5]),
            offset_percent: bin_to_percent(input[6])?,
        };

        Ok(uplink)
    }

    pub fn k_p(&self) -> f32 {
        self.k_p
    }

    pub fn k_i(&self) -> f32 {
        self.k_i
    }

    pub fn k_d(&self) -> f32 {
        self.k_d
    }

    pub fn pid_inverse(&self) -> bool {
        self.pid_inverse
    }

    pub fn closed_percent(&self) -> u8 {
        self.closed_percent
    }

    pub fn k_d_when_closed(&self) -> f32 {
        self.k_d_when_closed
    }

    pub fn offset_percent(&self) -> u8 {
        self.offset_percent
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
