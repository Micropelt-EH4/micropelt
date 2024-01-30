use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_status() {
    let bytes = Downlink::status().serialise().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialise_1_872() {
    let downlink = Downlink {
        travel_distance: TravelDistance::Um1872,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b10001001], bytes);
}

#[test]
fn serialise_2_56048() {
    let downlink = Downlink {
        travel_distance: TravelDistance::Um2560Point48,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b10000000], bytes);
}
