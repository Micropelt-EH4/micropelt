use std::io::{Error, ErrorKind, Result};

use super::DataRate;

const UPLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    data_rate: DataRate,
}

impl Uplink {
    pub(crate) fn deserialize(input: &[u8]) -> Result<Self> {
        if input.len() != UPLINK_N_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{:?} is of length {}", input, input.len()),
            ));
        }

        let data_rate = DataRate::from_bin(input[0])?;

        Ok(Self { data_rate })
    }

    pub fn data_rate(&self) -> &DataRate {
        &self.data_rate
    }
}
