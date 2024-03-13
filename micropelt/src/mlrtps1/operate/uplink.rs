use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::{
    bin_to_bool, bin_to_float_point_two_five, bin_to_float_point_zero_two, check_payload_length,
};

use super::SetPointTemperature;

const UPLINK_N_BYTES: usize = 4;

#[derive(Clone, Debug, PartialClose)]
pub struct Uplink {
    #[partial_close(resolution = 0.25)]
    ambient_temperature: f32,
    set_point_temperature: SetPointTemperature,
    #[partial_close(resolution = 0.02)]
    battery_v: f32,
    pir_status: PirStatus,
    ambient_sensor_error: bool,
    pir_sensor_error: bool,
    radio_communication_error: bool,
    radio_signal_strength_low: bool,
    battery_low: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PirStatus {
    NoMotionDetected,
    MotionDetected,
}

impl PartialEq for Uplink {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl Uplink {
    pub(crate) fn deserialise(input: &[u8]) -> Result<Self> {
        check_payload_length(input, UPLINK_N_BYTES)?;

        Ok(Self {
            ambient_temperature: bin_to_float_point_two_five(input[0]),
            pir_status: bin_to_pir_status((input[1] >> 5) & 1)?,
            battery_low: bin_to_bool((input[1] >> 4) & 1)?,
            radio_communication_error: bin_to_bool((input[1] >> 3) & 1)?,
            radio_signal_strength_low: bin_to_bool((input[1] >> 2) & 1)?,
            pir_sensor_error: bin_to_bool((input[1] >> 1) & 1)?,
            ambient_sensor_error: bin_to_bool(input[1] & 1)?,
            battery_v: bin_to_float_point_zero_two(input[2]),
            set_point_temperature: SetPointTemperature::from_bin(input[3])?,
        })
    }

    pub fn ambient_temperature(&self) -> f32 {
        self.ambient_temperature
    }

    pub fn set_point_temperature(&self) -> &SetPointTemperature {
        &self.set_point_temperature
    }

    pub fn battery_v(&self) -> f32 {
        self.battery_v
    }

    pub fn pir_status(&self) -> &PirStatus {
        &self.pir_status
    }

    pub fn ambient_sensor_error(&self) -> bool {
        self.ambient_sensor_error
    }

    pub fn pir_sensor_error(&self) -> bool {
        self.pir_sensor_error
    }

    pub fn radio_communication_error(&self) -> bool {
        self.radio_communication_error
    }

    pub fn radio_signal_strength_low(&self) -> bool {
        self.radio_signal_strength_low
    }

    pub fn battery_low(&self) -> bool {
        self.battery_low
    }
}

fn bin_to_pir_status(input: u8) -> Result<PirStatus> {
    match input {
        0 => Ok(PirStatus::NoMotionDetected),
        1 => Ok(PirStatus::MotionDetected),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("PIR status should only be 0 or 1, got {input}"),
        )),
    }
}

#[cfg(test)]
#[path = "./test_uplink.rs"]
mod test_uplink;
