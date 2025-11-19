use super::*;

#[test]
fn deserialise_rev_2_b() {
    let expected_output = Uplink {
        k_p: 1.0,
        k_i: 2.34,
        k_d: 4.6,
        closed_percent: 6,
        k_d_when_closed: 4.0,
        offset_percent: 0,
        pid_inverse: true,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[1, 117, 23, 128, 6, 20, 0]).unwrap()
    );
}
