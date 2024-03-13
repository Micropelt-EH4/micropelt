use super::*;

#[test]
fn test_action_and_open() {
    let expected_output = Uplink {
        action: Action::CloseFor30MinutesThenOperate,
        period: Period::One,
        beep: false,
        temperature_drop_detected: true,
    };
    assert_eq!(
        expected_output,
        Uplink::deserialise(&vec![0b10000001]).unwrap()
    );
}
