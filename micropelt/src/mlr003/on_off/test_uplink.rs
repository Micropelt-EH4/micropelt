use super::*;

#[test]
fn deserialise_done_recalibration() {
    let expected_output = Uplink {
        stays_on_regardless_of_calibration: false,
        done_recalibration_now: true,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[0b01000000]).unwrap());
}
