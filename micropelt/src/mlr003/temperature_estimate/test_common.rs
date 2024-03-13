use super::*;

#[test]
fn test_bin_to_f32_to_bin() {
    for u in 0..=255 {
        assert_eq!(
            u,
            float_point_two_five_to_bin_complement_of_two(
                bin_to_float_point_two_five_complement_of_two(u)
            )
            .unwrap()
        );
    }
}

#[test]
fn test_f32_to_bin_to_f32() {
    for i in -128..=127 {
        let f = i as f32 * 0.25;
        assert_eq!(
            f,
            bin_to_float_point_two_five_complement_of_two(
                float_point_two_five_to_bin_complement_of_two(f).unwrap()
            )
        );
    }
}
