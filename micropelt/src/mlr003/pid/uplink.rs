use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::{
    bin_to_float_point_one, bin_to_float_point_two, bin_to_float_point_zero_two, bin_to_percent,
};

use super::IntegralUnwind;

const UPLINK_N_BYTES_REV_2_3: usize = 2;
const UPLINK_N_BYTES_REV_2_9: usize = 4;
const UPLINK_N_BYTES_REV_2_A: usize = 6;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.1)]
    k_p: f32,
    #[partial_close(resolution = 0.02)]
    k_i: f32,
    #[partial_close(resolution = 0.2)]
    k_d: f32,
    integral_unwind: IntegralUnwind,
    closed_percent: u8,
    #[partial_close(resolution = 0.2)]
    k_d_when_closed: f32,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        let k_d;
        let integral_unwind;
        let closed_percent;
        let k_d_when_closed;

        match input.len() {
            UPLINK_N_BYTES_REV_2_3 => {
                k_d = 0.0;
                integral_unwind = IntegralUnwind::BackCalculate;
                closed_percent = 0;
                k_d_when_closed = 0.0;
            },
            UPLINK_N_BYTES_REV_2_9 => {
                k_d = 0.0;
                integral_unwind = IntegralUnwind::from_bin(input[3])?;
                closed_percent = 0;
                k_d_when_closed = 0.0;
            },
            UPLINK_N_BYTES_REV_2_A => {
                k_d = bin_to_float_point_two(input[2]);
                integral_unwind = IntegralUnwind::from_bin(input[3])?;
                closed_percent = bin_to_percent(input[4])?;
                k_d_when_closed = bin_to_float_point_two(input[5]);
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Payload length should be {UPLINK_N_BYTES_REV_2_3} or {UPLINK_N_BYTES_REV_2_9} or {UPLINK_N_BYTES_REV_2_A}, \
                    got {input:?} which is of length {}", input.len()
                    ),
                ))
            }
        };

        Ok(Self {
            k_p: bin_to_float_point_one(input[0]),
            k_i: bin_to_float_point_zero_two(input[1]),
            k_d,
            integral_unwind,
            closed_percent,
            k_d_when_closed,
        })
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

    pub fn integral_unwind(&self) -> IntegralUnwind {
        self.integral_unwind
    }

    pub fn closed_percent(&self) -> u8 {
        self.closed_percent
    }

    pub fn k_d_when_closed(&self) -> f32 {
        self.k_d_when_closed
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
