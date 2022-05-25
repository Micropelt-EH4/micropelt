use super::*;

use crate::lorawan::Downlink as LorawanDownlink;

#[test]
fn serialize_zero() {
    let downlink = Downlink {
        user_value: SetValue::ValvePosition(0),
        safety_value: SetValue::AmbientTemperature(0.0),
        flow_sensor_offset: 0,
        ..super::Downlink::default_radiator()
    };

    let bytes = downlink.serialize().unwrap().payload;

    for byte in bytes {
        assert_eq!(0, byte);
    }
}

#[test]
fn serialize_1() {
    let downlink = Downlink {
        user_value: SetValue::AmbientTemperature(18.5),
        room_temperature: 17.0,
        safety_value: SetValue::ValvePosition(65),
        radio_communication_interval: RadioCommunicationInterval::Minutes120,
        flow_sensor_offset: 4,
        p_controller_gain: 2,
        reference_run: false,
    };

    let bytes = downlink.serialize().unwrap().payload;

    let expected_output = vec![37, 68, 65, 58, 64, 96];

    assert_eq!(expected_output.len(), bytes.len());

    for i in 0..expected_output.len() {
        assert_eq!(expected_output[i], bytes[i]);
    }

    assert_eq!(expected_output, downlink.serialize().unwrap().payload);
}

#[test]
fn serialize_2() {
    let downlink = Downlink {
        user_value: SetValue::FlowTemperature(57.5),
        room_temperature: 0.0,
        safety_value: SetValue::FlowTemperature(58.0),
        radio_communication_interval: RadioCommunicationInterval::Minutes480,
        flow_sensor_offset: 2,
        p_controller_gain: 1,
        reference_run: false,
    };

    let bytes = downlink.serialize().unwrap().payload;

    let b64 = base64::encode(bytes);

    assert_eq!("cwB0RSBA", b64);
}

#[test]
fn test_set_value_to_bin() {
    assert_eq!(19, set_value_to_bin(&SetValue::ValvePosition(19)));
    assert_eq!(98, set_value_to_bin(&SetValue::FlowTemperature(49.0)));
    assert_eq!(39, set_value_to_bin(&SetValue::AmbientTemperature(19.5)));
}

#[test]
fn test_float_point_two_five_to_bin() {
    assert_eq!(17, float_point_two_five_to_bin(4.25));
}

#[test]
fn test_float_point_five_to_bin() {
    assert_eq!(28, float_point_five_to_bin(14.0));
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
