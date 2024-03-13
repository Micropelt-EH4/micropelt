use std::fmt;

const N_5_MINUTES_IN_1_HOUR: u8 = 12;

#[derive(Clone, Debug, PartialEq)]
pub enum Expiry {
    UntilDeviceLosesGatewayConnection,
    N5Minutes(u8),
}

impl fmt::Display for Expiry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UntilDeviceLosesGatewayConnection | Self::N5Minutes(0) => {
                write!(f, "until device loses Gateway communication")
            }
            Self::N5Minutes(n_5_minutes) if *n_5_minutes == N_5_MINUTES_IN_1_HOUR => {
                write!(f, "until 1 hour passes")
            }
            Self::N5Minutes(n_5_minutes) if n_5_minutes % N_5_MINUTES_IN_1_HOUR == 0 => write!(
                f,
                "until {} hours pass",
                n_5_minutes / N_5_MINUTES_IN_1_HOUR
            ),
            Self::N5Minutes(n_5_minutes) => {
                write!(f, "until {} minutes pass", (*n_5_minutes as u16) * 5)
            }
        }
    }
}

impl Expiry {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::UntilDeviceLosesGatewayConnection | Self::N5Minutes(0) => 0,
            Self::N5Minutes(n_5_minutes) => *n_5_minutes,
        }
    }

    pub(super) fn from_bin(input: u8) -> Self {
        match input {
            0 => Self::UntilDeviceLosesGatewayConnection,
            n_5_minutes => Self::N5Minutes(n_5_minutes),
        }
    }
}

#[cfg(test)]
#[path = "./test_expiry.rs"]
mod test_expiry;
