use std::fmt;
use std::io::Result;

use micropelt_derive::PartialClose;

use crate::utils::float_point_two_five_to_bin;
use crate::{lorawan, PortPayload};

use super::port::Port;

const DOWNLINK_N_BYTES: usize = 1;

#[derive(PartialClose)]
pub struct Downlink {
    #[partial_close(resolution = 0.25)]
    pub room_temperature: f32,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Room Temperature {:.2}", self.room_temperature)
    }
}

impl PartialEq for Downlink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl lorawan::Downlink for Downlink {
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = float_point_two_five_to_bin(self.room_temperature);

        Ok(PortPayload {
            port: Port::ExternalTemperature as u8,
            payload,
        })
    }
}
