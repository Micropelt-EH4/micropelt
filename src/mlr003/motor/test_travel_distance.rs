use super::*;

#[test]
fn to_and_fro() {
    assert_eq!(
        TravelDistance::Nm1456,
        TravelDistance::from_bin(TravelDistance::Nm1456.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm1664,
        TravelDistance::from_bin(TravelDistance::Nm1664.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm1872,
        TravelDistance::from_bin(TravelDistance::Nm1872.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm2080,
        TravelDistance::from_bin(TravelDistance::Nm2080.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm2288,
        TravelDistance::from_bin(TravelDistance::Nm2288.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm2496,
        TravelDistance::from_bin(TravelDistance::Nm2496.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Nm2560Point48,
        TravelDistance::from_bin(TravelDistance::Nm2560Point48.to_bin()).unwrap()
    );
}

#[test]
fn from_bin_is_err() {
    assert!(TravelDistance::from_bin(1).is_err());
    assert!(TravelDistance::from_bin(6).is_err());
    assert!(TravelDistance::from_bin(13).is_err());
    assert!(TravelDistance::from_bin(130).is_err());
}
