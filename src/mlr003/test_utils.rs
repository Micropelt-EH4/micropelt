use super::*;

#[test]
fn test_close() {
    assert!(close(1.0, 1.0, 1.0));
    assert!(close(2.4, 2.4009, 0.02));
    assert!(!close(9.5, 10.0, 0.5));
}
