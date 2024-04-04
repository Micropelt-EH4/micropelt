use super::*;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_default() {
    let downlink = Downlink {
        irun_to_mounting: 0x13,
        irun_to_0: 0x1f,
        sgthrs: 0x0f,
    };
    let bytes = downlink.serialise().unwrap().payload;
    assert_eq!(vec![0x13, 0x1f, 0x0f], bytes);
}
