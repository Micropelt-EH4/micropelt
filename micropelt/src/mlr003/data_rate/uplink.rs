use std::io::Result;

use crate::utils::check_payload_length;

use super::DataRate;

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uplink {
    data_rate: DataRate,
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        let data_rate = DataRate::from_bin(input[0])?;

        Ok(Self { data_rate })
    }

    pub fn data_rate(&self) -> &DataRate {
        &self.data_rate
    }
}
