use std::io::Result;

use crate::utils::{bin_to_bool, check_payload_length};

use super::{Action, Period};

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uplink {
    action: Action,
    beep: bool,
    period: Period,
    temperature_drop_detected: bool,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            action: Action::from_bin((input[0] >> 6) & 0b11)?,
            beep: bin_to_bool((input[0] >> 5) & 1)?,
            period: Period::from_bin((input[0] >> 4) & 1)?,
            temperature_drop_detected: bin_to_bool(input[0] & 1)?,
        })
    }

    pub fn action(&self) -> &Action {
        &self.action
    }

    pub fn beep(&self) -> bool {
        self.beep
    }

    pub fn period(&self) -> &Period {
        &self.period
    }

    pub fn temperature_drop_detected(&self) -> bool {
        self.temperature_drop_detected
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
