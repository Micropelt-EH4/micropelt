use std::fmt;
use std::io::{Error, ErrorKind, Result};

use crate::utils::check_payload_length;

const UPLINK_N_BYTES: usize = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    opening_point: Option<u8>,
    slow_harvesting_status: Status,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    NoBecauseDownlinkOff,
    NoBecauseRecentlyNonZero,
    NoBecauseNoHotWater,
    CannotDetectOpeningPoint,
    NoBecauseBatteryHigh,
    NoBecauseInsufficientCurrentGenerated,
    DetectingOpeningPoint,
    SlowHarvesting,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoBecauseDownlinkOff => write!(
                f,
                "Not Slow Harvesting because Downlink set Slow Harvesting to Off"
            ),
            Self::NoBecauseRecentlyNonZero => write!(
                f,
                "Not Slow Harvesting because device position was recently non-zero"
            ),
            Self::NoBecauseNoHotWater => write!(
                f,
                "Not Slow Harvesting because hot water is expected to be off"
            ),
            Self::CannotDetectOpeningPoint => write!(f, "Cannot Detect Opening Point"),
            Self::NoBecauseBatteryHigh => {
                write!(f, "Not Slow Harvesting because device battery is high")
            }
            Self::NoBecauseInsufficientCurrentGenerated => write!(
                f,
                "Not Slow Harvesting because previous attempt generated insufficient current"
            ),
            Self::DetectingOpeningPoint => write!(f, "Detecting Opening Point"),
            Self::SlowHarvesting => write!(f, "Slow Harvesting"),
        }
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            opening_point: bin_to_opening_point(input[0])?,
            slow_harvesting_status: bin_to_slow_harvesting_status(input[1])?,
        })
    }

    pub fn opening_point(&self) -> Option<u8> {
        self.opening_point
    }

    pub fn slow_harvesting_status(&self) -> Status {
        self.slow_harvesting_status
    }
}

fn bin_to_opening_point(input: u8) -> Result<Option<u8>> {
    if input == 0 {
        Ok(None)
    } else if (input >> 7) & 1 == 0 && input != 0 {
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid Opening Point input {}", input),
        ))
    } else {
        let percent = input & 0b1111111;
        if percent > 100 {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Opening Point {} > 100%", percent),
            ))
        } else {
            Ok(Some(percent))
        }
    }
}

fn bin_to_slow_harvesting_status(input: u8) -> Result<Status> {
    match input {
        0 => Ok(Status::NoBecauseDownlinkOff),
        1 => Ok(Status::NoBecauseRecentlyNonZero),
        2 => Ok(Status::NoBecauseNoHotWater),
        3 => Ok(Status::CannotDetectOpeningPoint),
        4 => Ok(Status::NoBecauseBatteryHigh),
        5 => Ok(Status::NoBecauseInsufficientCurrentGenerated),
        6 => Ok(Status::DetectingOpeningPoint),
        7 => Ok(Status::SlowHarvesting),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("Unexpected Slow Harvesting Status {}", input),
        )),
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
