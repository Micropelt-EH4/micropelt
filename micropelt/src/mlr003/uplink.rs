use std::io::{Error, ErrorKind, Result};

use crate::{lorawan, PortPayload};

use super::port::Port;
use super::{
    data_rate, external_sensor, motor, on_off, operate, pi, slow_harvest, temperature_drop,
    temperature_estimate, version,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Uplink {
    Operate(operate::Uplink),
    Version(version::Uplink),
    DataRate(data_rate::Uplink),
    Motor(motor::Uplink),
    SlowHarvest(slow_harvest::Uplink),
    TemperatureDrop(temperature_drop::Uplink),
    Pi(pi::Uplink),
    TemperatureEstimate(temperature_estimate::Uplink),
    ExternalSensor(external_sensor::Uplink),
    OnOff(on_off::Uplink),
}

impl lorawan::Uplink for Uplink {
    fn deserialise(input: &PortPayload) -> Result<Self> {
        if input.port == Port::Operate as u8 {
            Ok(Self::Operate(operate::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::Version as u8 {
            Ok(Self::Version(version::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::Motor as u8 {
            Ok(Self::Motor(motor::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::DataRate as u8 {
            Ok(Self::DataRate(data_rate::Uplink::deserialise(
                &input.payload,
            )?))
        } else if input.port == Port::SlowHarvest as u8 {
            Ok(Self::SlowHarvest(slow_harvest::Uplink::deserialise(
                &input.payload,
            )?))
        } else if input.port == Port::TemperatureDrop as u8 {
            Ok(Self::TemperatureDrop(
                temperature_drop::Uplink::deserialise(&input.payload)?,
            ))
        } else if input.port == Port::Pi as u8 {
            Ok(Self::Pi(pi::Uplink::deserialise(&input.payload)?))
        } else if input.port == Port::TemperatureEstimate as u8 {
            Ok(Self::TemperatureEstimate(
                temperature_estimate::Uplink::deserialise(&input.payload)?,
            ))
        } else if input.port == Port::ExternalSensor as u8 {
            Ok(Self::ExternalSensor(external_sensor::Uplink::deserialise(
                &input.payload,
            )?))
        } else if input.port == Port::OnOff as u8 {
            Ok(Self::OnOff(on_off::Uplink::deserialise(&input.payload)?))
        } else {
            Err(Error::new(
                ErrorKind::Unsupported,
                format!("fport {} is not yet used", input.port),
            ))
        }
    }
}
