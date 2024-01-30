use super::*;

#[test]
fn to_and_fro() {
    assert_eq!(
        DataRate::Sf7Bw125,
        DataRate::from_bin(DataRate::Sf7Bw125.to_bin()).unwrap()
    );
    assert_eq!(
        DataRate::Sf8Bw125,
        DataRate::from_bin(DataRate::Sf8Bw125.to_bin()).unwrap()
    );
}

#[test]
fn from_bin_is_err() {
    assert!(DataRate::from_bin(2).is_err());
    assert!(DataRate::from_bin(5).is_err());
    assert!(DataRate::from_bin(210).is_err());
}
