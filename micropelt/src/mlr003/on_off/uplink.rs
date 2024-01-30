use std::io::Result;

use crate::utils::{bin_to_bool, check_payload_length};

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    stays_on_regardless_of_calibration: bool,
    done_recalibration_now: bool,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            stays_on_regardless_of_calibration: bin_to_bool((input[0] >> 7) & 0b1)?,
            done_recalibration_now: bin_to_bool((input[0] >> 6) & 0b1)?,
        })
    }

    pub fn stays_on_regardless_of_calibration(&self) -> bool {
        self.stays_on_regardless_of_calibration
    }

    pub fn done_recalibration_now(&self) -> bool {
        self.done_recalibration_now
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
