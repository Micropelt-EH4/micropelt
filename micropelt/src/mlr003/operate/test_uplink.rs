use super::*;

use base64::Engine as _;

#[test]
fn uplink_partial_eq() {
    let uplink0 = Uplink {
        valve_position: 19,
        flow_raw_value: 36.5,
        flow_temperature: 41.5,
        ambient_raw_value: 22.25,
        ambient_temperature: 20.0,
        flow_sensor_error: false,
        ambient_sensor_error: false,
        battery_v: 2.76,
        battery_low: false,
        battery_high: true,
        average_current_consumed: 220,
        average_current_generated: 2120,
        harvesting: true,
        motor_error: false,
        radio_communication_error: false,
        radio_signal_strength_low: false,
        reference_run_complete: true,
        operating_condition_off: false,
    };

    let uplink1 = uplink0.clone();
    assert_eq!(uplink0, uplink1);

    let mut uplink2 = uplink0.clone();
    uplink2.battery_v = 2.765;
    assert_eq!(uplink0, uplink2);

    let mut uplink3 = uplink0.clone();
    uplink3.battery_v = 2.79;
    assert!(!uplink0.eq(&uplink3));
}

#[test]
fn deserialise_00() {
    let expected_output = Uplink {
        valve_position: 22,
        flow_raw_value: 49.5,
        flow_temperature: 53.5,
        ambient_raw_value: 29.25,
        ambient_temperature: 19.0,
        flow_sensor_error: false,
        ambient_sensor_error: false,
        battery_v: 2.58,
        battery_low: false,
        battery_high: false,
        average_current_consumed: 330,
        average_current_generated: 1820,
        harvesting: true,
        motor_error: false,
        radio_communication_error: true,
        radio_signal_strength_low: true,
        reference_run_complete: true,
        operating_condition_off: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[22, 99, 107, 117, 76, 38, 129, 33, 182, 16]).unwrap()
    );
}

#[test]
fn deserialise_01() {
    let expected_output = Uplink {
        valve_position: 33,
        flow_raw_value: 28.5,
        flow_temperature: 31.5,
        ambient_raw_value: 29.5,
        ambient_temperature: 26.5,
        flow_sensor_error: false,
        ambient_sensor_error: false,
        battery_v: 2.4,
        battery_low: false,
        battery_high: false,
        average_current_consumed: 2550,
        average_current_generated: 0,
        harvesting: false,
        motor_error: false,
        radio_communication_error: false,
        radio_signal_strength_low: false,
        reference_run_complete: true,
        operating_condition_off: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("ITk/dmoAeP8AEA==")
                .unwrap()
        )
        .unwrap()
    );
}

#[test]
fn deserialise_02() {
    let expected_output = Uplink {
        valve_position: 19,
        flow_raw_value: 47.5,
        flow_temperature: 50.5,
        ambient_raw_value: 36.5,
        ambient_temperature: 26.25,
        flow_sensor_error: false,
        ambient_sensor_error: false,
        battery_v: 2.74,
        battery_low: false,
        battery_high: true,
        average_current_consumed: 10,
        average_current_generated: 0,
        harvesting: false,
        motor_error: false,
        radio_communication_error: true,
        radio_signal_strength_low: true,
        reference_run_complete: true,
        operating_condition_off: false,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(
            &base64::engine::general_purpose::STANDARD
                .decode("E19lkmkGiQEAUA==")
                .unwrap()
        )
        .unwrap()
    );
}

#[test]
fn deserialise_something_afoot() {
    let expected_output = Uplink {
        valve_position: 100,
        flow_raw_value: 0.0,
        flow_temperature: 3.0,
        ambient_raw_value: 0.0,
        ambient_temperature: 0.0,
        flow_sensor_error: true,
        ambient_sensor_error: true,
        battery_v: 2.12,
        battery_low: true,
        battery_high: false,
        average_current_consumed: 480,
        average_current_generated: 0,
        harvesting: false,
        motor_error: true,
        radio_communication_error: true,
        radio_signal_strength_low: true,
        reference_run_complete: false,
        operating_condition_off: true,
    };

    assert_eq!(
        expected_output,
        Uplink::deserialise(&[100, 0, 6, 0, 0, 95, 106, 48, 0, 128]).unwrap()
    );
}
