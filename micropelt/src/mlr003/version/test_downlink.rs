use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_empty() {
    let downlink = Downlink::default();
    let bytes = downlink.serialise().unwrap().payload;
    assert!(bytes.is_empty());
}
