mod downlink;
pub use downlink::{
    Downlink, DownlinkStatus, HotWater, SlowHarvest, DEFAULT_FLOW_MAXIMUM_OC,
    MAXIMUM_FLOW_MAXIMUM_OC, MINIMUM_FLOW_MAXIMUM_OC,
};
mod uplink;
pub use uplink::{Status, Uplink};
