use super::*;

use crate::lorawan::Downlink as LorawanDownlink;

#[test]
fn serialize_empty() {
    let downlink = Downlink::default();
    let bytes = downlink.serialize().unwrap().payload;
    assert!(bytes.is_empty());
}
