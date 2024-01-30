use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RadioCommunicationIntervalR {
    Minutes5,
    Minutes10,
    Minutes60,
    Minutes120,
    Minutes480,
}

impl fmt::Display for RadioCommunicationIntervalR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount = match self {
            Self::Minutes5 => 5,
            Self::Minutes10 => 10,
            Self::Minutes60 => 60,
            Self::Minutes120 => 120,
            Self::Minutes480 => 480,
        };
        write!(f, "{amount} minutes")
    }
}

impl RadioCommunicationIntervalR {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Minutes5 => 1,
            Self::Minutes10 => 0,
            Self::Minutes60 => 2,
            Self::Minutes120 => 3,
            Self::Minutes480 => 4,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RadioCommunicationIntervalF {
    Minutes5,
    Minutes15,
    Minutes60,
    Minutes120,
    Minutes480,
}

impl fmt::Display for RadioCommunicationIntervalF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount = match self {
            Self::Minutes5 => 5,
            Self::Minutes15 => 15,
            Self::Minutes60 => 60,
            Self::Minutes120 => 120,
            Self::Minutes480 => 480,
        };
        write!(f, "{amount} minutes")
    }
}

impl RadioCommunicationIntervalF {
    pub(super) fn to_bin(&self) -> u8 {
        match self {
            Self::Minutes5 => 1,
            Self::Minutes15 => 0,
            Self::Minutes60 => 2,
            Self::Minutes120 => 3,
            Self::Minutes480 => 4,
        }
    }
}
