use super::*;

use base64::Engine as _;

use crate::lorawan::Downlink as _;

#[test]
fn serialise_r_zero() {
    let downlink = DownlinkR {
        user_value: SetValue::ValvePosition(0),
        safety_value: SetValue::AmbientTemperature(0.0),
        flow_sensor_offset: 0,
        ..super::DownlinkR::default()
    };

    let bytes = downlink.serialise().unwrap().payload;

    assert_eq!(vec![0; 6], bytes);
}

#[test]
fn serialise_r_01() {
    let downlink = DownlinkR {
        user_value: SetValue::AmbientTemperature(18.5),
        room_temperature: 17.0,
        safety_value: SetValue::ValvePosition(65),
        radio_communication_interval: RadioCommunicationIntervalR::Minutes120,
        flow_sensor_offset: 4,
        reference_run: false,
    };

    let expected_output = vec![37, 68, 65, 58, 64, 0];

    assert_eq!(expected_output, downlink.serialise().unwrap().payload);
}

#[test]
fn serialise_r_02() {
    let downlink = DownlinkR {
        user_value: SetValue::FlowTemperature(57.5),
        room_temperature: 0.0,
        safety_value: SetValue::FlowTemperature(58.0),
        radio_communication_interval: RadioCommunicationIntervalR::Minutes480,
        flow_sensor_offset: 2,
        reference_run: false,
    };

    let bytes = downlink.serialise().unwrap().payload;

    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);

    assert_eq!("cwB0RSAA", b64);
}

#[test]
fn serialise_f_zero() {
    let downlink = DownlinkF {
        user_value: SetValue::ValvePosition(0),
        safety_value: SetValue::AmbientTemperature(0.0),
        radio_communication_interval: RadioCommunicationIntervalF::Minutes15,
        k_p: Kp::Kp12,
        reference_run: false,
    };

    let bytes = downlink.serialise().unwrap().payload;

    assert_eq!(vec![0; 6], bytes);
}

#[test]
fn serialise_f_01() {
    let downlink = DownlinkF {
        user_value: SetValue::FlowTemperature(57.5),
        safety_value: SetValue::FlowTemperature(58.0),
        radio_communication_interval: RadioCommunicationIntervalF::Minutes120,
        k_p: Kp::Kp4,
        reference_run: false,
    };

    let bytes = downlink.serialise().unwrap().payload;

    assert_eq!(vec![115, 0, 116, 0b00110101, 0, 0b01000000], bytes);
}

#[test]
fn serialise_4_zero() {
    let downlink = Downlink4 {
        user_value: SetValue::ValvePosition(0),
        room_temperature: 0.0,
        safety_value: SetValue::AmbientTemperature(0.0),
        radio_communication_interval: RadioCommunicationIntervalR::Minutes10,
    };

    let bytes = downlink.serialise().unwrap().payload;

    assert_eq!(vec![0; 4], bytes);
}

#[test]
fn serialise_4_01() {
    let downlink = Downlink4 {
        user_value: SetValue::AmbientTemperature(2.5),
        room_temperature: 16.75,
        safety_value: SetValue::FlowTemperature(76.5),
        radio_communication_interval: RadioCommunicationIntervalR::Minutes120,
    };

    let bytes = downlink.serialise().unwrap().payload;

    assert_eq!(vec![5, 67, 153, 0b00111001], bytes);
}

#[test]
fn test_float_point_two_five_to_bin() {
    assert_eq!(17, float_point_two_five_to_bin(4.25));
}

#[test]
fn test_offset_comp_to_bin() {
    assert_eq!(8, offset_comp_to_bin(-8).unwrap());
    assert_eq!(12, offset_comp_to_bin(-4).unwrap());
    assert_eq!(15, offset_comp_to_bin(-1).unwrap());
    assert_eq!(0, offset_comp_to_bin(0).unwrap());
    assert_eq!(1, offset_comp_to_bin(1).unwrap());
    assert_eq!(7, offset_comp_to_bin(7).unwrap());
    assert!(offset_comp_to_bin(22).is_err());
}
