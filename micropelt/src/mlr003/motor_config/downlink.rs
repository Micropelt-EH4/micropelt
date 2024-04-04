use std::fmt;
use std::io::Result;

use crate::utils::check_range;
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 3;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Downlink {
    pub irun_to_mounting: u8,
    pub irun_to_0: u8,
    pub sgthrs: u8,
}

impl Default for Downlink {
    fn default() -> Self {
        Self {
            irun_to_mounting: 0x13,
            irun_to_0: 0x1f,
            sgthrs: 0x0f,
        }
    }
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IRUN to mounting {:#04x}\n\
            IRUN to 0% {:#04x}\n\
            SGTHRS {:#04x}",
            self.irun_to_mounting, self.irun_to_0, self.sgthrs
        )
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = bin_to_irun(self.irun_to_mounting)?;
        payload[1] = bin_to_irun(self.irun_to_0)?;
        payload[2] = self.sgthrs;

        Ok(PortPayload {
            port: Port::MotorConfig as u8,
            payload,
        })
    }
}

fn bin_to_irun(input: u8) -> Result<u8> {
    check_range(0.0, input as f32, 31.0, 1.0)?;

    Ok(input)
}

DownlinkStatus! {MotorConfig, "Motor Config"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
