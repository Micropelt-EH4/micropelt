use std::io::{Error, ErrorKind, Result};

use super::super::utils::close;
use super::set_value::SetValue;

const UPLINK_N_BYTES_REV_1_0: usize = 10;
const UPLINK_N_BYTES_REV_1_1: usize = 11;

#[derive(Clone, Debug)]
pub struct Uplink {
    flow_raw_value: f32,
    flow_temperature: f32,
    ambient_raw_value: f32,
    ambient_temperature: f32,
    battery_v: f32,
    average_current_consumed: u16,
    average_current_generated: u16,
    valve_position: u8,
    user_value: Option<SetValue>,
    radio_communication_error: bool,
    radio_signal_strength_low: bool,
    flow_sensor_error: bool,
    ambient_sensor_error: bool,
    battery_low: bool,
    battery_high: bool,
    harvesting: bool,
    motor_error: bool,
    reference_run_complete: bool,
    operating_condition_off: bool,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.valve_position == other.valve_position
            && close(self.flow_raw_value, other.flow_raw_value, 0.5)
            && close(self.flow_temperature, other.flow_temperature, 0.5)
            && close(self.ambient_raw_value, other.ambient_raw_value, 0.25)
            && close(self.ambient_temperature, other.ambient_temperature, 0.25)
            && self.flow_sensor_error == other.flow_sensor_error
            && self.ambient_sensor_error == other.ambient_sensor_error
            && close(self.battery_v, other.battery_v, 0.02)
            && self.battery_low == other.battery_low
            && self.battery_high == other.battery_high
            && self.average_current_consumed == other.average_current_consumed
            && self.harvesting == other.harvesting
            && self.motor_error == other.motor_error
            && self.radio_communication_error == other.radio_communication_error
            && self.radio_signal_strength_low == other.radio_signal_strength_low
            && self.reference_run_complete == other.reference_run_complete
            && self.operating_condition_off == other.operating_condition_off
            && self.user_value == other.user_value
    }
}

impl Uplink {
    pub(crate) fn deserialize(input: &[u8]) -> Result<Self> {
        let user_value = match input.len() {
            UPLINK_N_BYTES_REV_1_0 => None,
            UPLINK_N_BYTES_REV_1_1 => Some(bin_to_set_value(input[9] & 3, input[10])?),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{:?} is of length {}", input, input.len()),
                ))
            }
        };

        Ok(Self {
            valve_position: input[0],
            flow_raw_value: bin_to_float_point_five(input[1]),
            flow_temperature: bin_to_float_point_five(input[2]),
            ambient_raw_value: bin_to_float_point_two_five(input[3]),
            ambient_temperature: bin_to_float_point_two_five(input[4]),
            battery_low: bin_to_bool((input[5] >> 6) & 1)?,
            harvesting: bin_to_bool((input[5] >> 5) & 1)?,
            ambient_sensor_error: bin_to_bool((input[5] >> 4) & 1)?,
            flow_sensor_error: bin_to_bool((input[5] >> 3) & 1)?,
            radio_communication_error: bin_to_bool((input[5] >> 2) & 1)?,
            radio_signal_strength_low: bin_to_bool((input[5] >> 1) & 1)?,
            motor_error: bin_to_bool((input[5]) & 1)?,
            battery_v: bin_to_float_point_zero_two(input[6]),
            average_current_consumed: bin_to_ten(input[7]),
            average_current_generated: bin_to_ten(input[8]),
            operating_condition_off: bin_to_bool((input[9] >> 7) & 1)?,
            battery_high: bin_to_bool((input[9] >> 6) & 1)?,
            reference_run_complete: bin_to_bool((input[9] >> 4) & 1)?,
            user_value,
        })
    }

    pub fn valve_position(&self) -> u8 {
        self.valve_position
    }

    pub fn flow_raw_value(&self) -> f32 {
        self.flow_raw_value
    }

    pub fn flow_temperature(&self) -> f32 {
        self.flow_temperature
    }

    pub fn ambient_raw_value(&self) -> f32 {
        self.ambient_raw_value
    }

    pub fn ambient_temperature(&self) -> f32 {
        self.ambient_temperature
    }

    pub fn flow_sensor_error(&self) -> bool {
        self.flow_sensor_error
    }

    pub fn ambient_sensor_error(&self) -> bool {
        self.ambient_sensor_error
    }

    pub fn battery_v(&self) -> f32 {
        self.battery_v
    }

    pub fn battery_low(&self) -> bool {
        self.battery_low
    }

    pub fn battery_high(&self) -> bool {
        self.battery_high
    }

    pub fn average_current_consumed(&self) -> u16 {
        self.average_current_consumed
    }

    pub fn average_current_generated(&self) -> u16 {
        self.average_current_generated
    }

    pub fn harvesting(&self) -> bool {
        self.harvesting
    }

    pub fn motor_error(&self) -> bool {
        self.motor_error
    }

    pub fn radio_communication_error(&self) -> bool {
        self.radio_communication_error
    }

    pub fn radio_signal_strength_low(&self) -> bool {
        self.radio_signal_strength_low
    }

    pub fn reference_run_complete(&self) -> bool {
        self.reference_run_complete
    }

    pub fn operating_condition_off(&self) -> bool {
        self.operating_condition_off
    }

    pub fn user_value(&self) -> Option<&SetValue> {
        self.user_value.as_ref()
    }
}

fn bin_to_set_value(mode: u8, value: u8) -> Result<SetValue> {
    match mode {
        0 => Ok(SetValue::ValvePosition(value)),
        1 => Ok(SetValue::FlowTemperature(bin_to_float_point_five(value))),
        2 => Ok(SetValue::AmbientTemperature(bin_to_float_point_five(value))),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Unexpected set mode: {} (set value {})", mode, value),
        )),
    }
}

fn bin_to_float_point_five(input: u8) -> f32 {
    input as f32 * 0.5
}

fn bin_to_float_point_two_five(input: u8) -> f32 {
    input as f32 * 0.25
}

fn bin_to_bool(input: u8) -> Result<bool> {
    match input {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("A bit should only be 0 or 1, got {}", input),
        )),
    }
}

fn bin_to_float_point_zero_two(input: u8) -> f32 {
    input as f32 * 0.02
}

fn bin_to_ten(input: u8) -> u16 {
    input as u16 * 10
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
