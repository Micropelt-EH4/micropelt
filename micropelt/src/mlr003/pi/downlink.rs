use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{float_point_one_to_bin, float_point_zero_two_to_bin};
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 2;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.1)]
    pub k_p: f32,
    #[partial_close(resolution = 0.02)]
    pub k_i: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "K P {:.1}\n\
        K I {:.2}",
            self.k_p, self.k_i
        )
    }
}

impl PartialEq for Downlink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Downlink {
    pub fn default_radiator() -> Self {
        Self { k_p: 3.6, k_i: 0.4 }
    }

    pub fn default_domestic_hot_water() -> Self {
        Self { k_p: 4.0, k_i: 0.0 }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_point_one_to_bin(self.k_p)?;
        payload[1] = float_point_zero_two_to_bin(self.k_i)?;

        Ok(PortPayload {
            port: Port::Pi as u8,
            payload,
        })
    }
}

DownlinkStatus! {Pi, "PI Coefficients"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
