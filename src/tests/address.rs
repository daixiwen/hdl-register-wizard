use super::super::mdf_format;
use std::str::FromStr;

// test creating VectorValue with string
#[test]
fn from_str() {
    assert_eq!(
        mdf_format::Address::Auto,
        mdf_format::Address::from_str("auto").unwrap()
    );
    assert_eq!(
        mdf_format::Address::Single(mdf_format::VectorValue {
            value: 38,
            radix: mdf_format::RadixType::Decimal,
        }),
        mdf_format::Address::from_str("38").unwrap()
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: mdf_format::VectorValue {
                value: 64,
                radix: mdf_format::RadixType::Hexadecimal,
            },
            count: mdf_format::VectorValue {
                value: 10,
                radix: mdf_format::RadixType::Decimal,
            },
            increment: None
        }),
        mdf_format::Address::from_str("0x40:stride:10").unwrap()
    );
    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: mdf_format::VectorValue {
                value: 64,
                radix: mdf_format::RadixType::Hexadecimal,
            },
            count: mdf_format::VectorValue {
                value: 4,
                radix: mdf_format::RadixType::Decimal,
            },
            increment: None
        }),
        mdf_format::Address::from_str("0x40:stride:4").unwrap()
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: mdf_format::VectorValue {
                value: 64,
                radix: mdf_format::RadixType::Hexadecimal,
            },
            count: mdf_format::VectorValue {
                value: 10,
                radix: mdf_format::RadixType::Decimal,
            },
            increment: Some(mdf_format::VectorValue {
                value: 4,
                radix: mdf_format::RadixType::Hexadecimal,
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
        mdf_format::Address::Single(mdf_format::VectorValue {
            value: 38,
            radix: mdf_format::RadixType::Decimal,
        })
        .to_string(),
        "38"
    );

    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: mdf_format::VectorValue {
                value: 64,
                radix: mdf_format::RadixType::Hexadecimal,
            },
            count: mdf_format::VectorValue {
                value: 10,
                radix: mdf_format::RadixType::Decimal,
            },
            increment: None
        })
        .to_string(),
        "0x40:stride:10"
    );
    assert_eq!(
        mdf_format::Address::Stride(mdf_format::AddressStride {
            value: mdf_format::VectorValue {
                value: 64,
                radix: mdf_format::RadixType::Hexadecimal,
            },
            count: mdf_format::VectorValue {
                value: 10,
                radix: mdf_format::RadixType::Decimal,
            },
            increment: Some(mdf_format::VectorValue {
                value: 4,
                radix: mdf_format::RadixType::Hexadecimal,
            })
        })
        .to_string(),
        "0x40:stride:10:0x4"
    );
}
