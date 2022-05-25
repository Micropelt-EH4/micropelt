use super::*;

use crate::lorawan::Downlink as LorawanDownlink;

#[test]
fn serialize_empty() {
    let downlink = Downlink {
        travel_distance: None,
    };
    let bytes = downlink.serialize().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialize_1_872() {
    let downlink = Downlink {
        travel_distance: Some(TravelDistance::Nm1872),
    };
    let bytes = downlink.serialize().unwrap().payload;
    assert_eq!(vec![0b10001001], bytes);
}

#[test]
fn serialize_2_56048() {
    let downlink = Downlink {
        travel_distance: Some(TravelDistance::Nm2560Point48),
    };
    let bytes = downlink.serialize().unwrap().payload;
    assert_eq!(vec![0b10000000], bytes);
}
