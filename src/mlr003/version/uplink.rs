use std::fmt;
use std::io::{Error, ErrorKind, Result};

const UPLINK_N_BYTES: usize = 6;

#[derive(Clone, Debug, PartialEq)]
pub struct Uplink {
    revision: Version,
    hardware: Version,
    firmware: FirmwareVersion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FirmwareVersion {
    year_since_2000: u8,
    month: u8,
    day: u8,
    minor: u8,
}

impl Uplink {
    pub(crate) fn deserialize(input: &[u8]) -> Result<Self> {
        if input.len() != UPLINK_N_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{:?} is of length {}", input, input.len()),
            ));
        }

        let revision = bin_to_version(input[0]);
        let hardware = bin_to_version(input[1]);
        let year_since_2000 = input[2];
        let month = bin_to_month(input[3])?;
        let day = bin_to_day(month, input[4])?;
        let minor = input[5];

        Ok(Self {
            revision,
            hardware,
            firmware: FirmwareVersion {
                year_since_2000,
                month,
                day,
                minor,
            },
        })
    }

    pub fn revision(&self) -> &Version {
        &self.revision
    }

    pub fn hardware_version(&self) -> &Version {
        &self.hardware
    }

    pub fn firmware_version(&self) -> &FirmwareVersion {
        &self.firmware
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{:02}.{:02}.{:02}",
            self.year_since_2000 as u16 + 2000,
            self.month,
            self.day,
            self.minor,
        )
    }
}

#[rustfmt::skip]
fn bin_to_version(value: u8) -> Version {
    Version {
        major:  value       & 0b1111,
        minor: (value >> 4) & 0b1111,
    }
}

fn bin_to_month(value: u8) -> Result<u8> {
    if 0 < value && value <= 12 {
        Ok(value)
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "There are 12 months in a year, but received month {}",
                value
            ),
        ))
    }
}

fn bin_to_day(month: u8, value: u8) -> Result<u8> {
    let n_days = month_get_n_days(month)?;
    if 0 < value && value <= n_days {
        Ok(value)
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "Month {} contains {} days, but received day {}",
                month, n_days, value
            ),
        ))
    }
}

fn month_get_n_days(month: u8) -> Result<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        2 => Ok(29),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "There are 12 months in a year, but received month {}",
                month
            ),
        )),
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
