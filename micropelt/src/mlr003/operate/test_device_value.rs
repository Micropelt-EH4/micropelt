use super::*;

#[test]
fn display() {
    assert_eq!(
        DeviceValue::User(SetValue::FlowTemperature(59.5)).to_string(),
        SetValue::FlowTemperature(59.5).to_string()
    );
}

#[test]
fn partial_equal() {
    assert_eq!(
        DeviceValue::User(SetValue::AmbientTemperature(23.0)),
        DeviceValue::User(SetValue::AmbientTemperature(23.0))
    );
    assert!(
        DeviceValue::User(SetValue::AmbientTemperature(23.0))
            != DeviceValue::User(SetValue::AmbientTemperature(23.5))
    );
    assert!(
        DeviceValue::User(SetValue::AmbientTemperature(23.0))
            != DeviceValue::User(SetValue::FlowTemperature(23.0))
    );
    assert_eq!(
        DeviceValue::DetectingOpeningPoint(26.75),
        DeviceValue::DetectingOpeningPoint(26.75)
    );
    assert_eq!(
        DeviceValue::SlowHarvesting(24.25),
        DeviceValue::SlowHarvesting(24.25)
    );
    assert!(
        DeviceValue::User(SetValue::ValvePosition(25)) != DeviceValue::DetectingOpeningPoint(25.0)
    );
    assert!(
        DeviceValue::User(SetValue::FlowTemperature(25.0)) != DeviceValue::SlowHarvesting(25.0)
    );
    assert!(DeviceValue::DetectingOpeningPoint(25.0) != DeviceValue::SlowHarvesting(25.0));
}

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
