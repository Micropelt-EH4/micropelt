use super::*;

#[test]
fn version_to_string() {
    let version = Version { major: 8, minor: 3 };

    assert_eq!("8.3", version.to_string());
}

#[test]
fn firmware_version_to_string() {
    let version = FirmwareVersion {
        year_since_2000: 98,
        month: 7,
        day: 6,
        minor: 5,
    };

    assert_eq!("2098.07.06.05", version.to_string());

    let version = FirmwareVersion {
        year_since_2000: 0,
        month: 12,
        day: 25,
        minor: 0,
    };

    assert_eq!("2000.12.25.00", version.to_string());
}

#[test]
fn deserialise() {
    let input = [169, 108, 210, 9, 3, 24];

    let uplink = Uplink::deserialise(&input).unwrap();

    assert_eq!(
        Version {
            major: 9,
            minor: 10
        },
        uplink.revision
    );
    assert_eq!(
        Version {
            major: 12,
            minor: 6
        },
        uplink.hardware
    );
    assert_eq!(
        FirmwareVersion {
            year_since_2000: 210,
            month: 9,
            day: 3,
            minor: 24,
        },
        uplink.firmware,
    );
}

#[test]
fn test_bin_to_version() {
    assert_eq!(
        Version {
            major: 11,
            minor: 7
        },
        bin_to_version(123)
    );
}

#[test]
fn bin_to_month_ok() {
    assert_eq!(6, bin_to_month(6).unwrap());
}

#[test]
fn bin_to_month_err() {
    assert!(bin_to_month(0).is_err());
    assert!(bin_to_month(13).is_err());
    assert!(bin_to_month(200).is_err());
}

#[test]
fn bin_to_day_ok() {
    assert_eq!(1, bin_to_day(12, 1).unwrap());
}

#[test]
fn bin_to_day_err() {
    assert!(bin_to_day(0, 1).is_err());
    assert!(bin_to_day(1, 0).is_err());
    assert!(bin_to_day(1, 32).is_err());
    assert!(bin_to_day(2, 30).is_err());
    assert!(bin_to_day(4, 31).is_err());
}

#[test]
fn month_get_n_days_err() {
    assert!(month_get_n_days(0).is_err());
    assert!(month_get_n_days(13).is_err());
    assert!(month_get_n_days(17).is_err());
}
