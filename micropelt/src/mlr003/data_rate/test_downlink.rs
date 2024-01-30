use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_status() {
    let port_payload = Downlink::status().serialise().unwrap();
    assert_eq!(Port::DataRate as u8, port_payload.port);
    assert!(port_payload.payload.is_empty());
}

#[test]
fn display_status() {
    assert_eq!(
        String::from("Get Data Rate"),
        Downlink::status().to_string()
    );
}

#[test]
fn serialise_sf7bw125() {
    let downlink = Downlink {
        data_rate: DataRate::Sf7Bw125,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0], bytes);
}

#[test]
fn serialise_sf8bw125() {
    let downlink = Downlink {
        data_rate: DataRate::Sf8Bw125,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0b00000001], bytes);
}
