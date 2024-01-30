use super::*;

use base64::Engine as _;

#[test]
fn deserialise_wrong_length() {
    assert!(Uplink::deserialise(&[]).is_err());
    assert!(Uplink::deserialise(&[0, 0]).is_err());
}

#[test]
fn deserialise_1_456() {
    let expected_output = Uplink {
        travel_distance: TravelDistance::Um1456,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[0b00000111]).unwrap());
}

#[test]
fn deserialise_2_496() {
    let expected_output = Uplink {
        travel_distance: TravelDistance::Um2496,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[12]).unwrap());
    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("DA==")
                .unwrap()
        )
        .unwrap()
    );
}
