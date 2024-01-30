use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::{
    bin_to_bool, bin_to_float_point_five, bin_to_float_point_two_five, bin_to_float_point_zero_two,
    bin_to_ten,
};

use super::device_value::DeviceValue;

const UPLINK_N_BYTES_REV_1_0: usize = 10;
const UPLINK_N_BYTES_REV_1_1: usize = 11;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.5)]
    flow_raw_value: f32,
    #[partial_close(resolution = 0.5)]
    flow_temperature: f32,
    #[partial_close(resolution = 0.25)]
    ambient_raw_value: f32,
    #[partial_close(resolution = 0.25)]
    ambient_temperature: f32,
    #[partial_close(resolution = 0.02)]
    battery_v: f32,
    average_current_consumed: u16,
    average_current_generated: u16,
    valve_position: u8,
    device_value: Option<DeviceValue>,
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
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        let device_value = match input.len() {
            UPLINK_N_BYTES_REV_1_0 => None,
            UPLINK_N_BYTES_REV_1_1 => Some(DeviceValue::from_bin(input[9] & 0b111, input[10])?),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Payload length should be {UPLINK_N_BYTES_REV_1_0} or {UPLINK_N_BYTES_REV_1_1}, \
                    got {input:?} which is of length {}", input.len()
                    ),
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
            device_value,
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

    pub fn device_value(&self) -> Option<&DeviceValue> {
        self.device_value.as_ref()
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
