use super::*;

#[test]
fn deserialise_wrong_length() {
    assert!(Uplink::deserialise(&[0]).is_err());
}

#[test]
fn deserialise_zero() {
    let expected_output = Uplink {
        opening_point: None,
        slow_harvesting_status: Status::NoBecauseDownlinkOff,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[0, 0]).unwrap());
}

#[test]
fn deserialise_01() {
    let expected_output = Uplink {
        opening_point: Some(8),
        slow_harvesting_status: Status::SlowHarvesting,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[0b10001000, 7]).unwrap()
    );
}
