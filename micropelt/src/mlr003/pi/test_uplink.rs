use super::*;

#[test]
fn deserialise_kp_5_3_ki_0_54() {
    let expected_output = Uplink {
        k_p: 5.3,
        k_i: 0.54,
        integral_unwind: IntegralUnwind::Zero,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[53, 27, 0, 0]).unwrap()
    );
}

#[test]
fn deserialise_rev_2_3() {
    let expected_output = Uplink {
        k_p: 3.5,
        k_i: 0.6,
        integral_unwind: IntegralUnwind::BackCalculate,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[35, 30]).unwrap());
}
