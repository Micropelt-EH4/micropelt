use super::*;

#[test]
fn display() {
    assert_eq!("FP", SetPointTemperature::FreezeProtect.to_string());
    assert_eq!("-4", SetPointTemperature::Offset(-4).to_string());
    assert_eq!(" 0", SetPointTemperature::Offset(0).to_string());
    assert_eq!("+3", SetPointTemperature::Offset(3).to_string());
}
