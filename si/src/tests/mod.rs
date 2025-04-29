use crate::{scale::SIScale, si::SINumber, si::SIUnit};

#[test]
fn scale_factor_test(){
    assert_eq!(SIScale::Milli * 1.123456, 0.001123456);
}

#[test]
fn si_parse_test(){
    // Metre/millimetre edge case
    assert_eq!(
        SINumber::from_str("1m", SIUnit::Metre).unwrap(), 
        SINumber::new(1.0, SIScale::Milli, SIUnit::Metre)
    );
    assert_eq!(
        SINumber::from_str("1mm", SIUnit::Metre).unwrap(), 
        SINumber::new(1.0, SIScale::Milli, SIUnit::Metre)
    );

    // Smaller units
    assert_eq!(
        SINumber::from_str("1um", SIUnit::Metre).unwrap(), 
        SINumber::new(1.0, SIScale::Micro, SIUnit::Metre)
    );
    assert_eq!(
        SINumber::from_str("1nm", SIUnit::Metre).unwrap(), 
        SINumber::new(1.0, SIScale::Nano, SIUnit::Metre)
    );
    assert_eq!(
        SINumber::from_str("1fm", SIUnit::Metre).unwrap(), 
        SINumber::new(1.0, SIScale::Femto, SIUnit::Metre)
    );
}