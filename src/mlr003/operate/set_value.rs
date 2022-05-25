use std::fmt;

use super::super::utils::close;

pub const DEFAULT_AMBIENT_TEMPERATURE: f32 = 20.0;
pub const DEFAULT_FLOW_TEMPERATURE: f32 = 55.0;

#[derive(Clone, Debug)]
pub enum SetValue {
    ValvePosition(u8),
    FlowTemperature(f32),
    AmbientTemperature(f32),
}

impl fmt::Display for SetValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SetValue::ValvePosition(value) => write!(f, "Valve Position {}%", value),
            SetValue::FlowTemperature(value) => {
                write!(f, "Flow Temperature {}°C", value)
            }
            SetValue::AmbientTemperature(value) => {
                write!(f, "Ambient Temperature {}°C", value)
            }
        }
    }
}

impl PartialEq for SetValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SetValue::ValvePosition(svp), SetValue::ValvePosition(ovp)) => svp == ovp,
            (SetValue::FlowTemperature(sft), SetValue::FlowTemperature(oft)) => {
                close(*sft, *oft, 0.5)
            }
            (SetValue::AmbientTemperature(sat), SetValue::AmbientTemperature(oat)) => {
                close(*sat, *oat, 0.5)
            }
            _ => false,
        }
    }
}

impl SetValue {
    pub fn default_radiator() -> Self {
        Self::AmbientTemperature(DEFAULT_AMBIENT_TEMPERATURE)
    }

    pub fn default_domestic_hot_water() -> Self {
        Self::FlowTemperature(DEFAULT_FLOW_TEMPERATURE)
    }
}
