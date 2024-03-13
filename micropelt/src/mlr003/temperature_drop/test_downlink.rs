use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_status() {
    let bytes = Downlink::status().serialise().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialise_all_responses_off() {
    let downlink = Downlink {
        action: Action::None,
        period: Period::One,
        beep: false,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b00000000], bytes);
}

#[test]
fn serialise_all_responses_on() {
    let downlink = Downlink {
        action: Action::CloseFor60MinutesThenOperate,
        period: Period::Two,
        beep: true,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b11110000], bytes);
}
