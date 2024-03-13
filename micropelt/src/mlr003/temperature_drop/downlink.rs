use std::fmt;
use std::io::Result;

use crate::utils::bool_to_bin;
use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::{Action, Period};

const DOWNLINK_N_BYTES: usize = 1;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Downlink {
    pub action: Action,
    pub beep: bool,
    pub period: Period,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.action {
            Action::None => write!(f, "{}", self.action),
            Action::CloseFor30MinutesThenOperate | Action::CloseFor60MinutesThenOperate => {
                write!(
                    f,
                    "{}\n\
                {} beep upon resumption of normal operation\n\
                {}",
                    self.action,
                    if self.beep { "With" } else { "Without" },
                    self.period
                )
            }
        }
    }
}

impl lorawan::Downlink for Downlink {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] |= self.action.to_bin() << 6;
        payload[0] |= (bool_to_bin(self.beep) << 5);
        payload[0] |= (self.period.to_bin() << 4);

        Ok(PortPayload {
            port: Port::TemperatureDrop as u8,
            payload,
        })
    }
}

DownlinkStatus! {TemperatureDrop, "Temperature Drop Detection Settings"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
