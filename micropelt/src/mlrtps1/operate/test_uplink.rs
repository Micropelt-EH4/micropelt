use super::*;

use base64::Engine as _;

#[test]
fn uplink_partial_eq() {
    let uplink0 = Uplink {
        ambient_temperature: 23.25,
        set_point_temperature: SetPointTemperature::Offset(1),
        battery_v: 3.8,
        pir_status: PirStatus::MotionDetected,
        ambient_sensor_error: true,
        pir_sensor_error: false,
        radio_communication_error: true,
        radio_signal_strength_low: true,
        battery_low: false,
    };
    let uplink1 = uplink0.clone();
    assert_eq!(uplink0, uplink1);
}

#[test]
fn deserialise_00() {
    let expected_output = Uplink {
        ambient_temperature: 0.0,
        set_point_temperature: SetPointTemperature::Offset(0),
        battery_v: 0.0,
        pir_status: PirStatus::NoMotionDetected,
        ambient_sensor_error: false,
        pir_sensor_error: false,
        radio_communication_error: false,
        radio_signal_strength_low: false,
        battery_low: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("AAAAAA==")
                .unwrap()
        )
        .unwrap()
    )
}

#[test]
fn deserialise_01() {
    let expected_output = Uplink {
        ambient_temperature: 18.75,
        set_point_temperature: SetPointTemperature::Offset(-3),
        battery_v: 3.28,
        pir_status: PirStatus::MotionDetected,
        ambient_sensor_error: false,
        pir_sensor_error: false,
        radio_communication_error: false,
        radio_signal_strength_low: false,
        battery_low: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("SyCkDQ==")
                .unwrap()
        )
        .unwrap()
    )
}

#[test]
fn deserialise_02() {
    let expected_output = Uplink {
        ambient_temperature: 61.0,
        set_point_temperature: SetPointTemperature::FreezeProtect,
        battery_v: 0.6,
        pir_status: PirStatus::NoMotionDetected,
        ambient_sensor_error: true,
        pir_sensor_error: false,
        radio_communication_error: false,
        radio_signal_strength_low: true,
        battery_low: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("9AUe/w==")
                .unwrap()
        )
        .unwrap()
    )
}
