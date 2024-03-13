use super::*;

#[test]
fn deserialise_kp_5_3_ki_0_54() {
    let expected_output = Uplink {
        k_p: 5.3,
        k_i: 0.54,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[53, 27]).unwrap());
}
