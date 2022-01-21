use crate::utils;
use std::str::FromStr;

// test creating VectorValue with string
#[test]
fn from_str() {
    assert_eq!(
        utils::VectorValue {
            value: 50,
            radix: utils::RadixType::Decimal,
        },
        utils::VectorValue::from_str("50").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 38,
            radix: utils::RadixType::Decimal,
        },
        utils::VectorValue::from_str("0d38").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 38,
            radix: utils::RadixType::Decimal,
        },
        utils::VectorValue::from_str("0D38").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 48,
            radix: utils::RadixType::Hexadecimal,
        },
        utils::VectorValue::from_str("0x30").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 51,
            radix: utils::RadixType::Hexadecimal,
        },
        utils::VectorValue::from_str("0X33").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 9,
            radix: utils::RadixType::Binary,
        },
        utils::VectorValue::from_str("0b1001").unwrap()
    );
    assert_eq!(
        utils::VectorValue {
            value: 11,
            radix: utils::RadixType::Binary,
        },
        utils::VectorValue::from_str("0B1011").unwrap()
    );

    // test parse error
    if utils::VectorValue::from_str("abcd").is_ok() {
        panic!("should generate an error")
    }
}

// test converting VectorValue to string
#[test]
fn to_str() {
    assert_eq!(
        utils::VectorValue {
            value: 50,
            radix: utils::RadixType::Decimal,
        }
        .to_string(),
        "50"
    );
    assert_eq!(
        utils::VectorValue {
            value: 51,
            radix: utils::RadixType::Hexadecimal,
        }
        .to_string(),
        "0x33"
    );
    assert_eq!(
        utils::VectorValue {
            value: 9,
            radix: utils::RadixType::Binary,
        }
        .to_string(),
        "0b1001"
    );
}
