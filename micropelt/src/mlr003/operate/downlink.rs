use std::fmt;
use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::bool_to_bin;
use crate::{lorawan, PortPayload};

use super::super::port::Port;
use super::device_value::SetValue;
use super::radio_communication_interval::RadioCommunicationInterval;

const DOWNLINK_N_BYTES: usize = 6;

#[derive(Clone, Debug, PartialEq)]
pub struct DownlinkF {
    pub user_value: SetValue,
    pub safety_value: SetValue,
    pub radio_communication_interval: RadioCommunicationInterval,
    pub flow_sensor_offset: i8,
    pub k_p: Kp,
    pub reference_run: bool,
}

#[derive(Clone, Debug, PartialClose)]
pub struct DownlinkR {
    pub user_value: SetValue,
    #[partial_close(resolution = 0.25)]
    pub room_temperature: f32,
    pub safety_value: SetValue,
    pub radio_communication_interval: RadioCommunicationInterval,
    pub flow_sensor_offset: i8,
    pub reference_run: bool,
}

#[derive(Clone, Debug, PartialClose)]
pub struct Downlink4 {
    pub user_value: SetValue,
    #[partial_close(resolution = 0.25)]
    pub room_temperature: f32,
    pub safety_value: SetValue,
    pub radio_communication_interval: RadioCommunicationInterval,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kp {
    Kp4,
    Kp8,
    Kp12,
    Kp16,
}

impl fmt::Display for DownlinkF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User Value {}\n\
        Safety Value {}\n\
        Radio Communication Interval {}\n\
        Flow Sensor Offset {}\n\
        {}\n\
        Reference Run {}",
            self.user_value,
            self.safety_value,
            self.radio_communication_interval,
            self.flow_sensor_offset,
            self.k_p,
            self.reference_run
        )
    }
}

impl fmt::Display for DownlinkR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User Value {}\n\
        Room Temperature {}°C\n\
        Safety Value {}\n\
        Radio Communication Interval {}\n\
        Flow Sensor Offset {}\n\
        Reference Run {}",
            self.user_value,
            self.room_temperature,
            self.safety_value,
            self.radio_communication_interval,
            self.flow_sensor_offset,
            self.reference_run
        )
    }
}

impl fmt::Display for Downlink4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User Value {}\n\
        Room Temperature {}°C\n\
        Safety Value {}\n\
        Radio Communication Interval {}",
            self.user_value,
            self.room_temperature,
            self.safety_value,
            self.radio_communication_interval
        )
    }
}

impl fmt::Display for Kp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Kp4 => write!(f, "K P 4"),
            Self::Kp8 => write!(f, "K P 8"),
            Self::Kp12 => write!(f, "K P 12"),
            Self::Kp16 => write!(f, "K P 16"),
        }
    }
}

impl PartialEq for DownlinkR {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl PartialEq for Downlink4 {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Default for DownlinkF {
    fn default() -> Self {
        Self {
            user_value: SetValue::default_domestic_hot_water(),
            safety_value: SetValue::default_domestic_hot_water(),
            radio_communication_interval: RadioCommunicationInterval::Minutes60,
            flow_sensor_offset: 0,
            k_p: Kp::Kp4,
            reference_run: false,
        }
    }
}

impl Default for DownlinkR {
    fn default() -> Self {
        Self {
            user_value: SetValue::default_radiator(),
            room_temperature: 0.0,
            safety_value: SetValue::default_radiator(),
            radio_communication_interval: RadioCommunicationInterval::Minutes10,
            flow_sensor_offset: 0,
            reference_run: false,
        }
    }
}

impl lorawan::Downlink for DownlinkF {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.user_value.value_to_bin()?;
        payload[1] = 0;
        payload[2] = self.safety_value.value_to_bin()?;
        payload[3] = (self.radio_communication_interval.to_bin() << 4);
        payload[3] |= (self.user_value.user_mode_to_bin() << 2);
        payload[3] |= self.safety_value.safety_mode_to_bin();
        payload[4] = (offset_comp_to_bin(self.flow_sensor_offset)? << 4);
        payload[5] = (self.k_p.to_bin() << 5);
        payload[5] |= (bool_to_bin(self.reference_run) << 7);

        Ok(PortPayload {
            port: Port::Operate as u8,
            payload,
        })
    }
}

impl lorawan::Downlink for DownlinkR {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = self.user_value.value_to_bin()?;
        payload[1] = float_point_two_five_to_bin(self.room_temperature);
        payload[2] = self.safety_value.value_to_bin()?;
        payload[3] = (self.radio_communication_interval.to_bin() << 4);
        payload[3] |= (self.user_value.user_mode_to_bin() << 2);
        payload[3] |= self.safety_value.safety_mode_to_bin();
        payload[4] = (offset_comp_to_bin(self.flow_sensor_offset)? << 4);
        payload[5] = (bool_to_bin(self.reference_run) << 7);

        Ok(PortPayload {
            port: Port::Operate as u8,
            payload,
        })
    }
}

impl lorawan::Downlink for Downlink4 {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; 4];

        payload[0] = self.user_value.value_to_bin()?;
        payload[1] = float_point_two_five_to_bin(self.room_temperature);
        payload[2] = self.safety_value.value_to_bin()?;
        payload[3] = (self.radio_communication_interval.to_bin() << 4);
        payload[3] |= (self.user_value.user_mode_to_bin() << 2);
        payload[3] |= self.safety_value.safety_mode_to_bin();

        Ok(PortPayload {
            port: Port::Operate as u8,
            payload,
        })
    }
}

fn float_point_two_five_to_bin(input: f32) -> u8 {
    (input * 4.0) as u8
}

fn offset_comp_to_bin(input: i8) -> Result<u8> {
    let output = match input {
        -8 => 8,
        -7 => 9,
        -6 => 10,
        -5 => 11,
        -4 => 12,
        -3 => 13,
        -2 => 14,
        -1 => 15,
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected offset: {input}"),
            ))
        }
    };

    Ok(output)
}

impl Kp {
    fn to_bin(&self) -> u8 {
        match self {
            Self::Kp4 => 2,
            Self::Kp8 => 3,
            Self::Kp12 => 0,
            Self::Kp16 => 1,
        }
    }
}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
