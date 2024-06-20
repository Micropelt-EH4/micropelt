use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::{
    bin_to_bool, bin_to_fifty, bin_to_float_point_five, bin_to_float_point_zero_two, bin_to_ten,
};

const UPLINK_N_BYTES: usize = 8;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.5)]
    ntc1_raw_value: f32,
    #[partial_close(resolution = 0.5)]
    ntc1_temperature: f32,
    #[partial_close(resolution = 0.5)]
    ntc2_raw_value: f32,
    #[partial_close(resolution = 0.5)]
    ntc2_temperature: f32,
    #[partial_close(resolution = 0.02)]
    battery_v: f32,
    average_current_consumed: u16,
    average_current_generated: u16,
    ntc1_sensor_error: bool,
    ntc2_sensor_error: bool,
    radio_signal_strength_low: bool,
    radio_communication_error: bool,
    battery_low: bool,
    battery_high: bool,
    harvesting: bool,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        if input.len() != UPLINK_N_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("{input:?} is of length {}", input.len()),
            ));
        }

        Ok(Self {
            ntc1_raw_value: bin_to_float_point_five(input[0]),
            ntc1_temperature: bin_to_float_point_five(input[1]),
            ntc2_raw_value: bin_to_float_point_five(input[2]),
            ntc2_temperature: bin_to_float_point_five(input[3]),
            ntc1_sensor_error: bin_to_bool(input[4] & 1)?,
            ntc2_sensor_error: bin_to_bool((input[4] >> 1) & 1)?,
            radio_signal_strength_low: bin_to_bool((input[4] >> 2) & 1)?,
            radio_communication_error: bin_to_bool((input[4] >> 3) & 1)?,
            battery_low: bin_to_bool((input[4] >> 4) & 1)?,
            battery_high: bin_to_bool((input[4] >> 5) & 1)?,
            harvesting: bin_to_bool((input[4] >> 6) & 1)?,
            battery_v: bin_to_float_point_zero_two(input[5]),
            average_current_consumed: bin_to_ten(input[6]),
            average_current_generated: bin_to_fifty(input[7]),
        })
    }

    pub fn ntc1_raw_value(&self) -> f32 {
        self.ntc1_raw_value
    }

    pub fn ntc1_temperature(&self) -> f32 {
        self.ntc1_temperature
    }

    pub fn ntc2_raw_value(&self) -> f32 {
        self.ntc2_raw_value
    }

    pub fn ntc2_temperature(&self) -> f32 {
        self.ntc2_temperature
    }

    pub fn battery_v(&self) -> f32 {
        self.battery_v
    }

    pub fn average_current_consumed(&self) -> u16 {
        self.average_current_consumed
    }

    pub fn average_current_generated(&self) -> u16 {
        self.average_current_generated
    }

    pub fn ntc1_sensor_error(&self) -> bool {
        self.ntc1_sensor_error
    }

    pub fn ntc2_sensor_error(&self) -> bool {
        self.ntc2_sensor_error
    }

    pub fn radio_signal_strength_low(&self) -> bool {
        self.radio_signal_strength_low
    }

    pub fn radio_communication_error(&self) -> bool {
        self.radio_communication_error
    }

    pub fn battery_low(&self) -> bool {
        self.battery_low
    }

    pub fn battery_high(&self) -> bool {
        self.battery_high
    }

    pub fn harvesting(&self) -> bool {
        self.harvesting
    }
}

#[cfg(test)]
mod test_operate {
    use super::*;

    #[test]
    fn deserialise_00() {
        let expected_output = Uplink {
            ntc1_raw_value: 127.5,
            ntc1_temperature: 62.0,
            ntc2_raw_value: 127.5,
            ntc2_temperature: 62.0,
            ntc1_sensor_error: true,
            ntc2_sensor_error: true,
            radio_signal_strength_low: false,
            radio_communication_error: false,
            battery_low: false,
            battery_high: false,
            harvesting: false,
            battery_v: 2.6,
            average_current_consumed: 2550,
            average_current_generated: 0,
        };

        assert_eq!(
            expected_output,
            Uplink::deserialise(&[255, 124, 255, 124, 3, 130, 255, 0]).unwrap()
        );
    }
}
