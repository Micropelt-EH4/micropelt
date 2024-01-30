use super::*;

#[test]
fn test_set_value_to_bin() {
    assert_eq!(19, SetValue::ValvePosition(19).value_to_bin().unwrap());
    assert_eq!(98, SetValue::FlowTemperature(49.0).value_to_bin().unwrap());
    assert_eq!(
        39,
        SetValue::AmbientTemperature(19.5).value_to_bin().unwrap()
    );
}

#[test]
fn test_float_point_five_to_bin() {
    assert_eq!(28, float_point_five_to_bin(14.0, 35.0).unwrap());
}
