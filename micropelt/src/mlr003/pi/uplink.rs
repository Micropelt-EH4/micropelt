use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::{bin_to_float_point_one, bin_to_float_point_zero_two};

use super::IntegralUnwind;

const UPLINK_N_BYTES_REV_2_3: usize = 2;
const UPLINK_N_BYTES_REV_2_9: usize = 4;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.1)]
    k_p: f32,
    #[partial_close(resolution = 0.02)]
    k_i: f32,
    integral_unwind: IntegralUnwind,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        let integral_unwind = match input.len() {
            UPLINK_N_BYTES_REV_2_3 => IntegralUnwind::BackCalculate,
            UPLINK_N_BYTES_REV_2_9 => IntegralUnwind::from_bin(input[3])?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Payload length should be {UPLINK_N_BYTES_REV_2_3} or {UPLINK_N_BYTES_REV_2_9}, \
                    got {input:?} which is of length {}", input.len()
                    ),
                ))
            }
        };

        Ok(Self {
            k_p: bin_to_float_point_one(input[0]),
            k_i: bin_to_float_point_zero_two(input[1]),
            integral_unwind,
        })
    }

    pub fn k_p(&self) -> f32 {
        self.k_p
    }

    pub fn k_i(&self) -> f32 {
        self.k_i
    }

    pub fn integral_unwind(&self) -> IntegralUnwind {
        self.integral_unwind
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
