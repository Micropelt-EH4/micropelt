use super::*;

#[test]
fn to_and_fro() {
    assert_eq!(
        TravelDistance::Um1456,
        TravelDistance::from_bin(TravelDistance::Um1456.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um1664,
        TravelDistance::from_bin(TravelDistance::Um1664.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um1872,
        TravelDistance::from_bin(TravelDistance::Um1872.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um2080,
        TravelDistance::from_bin(TravelDistance::Um2080.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um2288,
        TravelDistance::from_bin(TravelDistance::Um2288.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um2496,
        TravelDistance::from_bin(TravelDistance::Um2496.to_bin()).unwrap()
    );
    assert_eq!(
        TravelDistance::Um2560Point48,
        TravelDistance::from_bin(TravelDistance::Um2560Point48.to_bin()).unwrap()
    );
}

#[test]
fn range() {
    assert!(TravelDistance::from_bin(1).is_err());
    assert!(TravelDistance::from_bin(2).is_err());
    assert!(TravelDistance::from_bin(3).is_ok());
    assert!(TravelDistance::from_bin(13).is_err());
    assert!(TravelDistance::from_bin(130).is_err());
}
