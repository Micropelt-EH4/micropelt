use std::fmt;
use std::io::Result;

use crate::utils::bool_to_bin;
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Downlink {
    pub stay_on_regardless_of_calibration: bool,
    pub do_recalibration_now: bool,
    pub switch_off_now: bool,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "At 6 weeks, {}{}{}",
            if self.stay_on_regardless_of_calibration {
                "stay on"
            } else {
                "if unmounted, switch off"
            },
            if self.do_recalibration_now {
                "\nDo a Recalibration now"
            } else {
                ""
            },
            if self.switch_off_now {
                "\nSwitch Off now"
            } else {
                ""
            }
        )
    }
}

impl Default for Downlink {
    fn default() -> Self {
        Self {
            stay_on_regardless_of_calibration: true,
            do_recalibration_now: false,
            switch_off_now: false,
        }
    }
}

impl lorawan::Downlink for Downlink {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] |= (bool_to_bin(self.stay_on_regardless_of_calibration) << 7);
        payload[0] |= (bool_to_bin(self.do_recalibration_now) << 6);
        payload[0] |= bool_to_bin(self.switch_off_now);

        Ok(PortPayload {
            port: Port::OnOff as u8,
            payload,
        })
    }
}

DownlinkStatus! {OnOff, "On/Off Configuration"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
