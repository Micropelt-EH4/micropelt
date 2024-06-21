use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_kp_3_6_ki_0_40() {
    let downlink = Downlink {
        k_p: 3.6,
        k_i: 0.40,
        k_d: 0.0,
        integral_unwind: IntegralUnwind::Zero,
        closed_percent: 0,
        k_d_when_closed: 0.0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![36, 20, 0, 0, 0, 0], bytes);
}

#[test]
fn serialise_kp_4_0_ki_0_00() {
    let downlink = Downlink {
        k_p: 4.0,
        k_i: 0.00,
        k_d: 0.0,
        integral_unwind: IntegralUnwind::Zero,
        closed_percent: 0,
        k_d_when_closed: 0.0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![40, 0, 0, 0, 0, 0], bytes);
}

#[test]
fn serialise_kp_7_5_ki_4_12() {
    let downlink = Downlink {
        k_p: 7.5,
        k_i: 4.12,
        k_d: 0.0,
        integral_unwind: IntegralUnwind::BackCalculate,
        closed_percent: 0,
        k_d_when_closed: 0.0,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![75, 206, 0, 1, 0, 0], bytes);
}
