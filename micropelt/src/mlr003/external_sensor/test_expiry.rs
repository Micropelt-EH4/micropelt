use super::*;

#[test]
fn bin_to_expiry_to_bin() {
    for b in 0..=255 {
        assert_eq!(b, Expiry::from_bin(b).to_bin());
    }
}

#[test]
fn display_minutes_or_hours() {
    assert_eq!(
        String::from("until 35 minutes pass"),
        Expiry::N5Minutes(7).to_string(),
    );

    assert_eq!(
        String::from("until 1 hour passes"),
        Expiry::N5Minutes(12).to_string(),
    );

    assert_eq!(
        String::from("until 2 hours pass"),
        Expiry::N5Minutes(24).to_string(),
    );

    assert_eq!(
        String::from("until 235 minutes pass"),
        Expiry::N5Minutes(47).to_string(),
    );

    assert_eq!(
        String::from("until 4 hours pass"),
        Expiry::N5Minutes(48).to_string(),
    );
}
