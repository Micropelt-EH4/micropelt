use super::*;

#[test]
fn deserialise_rev_2_a() {
    let expected_output = Uplink {
        k_p: 1.2,
        k_i: 2.34,
        k_d: 4.6,
        integral_unwind: IntegralUnwind::BackCalculate,
        closed_percent: 6,
        k_d_when_closed: 4.0,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[12, 117, 23, 0b00000001, 6, 20]).unwrap()
    );
}

#[test]
fn deserialise_rev_2_9() {
    let expected_output = Uplink {
        k_p: 5.3,
        k_i: 0.54,
        k_d: 0.0,
        integral_unwind: IntegralUnwind::Zero,
        closed_percent: 0,
        k_d_when_closed: 0.0,
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
        k_d: 0.0,
        integral_unwind: IntegralUnwind::BackCalculate,
        closed_percent: 0,
        k_d_when_closed: 0.0,
    };

    assert_eq!(expected_output, Uplink::deserialise(&[35, 30]).unwrap());
}
