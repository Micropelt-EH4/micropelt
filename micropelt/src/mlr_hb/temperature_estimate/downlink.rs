use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{check_range, float_point_zero_one_to_bin};
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 3;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.00001)]
    pub p2: f32,
    #[partial_close(resolution = 0.01)]
    pub p1: f32,
    #[partial_close(resolution = 0.01)]
    pub p0: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Temperature = {:.5} * Raw² + {:.2} * Raw {:+.2}",
            self.p2, self.p1, self.p0
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
        Self {
            p2: 0.00123,
            p1: 0.93,
            p0: 0.68,
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_plus_minus_point_zero_one_to_bin(self.p0)?;
        payload[1] = float_point_zero_one_to_bin(self.p1)?;
        payload[2] = float_point_zero_zero_zero_zero_one_to_bin(self.p2)?;

        Ok(PortPayload {
            port: Port::TemperatureEstimate as u8,
            payload,
        })
    }
}

pub fn float_plus_minus_point_zero_one_to_bin(input: f32) -> Result<u8> {
    check_range(-1.28, input, 1.27, 0.01)?;

    if input >= 0.0 {
        Ok((input * 100.0) as u8)
    } else {
        Ok(((input.abs() * 100.0) as u8 ^ 0b11111111) + 1)
    }
}

fn float_point_zero_zero_zero_zero_one_to_bin(input: f32) -> Result<u8> {
    check_range(0.0, input, 0.00255, 0.00001)?;

    Ok((input * 100000.0) as u8)
}

DownlinkStatus! {TemperatureEstimate, "Temperature Estimate Coefficients"}
