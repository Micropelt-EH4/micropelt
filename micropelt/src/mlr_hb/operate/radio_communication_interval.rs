use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RadioCommunicationInterval {
    Minutes5,
    Minutes10,
    Minutes15,
    Minutes20,
    Minutes30,
    Minutes60,
    Minutes120,
    Minutes480,
}

impl fmt::Display for RadioCommunicationInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount = match self {
            Self::Minutes5 => 5,
            Self::Minutes10 => 10,
            Self::Minutes15 => 15,
            Self::Minutes20 => 20,
            Self::Minutes30 => 30,
            Self::Minutes60 => 60,
            Self::Minutes120 => 120,
            Self::Minutes480 => 480,
        };
        write!(f, "{amount} minutes")
    }
}

impl RadioCommunicationInterval {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Minutes5 => 0,
            Self::Minutes10 => 1,
            Self::Minutes15 => 2,
            Self::Minutes20 => 3,
            Self::Minutes30 => 4,
            Self::Minutes60 => 5,
            Self::Minutes120 => 6,
            Self::Minutes480 => 7,
        }
    }
}
