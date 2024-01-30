use std::fmt;
use std::io::{Error, ErrorKind, Result};

use micropelt_derive::PartialClose;

use crate::utils::check_range;

pub const DEFAULT_AMBIENT_TEMPERATURE: f32 = 20.0;
pub const DEFAULT_FLOW_TEMPERATURE: f32 = 55.0;

#[derive(Clone, Debug, PartialClose)]
pub enum SetValue {
    ValvePosition(u8),
    #[partial_close(resolution = 0.5)]
    FlowTemperature(f32),
    #[partial_close(resolution = 0.5)]
    AmbientTemperature(f32),
}

impl fmt::Display for SetValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ValvePosition(value) => write!(f, "Valve Position {value}%"),
            Self::FlowTemperature(value) => {
                write!(f, "Flow Temperature {value}°C")
            }
            Self::AmbientTemperature(value) => {
                write!(f, "Ambient Temperature {value}°C")
            }
        }
    }
}

impl PartialEq for SetValue {
    fn eq(&self, other: &Self) -> bool {
        self.partial_close(other)
    }
}

impl SetValue {
    pub fn default_radiator() -> Self {
        Self::AmbientTemperature(DEFAULT_AMBIENT_TEMPERATURE)
    }

    pub fn default_domestic_hot_water() -> Self {
        Self::FlowTemperature(DEFAULT_FLOW_TEMPERATURE)
    }

    pub(super) fn from_bin(mode: u8, value: u8) -> Result<Self> {
        match mode {
            0 => Ok(Self::ValvePosition(value)),
            1 => Ok(Self::FlowTemperature(bin_to_float_point_five(value))),
            2 => Ok(Self::AmbientTemperature(bin_to_float_point_five(value))),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Unexpected set mode: {mode} (set value {value})"),
            )),
        }
    }

    pub(super) fn value_to_bin(&self) -> Result<u8> {
        match self {
            SetValue::ValvePosition(value) => Ok(percent_to_bin(*value)?),
            SetValue::FlowTemperature(value) => Ok(float_point_five_to_bin(*value, 80.0)?),
            SetValue::AmbientTemperature(value) => Ok(float_point_five_to_bin(*value, 40.0)?),
        }
    }

    pub(super) fn user_mode_to_bin(&self) -> u8 {
        match self {
            SetValue::ValvePosition(_) => 0,
            SetValue::FlowTemperature(_) => 1,
            SetValue::AmbientTemperature(_) => 2,
        }
    }

    pub(super) fn safety_mode_to_bin(&self) -> u8 {
        match self {
            SetValue::AmbientTemperature(_) => 0,
            SetValue::FlowTemperature(_) => 1,
            SetValue::ValvePosition(_) => 2,
        }
    }
}

pub(super) fn bin_to_float_point_five(input: u8) -> f32 {
    input as f32 * 0.5
}

fn percent_to_bin(input: u8) -> Result<u8> {
    check_range(0.0, input as f32, 100.0, 1.0)?;

    Ok(input)
}

fn float_point_five_to_bin(input: f32, upper: f32) -> Result<u8> {
    check_range(0.0, input, upper, 0.5)?;

    Ok((input * 2.0) as u8)
}

#[cfg(test)]
#[path = "./test_set_value.rs"]
mod test_set_value;
