use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_kp_3_6_ki_0_40() {
    let downlink = Downlink {
        k_p: 3.6,
        k_i: 0.40,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![36, 20], bytes);
}

#[test]
fn serialise_kp_4_0_ki_0_00() {
    let downlink = Downlink {
        k_p: 4.0,
        k_i: 0.00,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![40, 0], bytes);
}

#[test]
fn serialise_kp_7_5_ki_4_12() {
    let downlink = Downlink {
        k_p: 7.5,
        k_i: 4.12,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![75, 206], bytes);
}
