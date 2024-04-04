use super::*;

use super::super::Downlink;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_status() {
    let bytes = Downlink::status().serialise().unwrap().payload;
    assert!(bytes.is_empty());
}

#[test]
fn serialise_offset_minus_7() {
    let downlink = Downlink { flow_offset: -7.0 };

    assert_eq!(vec![228], downlink.serialise().unwrap().payload);
}

#[test]
fn serialise_offset_zero() {
    let downlink = Downlink { flow_offset: 0.0 };

    assert_eq!(vec![0], downlink.serialise().unwrap().payload);
}

#[test]
fn tx_rx() {
    let downlink = Downlink { flow_offset: 0.25 };

    let uplink = Uplink { flow_offset: 0.25 };

    assert_eq!(
        uplink,
        Uplink::deserialise(&downlink.serialise().unwrap().payload).unwrap(),
    );
}
