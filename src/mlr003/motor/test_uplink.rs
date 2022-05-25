use super::*;

#[test]
fn deserialize_wrong_length() {
    assert!(Uplink::deserialize(&[]).is_err());
    assert!(Uplink::deserialize(&[0, 0]).is_err());
}

#[test]
fn deserialize_1_456() {
    let expected_output = Uplink {
        travel_distance: TravelDistance::Nm1456,
    };

    assert_eq!(expected_output, Uplink::deserialize(&[0b00000111]).unwrap());
}

#[test]
fn deserialize_2_496() {
    let expected_output = Uplink {
        travel_distance: TravelDistance::Nm2496,
    };

    assert_eq!(expected_output, Uplink::deserialize(&[12]).unwrap());
    assert_eq!(
        expected_output,
        Uplink::deserialize(&base64::decode("DA==").unwrap()).unwrap()
    );
}
