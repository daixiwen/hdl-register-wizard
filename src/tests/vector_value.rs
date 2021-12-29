use super::super::mdf_format;
use std::str::FromStr;

// test creating VectorValue with string
#[test]
fn from_str() {
    assert_eq!(
        mdf_format::VectorValue {
            value: 50,
            radix: mdf_format::RadixType::Decimal,
        },
        mdf_format::VectorValue::from_str("50").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 38,
            radix: mdf_format::RadixType::Decimal,
        },
        mdf_format::VectorValue::from_str("0d38").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 38,
            radix: mdf_format::RadixType::Decimal,
        },
        mdf_format::VectorValue::from_str("0D38").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 48,
            radix: mdf_format::RadixType::Hexadecimal,
        },
        mdf_format::VectorValue::from_str("0x30").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 51,
            radix: mdf_format::RadixType::Hexadecimal,
        },
        mdf_format::VectorValue::from_str("0X33").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 9,
            radix: mdf_format::RadixType::Binary,
        },
        mdf_format::VectorValue::from_str("0b1001").unwrap()
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 11,
            radix: mdf_format::RadixType::Binary,
        },
        mdf_format::VectorValue::from_str("0B1011").unwrap()
    );

    // test parse error
    if mdf_format::VectorValue::from_str("abcd").is_ok() {
        panic!("should generate an error")
    }
}

// test converting VectorValue to string
#[test]
fn to_str() {
    assert_eq!(
        mdf_format::VectorValue {
            value: 50,
            radix: mdf_format::RadixType::Decimal,
        }
        .to_string(),
        "50"
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 51,
            radix: mdf_format::RadixType::Hexadecimal,
        }
        .to_string(),
        "0x33"
    );
    assert_eq!(
        mdf_format::VectorValue {
            value: 9,
            radix: mdf_format::RadixType::Binary,
        }
        .to_string(),
        "0b1001"
    );
}
