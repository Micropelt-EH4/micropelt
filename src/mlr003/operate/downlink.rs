use std::fmt;
use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::super::utils::{close, Port};
use super::set_value::SetValue;

const DOWNLINK_N_BYTES: usize = 6;

#[derive(Clone, Debug)]
pub struct Downlink {
    pub user_value: SetValue,
    pub room_temperature: f32,
    pub safety_value: SetValue,
    pub radio_communication_interval: RadioCommunicationInterval,
    pub flow_sensor_offset: i8,
    pub p_controller_gain: u8,
    pub reference_run: bool,
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User Value {}\n\
        Room Temperature {}°C\n\
        Safety Value {}\n\
        Radio Communication Interval {}\n\
        Flow Sensor Offset {}\n\
        P Controller Gain {}\n\
        Reference Run {}",
            self.user_value,
            self.room_temperature,
            self.safety_value,
            self.radio_communication_interval,
            self.flow_sensor_offset,
            self.p_controller_gain,
            self.reference_run
        )
    }
}

impl PartialEq for Downlink {
    fn eq(&self, other: &Self) -> bool {
        self.user_value == other.user_value
            && close(self.room_temperature, other.room_temperature, 0.25)
            && self.safety_value == other.safety_value
            && self.radio_communication_interval == other.radio_communication_interval
            && self.flow_sensor_offset == other.flow_sensor_offset
            && self.p_controller_gain == other.p_controller_gain
            && self.reference_run == other.reference_run
    }
}

impl Downlink {
    pub fn default_radiator() -> Self {
        Self {
            user_value: SetValue::default_radiator(),
            room_temperature: 0.0,
            safety_value: SetValue::default_radiator(),
            radio_communication_interval: RadioCommunicationInterval::Minutes10,
            flow_sensor_offset: 0,
            p_controller_gain: 3,
            reference_run: false,
        }
    }

    pub fn default_domestic_hot_water() -> Self {
        Self {
            user_value: SetValue::default_domestic_hot_water(),
            room_temperature: 0.0,
            safety_value: SetValue::default_domestic_hot_water(),
            radio_communication_interval: RadioCommunicationInterval::Minutes10,
            flow_sensor_offset: 0,
            p_controller_gain: 3,
            reference_run: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RadioCommunicationInterval {
    Minutes5,
    Minutes10,
    Minutes60,
    Minutes120,
    Minutes480,
}

impl fmt::Display for RadioCommunicationInterval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount = match self {
            RadioCommunicationInterval::Minutes5 => 5,
            RadioCommunicationInterval::Minutes10 => 10,
            RadioCommunicationInterval::Minutes60 => 60,
            RadioCommunicationInterval::Minutes120 => 120,
            RadioCommunicationInterval::Minutes480 => 480,
        };
        write!(f, "{} minutes", amount)
    }
}

impl lorawan::Downlink for Downlink {
    #[allow(unused_parens)]
    fn serialize(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] = set_value_to_bin(&self.user_value);
        payload[1] = float_point_two_five_to_bin(self.room_temperature);
        payload[2] = set_value_to_bin(&self.safety_value);
        payload[3] |=
            (radio_communication_interval_to_bin(&self.radio_communication_interval) << 4);
        payload[3] |= (user_mode_to_bin(&self.user_value) << 2);
        payload[3] |= safety_mode_to_bin(&self.safety_value);
        payload[4] |= (offset_comp_to_bin(self.flow_sensor_offset)? << 4);
        payload[5] |= (p_controller_gain_to_bin(self.p_controller_gain)? << 5);
        payload[5] |= (bool_to_bin(self.reference_run) << 7);

        Ok(PortPayload {
            port: Port::Operate as u8,
            payload,
        })
    }
}

fn set_value_to_bin(mode_and_value: &SetValue) -> u8 {
    match mode_and_value {
        SetValue::ValvePosition(value) => *value,
        SetValue::FlowTemperature(value) => float_point_five_to_bin(*value),
        SetValue::AmbientTemperature(value) => float_point_five_to_bin(*value),
    }
}

fn float_point_two_five_to_bin(input: f32) -> u8 {
    (input * 4.0) as u8
}

fn float_point_five_to_bin(input: f32) -> u8 {
    (input * 2.0) as u8
}

fn radio_communication_interval_to_bin(input: &RadioCommunicationInterval) -> u8 {
    match input {
        RadioCommunicationInterval::Minutes5 => 1,
        RadioCommunicationInterval::Minutes10 => 0,
        RadioCommunicationInterval::Minutes60 => 2,
        RadioCommunicationInterval::Minutes120 => 3,
        RadioCommunicationInterval::Minutes480 => 4,
    }
}

fn user_mode_to_bin(input: &SetValue) -> u8 {
    match input {
        SetValue::ValvePosition(_) => 0,
        SetValue::FlowTemperature(_) => 1,
        SetValue::AmbientTemperature(_) => 2,
    }
}

fn safety_mode_to_bin(input: &SetValue) -> u8 {
    match input {
        SetValue::AmbientTemperature(_) => 0,
        SetValue::FlowTemperature(_) => 1,
        SetValue::ValvePosition(_) => 2,
    }
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
                format!("Unexpected offset: {}", input),
            ))
        }
    };

    Ok(output)
}

fn p_controller_gain_to_bin(input: u8) -> Result<u8> {
    let output = match input {
        1 => 2,
        2 => 3,
        3 => 0,
        4 => 1,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected proportional controller gain: {}", input),
            ))
        }
    };

    Ok(output)
}

fn bool_to_bin(input: bool) -> u8 {
    match input {
        true => 1,
        false => 0,
    }
}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
