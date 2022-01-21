use crate::utils;
use crate::mdf_format;
use std::str::FromStr;

// test creating VectorValue with string
#[test]
fn from_str() {
    assert_eq!(
        mdf_format::Address::Auto,
        mdf_format::Address::from_str("auto").unwrap()
    );
    assert_eq!(
        mdf_format::Address::Single(utils::VectorValue {
            value: 38,
            radix: utils::RadixType::Decimal,
        }),
        mdf_format::Address::from_str("38").unwrap()
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal,
            },
            count: utils::VectorValue {
                value: 10,
                radix: utils::RadixType::Decimal,
            },
            increment: None
        }),
        mdf_format::Address::from_str("0x40:stride:10").unwrap()
    );
    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal,
            },
            count: utils::VectorValue {
                value: 4,
                radix: utils::RadixType::Decimal,
            },
            increment: None
        }),
        mdf_format::Address::from_str("0x40:stride:4").unwrap()
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal,
            },
            count: utils::VectorValue {
                value: 10,
                radix: utils::RadixType::Decimal,
            },
            increment: Some(utils::VectorValue {
                value: 4,
                radix: utils::RadixType::Hexadecimal,
            })
        }),
        mdf_format::Address::from_str("0x40:stride:10:0x4").unwrap()
    );

    // test parse error
    if mdf_format::Address::from_str("abcd").is_ok() {
        panic!("should generate an error")
    }
    if mdf_format::Address::from_str("0x40:bug:10:0x4").is_ok() {
        panic!("should generate an error")
    }
}

// test converting VectorValue to string
#[test]
fn to_str() {
    assert_eq!(mdf_format::Address::Auto.to_string(), "auto");
    assert_eq!(
        mdf_format::Address::Single(utils::VectorValue {
            value: 38,
            radix: utils::RadixType::Decimal,
        })
        .to_string(),
        "38"
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal,
            },
            count: utils::VectorValue {
                value: 10,
                radix: utils::RadixType::Decimal,
            },
            increment: None
        })
        .to_string(),
        "0x40:stride:10"
    );
    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal,
            },
            count: utils::VectorValue {
                value: 10,
                radix: utils::RadixType::Decimal,
            },
            increment: Some(utils::VectorValue {
                value: 4,
                radix: utils::RadixType::Hexadecimal,
            })
        })
        .to_string(),
        "0x40:stride:10:0x4"
    );
}
