use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{check_range, float_point_zero_one_to_bin};
use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::common::float_point_two_five_to_bin_complement_of_two;

const DOWNLINK_N_BYTES: usize = 8;

pub const MAX_VALUES: u8 = 20;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.25)]
    pub flow_add: f32,
    #[partial_close(resolution = 0.01)]
    pub flow_multiply: f32,
    pub n_delay_flow: u8,
    pub n_moving_average_flow: u8,
    pub n_moving_average_ambient: u8,
    pub n_moving_average_all: u8,
    #[partial_close(resolution = 0.01)]
    pub ratio_offset: f32,
    #[partial_close(resolution = 0.01)]
    pub power: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Temperature Estimation:\n\
            Flow Temperature = {:.2} * Flow Raw {:+}\n\
            Skip {} most recent Flow Raw Temperature values\n\
            {} values for Flow Moving Average\n\
            {} values for Ambient Moving Average \n\
            {} values for Estimate Moving Average \n\
            Ratio Offset {:.2}\n\
            Ratio Power {:.2}",
            self.flow_multiply,
            self.flow_add,
            self.n_delay_flow,
            self.n_moving_average_flow,
            self.n_moving_average_ambient,
            self.n_moving_average_all,
            self.ratio_offset,
            self.power,
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
            flow_add: 0.0,
            flow_multiply: 1.0,
            n_delay_flow: 0,
            n_moving_average_flow: 6,
            n_moving_average_ambient: 1,
            n_moving_average_all: 1,
            ratio_offset: 0.0,
            power: 1.2,
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_point_two_five_to_bin_complement_of_two(self.flow_add)?;
        payload[1] = float_point_zero_one_to_bin(self.flow_multiply)?;
        payload[2] = n_values_to_bin(self.n_delay_flow)?;
        payload[3] = n_values_to_bin(self.n_moving_average_flow)?;
        payload[4] = n_values_to_bin(self.n_moving_average_ambient)?;
        payload[5] = n_values_to_bin(self.n_moving_average_all)?;
        payload[6] = float_point_zero_one_to_bin(self.ratio_offset)?;
        payload[7] = float_point_zero_one_to_bin(self.power)?;

        Ok(PortPayload {
            port: Port::TemperatureEstimate as u8,
            payload,
        })
    }
}

fn n_values_to_bin(input: u8) -> Result<u8> {
    check_range(0.0, input as f32, MAX_VALUES as f32, 1.0)?;

    Ok(input)
}

DownlinkStatus! {TemperatureEstimate, "Temperature Estimation Coefficients"}
