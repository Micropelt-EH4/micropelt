use std::fmt;
use std::io::{Error, ErrorKind, Result};

use crate::utils::check_payload_length;

const UPLINK_N_BYTES: usize = 6;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uplink {
    revision: Version,
    hardware: Version,
    firmware: FirmwareVersion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirmwareVersion {
    year_since_2000: u8,
    month: u8,
    day: u8,
    minor: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:01x}.{:01x}", self.major, self.minor)
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

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        let revision = Version::from_bin(input[0]);
        let hardware = Version::from_bin(input[1]);
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

impl Version {
    #[rustfmt::skip]
    pub fn from_bin(value: u8) -> Self {
        Self {
            major:  value       & 0b1111,
            minor: (value >> 4) & 0b1111,
        }
    }
}

fn bin_to_month(value: u8) -> Result<u8> {
    if 0 < value && value <= 12 {
        Ok(value)
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!("There are 12 months in a year, but received month {value}"),
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
            format!("Month {month} contains {n_days} days, but received day {value}",),
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
            format!("There are 12 months in a year, but received month {month}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_to_string() {
        assert_eq!("8.3", Version { major: 8, minor: 3 }.to_string());
        assert_eq!(
            "2.a",
            Version {
                major: 2,
                minor: 10
            }
            .to_string()
        );
    }

    #[test]
    fn firmware_version_to_string() {
        assert_eq!(
            "2098.07.06.05",
            FirmwareVersion {
                year_since_2000: 98,
                month: 7,
                day: 6,
                minor: 5,
            }
            .to_string()
        );

        assert_eq!(
            "2000.12.25.00",
            FirmwareVersion {
                year_since_2000: 0,
                month: 12,
                day: 25,
                minor: 0,
            }
            .to_string()
        );
    }

    #[test]
    fn deserialise() {
        let input = [0x9a, 0xc6, 210, 9, 3, 24];

        let uplink = Uplink::deserialise(&input).unwrap();

        assert_eq!(
            Version {
                major: 0xa,
                minor: 9
            },
            uplink.revision
        );
        assert_eq!(
            Version {
                major: 6,
                minor: 0xc
            },
            uplink.hardware
        );
        assert_eq!(
            FirmwareVersion {
                year_since_2000: 210,
                month: 9,
                day: 3,
                minor: 24,
            },
            uplink.firmware
        );
    }

    #[test]
    fn bin_to_version() {
        assert_eq!(
            Version {
                major: 11,
                minor: 7
            },
            Version::from_bin(123)
        );
    }

    #[test]
    fn bin_to_month_ok() {
        assert_eq!(6, bin_to_month(6).unwrap());
    }

    #[test]
    fn bin_to_month_err() {
        assert!(bin_to_month(0).is_err());
        assert!(bin_to_month(13).is_err());
        assert!(bin_to_month(200).is_err());
    }

    #[test]
    fn bin_to_day_ok() {
        assert_eq!(1, bin_to_day(12, 1).unwrap());
    }

    #[test]
    fn bin_to_day_err() {
        assert!(bin_to_day(0, 1).is_err());
        assert!(bin_to_day(1, 0).is_err());
        assert!(bin_to_day(1, 32).is_err());
        assert!(bin_to_day(2, 30).is_err());
        assert!(bin_to_day(4, 31).is_err());
    }

    #[test]
    fn month_get_n_days_err() {
        assert!(month_get_n_days(0).is_err());
        assert!(month_get_n_days(13).is_err());
        assert!(month_get_n_days(17).is_err());
    }
}
