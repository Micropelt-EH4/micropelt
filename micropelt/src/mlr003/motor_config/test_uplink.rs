use super::*;

#[test]
fn deserialise_123() {
    let expected_output = Uplink {
        irun_to_mounting: 1,
        irun_to_0: 2,
        sgthrs: 3,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[1, 2, 3]).unwrap());
}
