use std::io::Result;

pub struct PortPayload {
    pub port: u8,
    pub payload: Vec<u8>,
}

pub trait Downlink {
    fn serialise(&self) -> Result<PortPayload>;
}

pub trait Uplink {
    fn deserialise(input: &PortPayload) -> Result<Self>
    where
        Self: Sized;
}
