use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_off() {
    let downlink = Downlink {
        stay_on_regardless_of_calibration: false,
        do_recalibration_now: false,
        switch_off_now: true,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b00000001], bytes);
}

#[test]
fn serailize_on_and_recalibrate() {
    let downlink = Downlink {
        stay_on_regardless_of_calibration: true,
        do_recalibration_now: true,
        switch_off_now: false,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b11000000], bytes);
}
