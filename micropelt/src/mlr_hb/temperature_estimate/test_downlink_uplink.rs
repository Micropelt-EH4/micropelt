use super::*;

use super::super::downlink::float_plus_minus_point_zero_one_to_bin;

#[test]
fn complement_of_two() {
    assert_close(0.0, bin_to_float_plus_minus_point_zero_one(0));
    assert_close(0.01, bin_to_float_plus_minus_point_zero_one(1));
    assert_close(0.02, bin_to_float_plus_minus_point_zero_one(2));
    assert_close(1.26, bin_to_float_plus_minus_point_zero_one(126));
    assert_close(1.27, bin_to_float_plus_minus_point_zero_one(127));
    assert_close(-1.28, bin_to_float_plus_minus_point_zero_one(128));
    assert_close(-1.27, bin_to_float_plus_minus_point_zero_one(129));
    assert_close(-0.01, bin_to_float_plus_minus_point_zero_one(255));

    assert_eq!(0, float_plus_minus_point_zero_one_to_bin(0.0).unwrap());
    assert_eq!(1, float_plus_minus_point_zero_one_to_bin(0.01).unwrap());
    assert_eq!(2, float_plus_minus_point_zero_one_to_bin(0.02).unwrap());
    assert_eq!(126, float_plus_minus_point_zero_one_to_bin(1.26).unwrap());
    assert_eq!(127, float_plus_minus_point_zero_one_to_bin(1.27).unwrap());
    assert_eq!(128, float_plus_minus_point_zero_one_to_bin(-1.28).unwrap());
    assert_eq!(129, float_plus_minus_point_zero_one_to_bin(-1.27).unwrap());
    assert_eq!(255, float_plus_minus_point_zero_one_to_bin(-0.01).unwrap());
}

fn assert_close(expected: f32, actual: f32) {
    assert!((expected - actual).abs() < 0.0000001);
}
