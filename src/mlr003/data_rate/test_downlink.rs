use super::*;

use crate::lorawan::Downlink as LorawanDownlink;

#[test]
fn serialize_empty() {
    let downlink = Downlink { data_rate: None };
    let bytes = downlink.serialize().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialize_sf7bw125() {
    let downlink = Downlink {
        data_rate: Some(DataRate::Sf7Bw125),
    };
    let bytes = downlink.serialize().unwrap().payload;
    assert_eq!(vec![0], bytes);
}

#[test]
fn serialize_sf8bw125() {
    let downlink = Downlink {
        data_rate: Some(DataRate::Sf8Bw125),
    };
    let bytes = downlink.serialize().unwrap().payload;
    assert_eq!(vec![0b00000001], bytes);
}
