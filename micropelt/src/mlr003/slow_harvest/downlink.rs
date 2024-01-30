use std::fmt;
use std::io::Result;

use crate::utils::check_range;
use crate::{lorawan, PortPayload};

use super::super::port::Port;

const DOWNLINK_N_BYTES: usize = 2;

pub const MINIMUM_FLOW_MAXIMUM_OC: f32 = 0.0;
pub const DEFAULT_FLOW_MAXIMUM_OC: f32 = 26.0;
pub const MAXIMUM_FLOW_MAXIMUM_OC: f32 = 33.0;

#[derive(Debug)]
pub struct Downlink {
    pub slow_harvest: SlowHarvest,
    pub hot_water: HotWater,
    pub opening_point_reset: bool,
    pub flow_maximum: f32,
}

#[derive(Debug)]
pub enum SlowHarvest {
    Off,
    OpdNowShOff,
    Default,
    Now,
}

#[derive(Debug)]
pub enum HotWater {
    Off,
    UseTimeOfYear,
    On,
}

impl Default for Downlink {
    fn default() -> Self {
        Self {
            slow_harvest: SlowHarvest::Default,
            hot_water: HotWater::UseTimeOfYear,
            opening_point_reset: false,
            flow_maximum: 0.0,
        }
    }
}

impl fmt::Display for Downlink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n\
            {}\n\
            {}\
            Maximum Flow Temperature {}°C",
            self.slow_harvest,
            self.hot_water,
            if self.opening_point_reset {
                "Reset Opening Point\n"
            } else {
                ""
            },
            self.flow_maximum
        )
    }
}

impl fmt::Display for SlowHarvest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Off => write!(
                f,
                "Disable Opening Point Detection, Disable Slow Harvesting"
            ),
            Self::OpdNowShOff => {
                write!(f, "Do Opening Point Detection now, Disable Slow Harvesting")
            }
            Self::Default => write!(
                f,
                "Default Opening Point Detection, Default Slow Harvesting"
            ),
            Self::Now => write!(f, "Do Opening Point Detection and Slow Harvesting now!"),
        }
    }
}

impl fmt::Display for HotWater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Off => write!(f, "Hot Water is Off"),
            Self::UseTimeOfYear => {
                write!(f, "Use time of year to determine hot water availability")
            }
            Self::On => write!(f, "Hot Water is On"),
        }
    }
}

impl lorawan::Downlink for Downlink {
    #[allow(unused_parens)]
    fn serialise(&self) -> Result<PortPayload> {
        let mut payload = vec![0; DOWNLINK_N_BYTES];

        payload[0] |= slow_harvest_to_bin(&self.slow_harvest);
        payload[0] |= (hot_water_to_bin(&self.hot_water) << 2);
        payload[0] |= (bool_to_bin(self.opening_point_reset) << 4);

        payload[1] |= flow_to_bin(self.flow_maximum)?;

        Ok(PortPayload {
            port: Port::SlowHarvest as u8,
            payload,
        })
    }
}

fn slow_harvest_to_bin(input: &SlowHarvest) -> u8 {
    match input {
        SlowHarvest::Off => 0b11,
        SlowHarvest::OpdNowShOff => 0b10,
        SlowHarvest::Default => 0b00,
        SlowHarvest::Now => 0b01,
    }
}

fn hot_water_to_bin(input: &HotWater) -> u8 {
    match input {
        HotWater::Off => 0b01,
        HotWater::UseTimeOfYear => 0b00,
        HotWater::On => 0b10,
    }
}

fn bool_to_bin(input: bool) -> u8 {
    match input {
        true => 1,
        false => 0,
    }
}

fn flow_to_bin(input: f32) -> Result<u8> {
    check_range(MINIMUM_FLOW_MAXIMUM_OC, input, MAXIMUM_FLOW_MAXIMUM_OC, 0.5)?;

    Ok((input * 2.0) as u8)
}

DownlinkStatus! {SlowHarvest, "Slow Harvest Status"}

#[cfg(test)]
#[path = "./test_downlink.rs"]
mod test_downlink;
