use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_zero() {
    let downlink = Downlink::default();

    let bytes = downlink.serialise().unwrap().payload;

    for byte in bytes {
        assert_eq!(0, byte);
    }
}

#[test]
fn serialise_01() {
    let downlink = Downlink {
        slow_harvest: SlowHarvest::Now,
        hot_water: HotWater::On,
        opening_point_reset: false,
        flow_maximum: 23.5,
    };

    let expected_output = vec![0b00001001, 47];

    assert_eq!(expected_output, downlink.serialise().unwrap().payload);
}

#[test]
fn serialise_02() {
    let downlink = Downlink {
        slow_harvest: SlowHarvest::Off,
        hot_water: HotWater::Off,
        opening_point_reset: true,
        flow_maximum: 20.0,
    };

    let expected_output = vec![0b00010111, 40];

    assert_eq!(expected_output, downlink.serialise().unwrap().payload);
}

#[test]
fn serialise_too_cold() {
    let downlink = Downlink {
        slow_harvest: SlowHarvest::Off,
        hot_water: HotWater::Off,
        opening_point_reset: false,
        flow_maximum: -4.0,
    };

    assert!(downlink.serialise().is_err());
}

#[test]
fn serialise_too_hot() {
    let downlink = Downlink {
        slow_harvest: SlowHarvest::Off,
        hot_water: HotWater::Off,
        opening_point_reset: false,
        flow_maximum: 58.5,
    };

    assert!(downlink.serialise().is_err());
}
