use std::fmt;
use std::io::Result;

use crate::{lorawan, PortPayload};

use super::super::utils::Port;

use super::DataRate;

#[derive(Clone, Debug, PartialEq)]
pub struct Downlink {
    pub data_rate: Option<DataRate>,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.data_rate {
            Some(dr) => write!(f, "Set Data Rate to {}", dr),
            None => write!(f, "Get Data Rate"),
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialize(&self) -> Result<PortPayload> {
        let payload = match &self.data_rate {
            Some(dr) => vec![dr.to_bin()],
            None => Vec::with_capacity(0),
        };

        Ok(PortPayload {
            port: Port::DataRate as u8,
            payload,
        })
    }
}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
