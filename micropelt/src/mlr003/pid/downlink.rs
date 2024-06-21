use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{
    float_point_one_to_bin, float_point_two_to_bin, float_point_zero_two_to_bin, percent_to_bin,
};
use crate::{lorawan, PortPayload};

use super::super::port::Port;

use super::IntegralUnwind;

const DOWNLINK_N_BYTES: usize = 6;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.1)]
    pub k_p: f32,
    #[partial_close(resolution = 0.02)]
    pub k_i: f32,
    #[partial_close(resolution = 0.2)]
    pub k_d: f32,
    pub integral_unwind: IntegralUnwind,
    pub closed_percent: u8,
    #[partial_close(resolution = 0.2)]
    pub k_d_when_closed: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "K P {:.1}\n\
        K I {:.2}\n\
        K D {:.1}\n\
        Integral Unwind: {}\n\
        Close to {}%\n\
        When Closed, K D {:.1}",
            self.k_p,
            self.k_i,
            self.k_d,
            self.integral_unwind,
            self.closed_percent,
            self.k_d_when_closed
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
        Self {
            k_p: 2.4,
            k_i: 0.06,
            k_d: 22.2,
            integral_unwind: IntegralUnwind::Zero,
            closed_percent: 0,
            k_d_when_closed: 0.0,
        }
    }

    pub fn default_domestic_hot_water() -> Self {
        Self {
            k_p: 4.0,
            k_i: 0.0,
            k_d: 0.0,
            integral_unwind: IntegralUnwind::Zero,
            closed_percent: 0,
            k_d_when_closed: 0.0,
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_point_one_to_bin(self.k_p)?;
        payload[1] = float_point_zero_two_to_bin(self.k_i)?;
        payload[2] = float_point_two_to_bin(self.k_d)?;
        payload[3] = self.integral_unwind.to_bin();
        payload[4] = percent_to_bin(self.closed_percent)?;
        payload[5] = float_point_two_to_bin(self.k_d_when_closed)?;

        Ok(PortPayload {
            port: Port::Pid as u8,
            payload,
        })
    }
}

DownlinkStatus! {Pid, "PID Coefficients"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
