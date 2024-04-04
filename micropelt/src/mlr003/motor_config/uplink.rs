use std::io::Result;

use crate::utils::check_payload_length;

const UPLINK_N_BYTES: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uplink {
    irun_to_mounting: u8,
    irun_to_0: u8,
    sgthrs: u8,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            irun_to_mounting: input[0],
            irun_to_0: input[1],
            sgthrs: input[2],
        })
    }

    pub fn irun_to_mounting(&self) -> u8 {
        self.irun_to_mounting
    }

    pub fn irun_to_0(&self) -> u8 {
        self.irun_to_0
    }

    pub fn sgthrs(&self) -> u8 {
        self.sgthrs
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
