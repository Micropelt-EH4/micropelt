use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::common::float_point_two_five_to_bin_complement_of_two;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.25)]
    pub flow_offset: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Flow Temperature = Flow Raw Moving Average {:+}",
            self.flow_offset,
        )
    }
}

impl PartialEq for Downlink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Default for Downlink {
    fn default() -> Self {
        Self { flow_offset: 0.0 }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_point_two_five_to_bin_complement_of_two(self.flow_offset)?;

        Ok(PortPayload {
            port: Port::TemperatureEstimate as u8,
            payload,
        })
    }
}

DownlinkStatus! {TemperatureEstimate, "Temperature Estimation Coefficients"}
