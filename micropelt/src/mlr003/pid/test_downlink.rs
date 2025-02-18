use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_kp_3_ki_0_40() {
    let downlink = Downlink {
        k_p: 3,
        k_i: 0.40,
        k_d: 0.0,
        closed_percent: 0,
        k_d_when_closed: 0.0,
        offset_percent: 0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![3, 20, 0, 128, 0, 0, 0], bytes);
}

#[test]
fn serialise_kp_4_ki_0_00() {
    let downlink = Downlink {
        k_p: 4,
        k_i: 0.00,
        k_d: 0.0,
        closed_percent: 0,
        k_d_when_closed: 0.0,
        offset_percent: 0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![4, 0, 0, 128, 0, 0, 0], bytes);
}

#[test]
fn serialise_kp_255_ki_4_12() {
    let downlink = Downlink {
        k_p: 255,
        k_i: 4.12,
        k_d: 0.0,
        closed_percent: 0,
        k_d_when_closed: 0.0,
        offset_percent: 0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![255, 206, 0, 128, 0, 0, 0], bytes);
}
