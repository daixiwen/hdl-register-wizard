//! Tests for the address type

use crate::file_formats::mdf;
use crate::utils;
use std::str::FromStr;

/// test creating Address with string
#[test]
fn from_str() {
    assert_eq!(
        mdf::Address {
            value: None,
            stride: None
        },
        mdf::Address::from_str("auto").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 38,
                radix: utils::RadixType::Decimal
            }),
            stride: None
        },
        mdf::Address::from_str("38").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal
            }),
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: None
            })
        },
        mdf::Address::from_str("0x40:stride:10").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal
            }),
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Decimal
                },
                increment: None
            })
        },
        mdf::Address::from_str("0x40:stride:4").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal
            }),
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: Some(utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Hexadecimal
                })
            })
        },
        mdf::Address::from_str("0x40:stride:10:0x4").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: None,
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Decimal
                },
                increment: None
            })
        },
        mdf::Address::from_str("auto:stride:4").unwrap()
    );

    assert_eq!(
        mdf::Address {
            value: None,
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: Some(utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Hexadecimal
                })
            })
        },
        mdf::Address::from_str("auto:stride:10:0x4").unwrap()
    );

    // test parse error
    if mdf::Address::from_str("abcd").is_ok() {
        panic!("should generate an error")
    }
    if mdf::Address::from_str("0x40:bug:10:0x4").is_ok() {
        panic!("should generate an error")
    }
}

/// test converting Address to string
#[test]
fn to_str() {
    assert_eq!(
        mdf::Address {
            value: None,
            stride: None
        }
        .to_string(),
        "auto"
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 38,
                radix: utils::RadixType::Decimal
            }),
            stride: None
        }
        .to_string(),
        "38"
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal
            }),
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: None
            })
        }
        .to_string(),
        "0x40:stride:10"
    );

    assert_eq!(
        mdf::Address {
            value: Some(utils::VectorValue {
                value: 64,
                radix: utils::RadixType::Hexadecimal
            }),
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: Some(utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Hexadecimal
                })
            })
        }
        .to_string(),
        "0x40:stride:10:0x4"
    );

    assert_eq!(
        mdf::Address {
            value: None,
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: None
            })
        }
        .to_string(),
        "auto:stride:10"
    );

    assert_eq!(
        mdf::Address {
            value: None,
            stride: Some(mdf::AddressStride {
                count: utils::VectorValue {
                    value: 10,
                    radix: utils::RadixType::Decimal
                },
                increment: Some(utils::VectorValue {
                    value: 4,
                    radix: utils::RadixType::Hexadecimal
                })
            })
        }
        .to_string(),
        "auto:stride:10:0x4"
    );
}
