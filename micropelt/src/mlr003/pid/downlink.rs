use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::{
    float_point_two_to_bin, float_point_zero_two_to_bin, percent_to_bin, p_max_to_bin
};
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 7;

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink {
    pub k_p: u8,
    #[partial_close(resolution = 0.02)]
    pub k_i: f32,
    #[partial_close(resolution = 0.2)]
    pub k_d: f32,
    pub closed_percent: u8,
    #[partial_close(resolution = 0.2)]
    pub k_d_when_closed: f32,
    pub offset_percent: u8,
    pub pid_inverse: bool,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "K P {:.1}\n\
        K I {:.2}\n\
        K D {:.1}\n\
        Close to {}%\n\
        When Closed, K D {:.1}\n\
        Offset to {}%\n\
        Inverse PID {}",
            self.k_p,
            self.k_i,
            self.k_d,
            self.closed_percent,
            self.k_d_when_closed,
            self.offset_percent,
            self.pid_inverse,
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
            k_p: 20,
            k_i: 1.5,
            k_d: 0.0,
            closed_percent: 100,
            k_d_when_closed: 0.0,
            offset_percent: 0,
            pid_inverse: true,
        }
    }

    pub fn default_domestic_hot_water() -> Self {
        Self {
            k_p: 4,
            k_i: 0.0,
            k_d: 0.0,
            closed_percent: 0,
            k_d_when_closed: 0.0,
            offset_percent: 0,
            pid_inverse: false,
        }
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = p_max_to_bin(self.k_p)?;
        payload[1] = float_point_zero_two_to_bin(self.k_i)?;
        payload[2] = float_point_two_to_bin(self.k_d)?;
        payload[3] = if self.pid_inverse == true {128} else {192};
        payload[4] = percent_to_bin(self.closed_percent)?;
        payload[5] = float_point_two_to_bin(self.k_d_when_closed)?;
        payload[6] = percent_to_bin(self.offset_percent)?;

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
