use super::*;

use super::super::Downlink;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_status() {
    let bytes = Downlink::status().serialise().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialise_add_minus_7_mul_1p1_power_1p3() {
    let downlink = Downlink {
        flow_add: -7.0,
        flow_multiply: 1.1,
        n_delay_flow: 0,
        n_moving_average_flow: 1,
        n_moving_average_ambient: 1,
        n_moving_average_all: 1,
        ratio_offset: 0.0,
        power: 1.3,
    };

    assert_eq!(
        vec![228, 110, 0, 1, 1, 1, 0, 130],
        downlink.serialise().unwrap().payload
    );
}

#[test]
fn serialise_super_smooth() {
    let downlink = Downlink {
        flow_add: 0.0,
        flow_multiply: 1.0,
        n_delay_flow: 0,
        n_moving_average_flow: 12,
        n_moving_average_ambient: 2,
        n_moving_average_all: 6,
        ratio_offset: 0.0,
        power: 1.3,
    };

    assert_eq!(
        vec![0, 100, 0, 12, 2, 6, 0, 130],
        downlink.serialise().unwrap().payload
    );
}

#[test]
fn tx_rx() {
    let downlink = Downlink {
        flow_add: 0.25,
        flow_multiply: 1.01,
        n_delay_flow: 1,
        n_moving_average_flow: 2,
        n_moving_average_ambient: 3,
        n_moving_average_all: 4,
        ratio_offset: 0.01,
        power: 0.02,
    };

    let uplink = Uplink {
        flow_add: 0.25,
        flow_multiply: 1.01,
        n_delay_flow: 1,
        n_moving_average_flow: 2,
        n_moving_average_ambient: 3,
        n_moving_average_all: 4,
        ratio_offset: 0.01,
        power: 0.02,
    };

    assert_eq!(
        uplink,
        Uplink::deserialise(&downlink.serialise().unwrap().payload).unwrap(),
    );
}
