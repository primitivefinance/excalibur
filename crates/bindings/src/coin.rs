pub use coin::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod coin {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialSupply"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@\x90\x80\x82R4b\0\x04\xE8WPb\0\x11\x1F\x90\x818\x03\x91\x82\x82Q\x93\x84\x92b\0\0)\x82\x85b\0\x05gV[\x839` \x93\x84\x91\x81\x01\x03\x12b\0\x04\x99WQ\x81Q\x90b\0\0H\x82b\0\x055V[`\x04\x93\x84\x83Rc!\xB7\xB4\xB7`\xE1\x1B\x81\x84\x01R\x83Q\x91b\0\0h\x83b\0\x055V[\x85\x83Rc!\xA7\xA4\xA7`\xE1\x1B\x82\x84\x01R\x83Q`\x01`\x01`@\x1B\x03\x94\x90\x93\x90\x85\x85\x11b\0\x04\x84W`\0\x94\x80b\0\0\x9D\x87Tb\0\x05\x8BV[\x92`\x1F\x93\x84\x81\x11b\0\x043W[P\x86\x90\x84\x83\x11`\x01\x14b\0\x03\xCBW\x88\x92b\0\x03\xBFW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x85U[\x81Q\x90\x86\x82\x11b\0\x03\xACW\x81\x90`\x01\x93b\0\0\xF1\x85Tb\0\x05\x8BV[\x82\x81\x11b\0\x03WW[P\x86\x91\x83\x11`\x01\x14b\0\x02\xF3W\x87\x92b\0\x02\xE7W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\x12`\x80RF`\xA0R\x85Q\x84T\x91\x81\x86b\0\x01>\x85b\0\x05\x8BV[\x92\x83\x83R\x87\x83\x01\x95\x88\x82\x82\x16\x91\x82`\0\x14b\0\x02\xC7WPP`\x01\x14b\0\x02\x87W[Pb\0\x01n\x92P\x03\x82b\0\x05gV[Q\x90 \x85Q\x83\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R\x87\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81R`\xC0\x81\x01\x95\x81\x87\x10\x90\x87\x11\x17b\0\x02tW\x85\x87RQ\x90 `\xC0R`\x02T\x81\x81\x01\x80\x91\x11b\0\x02aW\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x92\x91`\x02U3\x84R`\x03\x82R\x85\x84 \x81\x81T\x01\x90U\x84R3\x93\xA3Qa\x0BV\x90\x81b\0\x05\xC9\x829`\x80Q\x81a\x05\xA3\x01R`\xA0Q\x81a\t~\x01R`\xC0Q\x81a\t\xA5\x01R\xF3[cNH{q`\xE0\x1B\x84R`\x11\x87R`$\x84\xFD[cNH{q`\xE0\x1B\x85R`A\x88R`$\x85\xFD[\x87\x91P\x88\x80R\x81\x89 \x90\x89\x91[\x85\x83\x10b\0\x02\xAEWPPb\0\x01n\x93P\x82\x01\x018b\0\x01_V[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x89\x93\x90\x92\x01\x91\x81\x01b\0\x02\x94V[`\xFF\x19\x16\x88Rb\0\x01n\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pb\0\x01_\x90PV[\x01Q\x90P8\x80b\0\x01\x0FV[\x84\x88R\x86\x88 \x85\x94P\x91\x90`\x1F\x19\x84\x16\x89[\x89\x82\x82\x10b\0\x03@WPP\x84\x11b\0\x03&W[PPP\x81\x1B\x01\x81Ub\0\x01#V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x03\x18V[\x83\x85\x01Q\x86U\x88\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01b\0\x03\x05V[\x90\x91\x92P\x84\x88R\x86\x88 \x83\x80\x86\x01`\x05\x1C\x82\x01\x92\x89\x87\x10b\0\x03\xA2W[\x91\x86\x95\x88\x92\x95\x94\x93\x01`\x05\x1C\x01\x91[\x82\x81\x10b\0\x03\x93WPPb\0\0\xFAV[\x8A\x81U\x86\x95P\x87\x91\x01b\0\x03\x83V[\x92P\x81\x92b\0\x03tV[cNH{q`\xE0\x1B\x86R`A\x89R`$\x86\xFD[\x01Q\x90P8\x80b\0\0\xC0V[\x88\x80R\x87\x89 \x92P`\x1F\x19\x84\x16\x89[\x89\x82\x82\x10b\0\x04\x1CWPP\x90\x84`\x01\x95\x94\x93\x92\x10b\0\x04\x02W[PPP\x81\x1B\x01\x85Ub\0\0\xD5V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x03\xF4V[`\x01\x85\x96\x82\x93\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01b\0\x03\xDAV[\x90\x91P\x87\x80R\x86\x88 \x84\x80\x85\x01`\x05\x1C\x82\x01\x92\x89\x86\x10b\0\x04zW[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10b\0\x04kWPb\0\0\xAAV[\x89\x81U\x84\x93P`\x01\x01b\0\x04\\V[\x92P\x81\x92b\0\x04OV[`A\x88cNH{q`\xE0\x1B`\0RR`$`\0\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17b\0\x05QW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17b\0\x05QW`@RV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15b\0\x05\xBDW[` \x83\x10\x14b\0\x05\xA7WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91b\0\x05\x9BV\xFE`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x07%W\x80c\t^\xA7\xB3\x14a\x06\xB6W\x80c\x18\x16\r\xDD\x14a\x06\x99W\x80c#\xB8r\xDD\x14a\x05\xC7W\x80c1<\xE5g\x14a\x05\x8BW\x80c6D\xE5\x15\x14a\x05iW\x80cp\xA0\x821\x14a\x051W\x80c~\xCE\xBE\0\x14a\x04\xF9W\x80c\x95\xD8\x9BA\x14a\x04\x17W\x80c\xA9\x05\x9C\xBB\x14a\x03\x93W\x80c\xD5\x05\xAC\xCF\x14a\x01TWc\xDDb\xED>\x14a\x01\x02WPa\0\x11V[\x914a\x01OW\x81`\x03\x196\x01\x12a\x01JW` \x92\x82\x91a\x01 a\t%V[a\x01(a\t@V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x08\x1AV[a\x07\xCAV[P\x904a\x01OW`\xE06`\x03\x19\x01\x12a\x01JWa\x01oa\t%V[\x90a\x01xa\t@V[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\x8FWB\x85\x10a\x03LWa\x01\x9Ea\tyV[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x05\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x038W\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03%W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x03\x1BW\x86Q\x16\x96\x87\x15\x15\x80a\x03\x12W[\x15a\x02\xE0W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\x9DV[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P\x904a\x01OW\x80`\x03\x196\x01\x12a\x01JW` \x91a\x03\xB0a\t%V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x03\xCA\x84\x82Ta\tVV[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x85\x90\xA3Q`\x01\x81R\xF3[P4a\x01OW\x80`\x03\x196\x01\x12a\x01JW\x81Q\x90\x80`\x01\x80T\x90a\x04:\x82a\x08jV[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x04\xCCWP`\x01\x14a\x04tW[a\x04p\x86\x88a\x04f\x82\x89\x03\x83a\x08\xA4V[Q\x91\x82\x91\x82a\x08\xDCV[\x03\x90\xF3[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x04\xB9WPPPP\x81\x01` \x01a\x04f\x82a\x04p\x86a\x04UV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x04\x9CV[\x90Pa\x04p\x97\x95P\x86\x93P` \x92Pa\x04f\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86a\x04UV[P\x904a\x01OW` 6`\x03\x19\x01\x12a\x01JW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05!a\t%V[\x16\x81R`\x05\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01OW` 6`\x03\x19\x01\x12a\x01JW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05Ya\t%V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90a\x05\x84a\tyV[\x90Q\x90\x81R\xF3[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01OW``6`\x03\x19\x01\x12a\x01JWa\x05\xE1a\t%V[\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEFa\x06\na\t@V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\x06vW[PPP\x86\x88R`\x03\x85R\x82\x88 a\x06W\x85\x82Ta\tVV[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\x06\x7F\x91a\tVV[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\x06?V[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x01OW\x81`\x03\x196\x01\x12a\x01JW` \x92a\x06\xD3a\t%V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P4a\x07\xCAW\x80`\x03\x196\x01\x12a\x01JW\x81Q\x90\x80\x80T\x90a\x07F\x82a\x08jV[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x04\xCCWP`\x01\x14a\x07sWa\x04p\x86\x88a\x04f\x82\x89\x03\x83a\x08\xA4V[\x80\x80\x95PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x83\x85\x10a\x07\xB7WPPPP\x81\x01` \x01a\x04f\x82a\x04p\x86a\x04UV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x07\x9AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x08\x9AW[` \x83\x10\x14a\x08\x84WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x08yV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\xC6W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\t\x11WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\x08\xEFV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t;WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t;WV[\x91\x90\x82\x03\x91\x82\x11a\tcWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\t\xC7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x82\x91a\t\xD7\x82a\x08jV[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87`\x01\x82\x16\x91\x82`\0\x14a\x0B\x02WPP`\x01\x14a\n\xA9W[Pa\n\n\x92P\x03\x82a\x08\xA4V[Q\x90 \x91`@Q\x91\x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x83\x01RF`\x80\x83\x01R0`\xA0\x83\x01R`\xA0\x82R`\xC0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\n\x95WP`@RQ\x90 \x90V[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x87\x80R\x86\x91P\x87\x90\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x85\x83\x10a\n\xEAWPPa\n\n\x93P\x82\x01\x018a\t\xFDV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\n\xD3V[`\xFF\x19\x16\x88Ra\n\n\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\t\xFD\x90PV\xFE\xA2dipfsX\"\x12 \x19q\x97\xEAS\x08\xE45\xA4\x82\xE0\xA9\xA5\t\x1D\xB5#\xC9\x19\xCC\xEE\x1D\x0BCn\xAD\xDCb\x8F\x86zLdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static COIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x90\x816\x10\x15a\0rW[\x90` `\x84\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R\xFD[`\0\x805`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x07%W\x80c\t^\xA7\xB3\x14a\x06\xB6W\x80c\x18\x16\r\xDD\x14a\x06\x99W\x80c#\xB8r\xDD\x14a\x05\xC7W\x80c1<\xE5g\x14a\x05\x8BW\x80c6D\xE5\x15\x14a\x05iW\x80cp\xA0\x821\x14a\x051W\x80c~\xCE\xBE\0\x14a\x04\xF9W\x80c\x95\xD8\x9BA\x14a\x04\x17W\x80c\xA9\x05\x9C\xBB\x14a\x03\x93W\x80c\xD5\x05\xAC\xCF\x14a\x01TWc\xDDb\xED>\x14a\x01\x02WPa\0\x11V[\x914a\x01OW\x81`\x03\x196\x01\x12a\x01JW` \x92\x82\x91a\x01 a\t%V[a\x01(a\t@V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[a\x08\x1AV[a\x07\xCAV[P\x904a\x01OW`\xE06`\x03\x19\x01\x12a\x01JWa\x01oa\t%V[\x90a\x01xa\t@V[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03\x8FWB\x85\x10a\x03LWa\x01\x9Ea\tyV[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x05\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x038W\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x03%W\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x03\x1BW\x86Q\x16\x96\x87\x15\x15\x80a\x03\x12W[\x15a\x02\xE0W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02\x9DV[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P\x904a\x01OW\x80`\x03\x196\x01\x12a\x01JW` \x91a\x03\xB0a\t%V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x03\xCA\x84\x82Ta\tVV[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x85\x90\xA3Q`\x01\x81R\xF3[P4a\x01OW\x80`\x03\x196\x01\x12a\x01JW\x81Q\x90\x80`\x01\x80T\x90a\x04:\x82a\x08jV[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x04\xCCWP`\x01\x14a\x04tW[a\x04p\x86\x88a\x04f\x82\x89\x03\x83a\x08\xA4V[Q\x91\x82\x91\x82a\x08\xDCV[\x03\x90\xF3[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x04\xB9WPPPP\x81\x01` \x01a\x04f\x82a\x04p\x86a\x04UV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x04\x9CV[\x90Pa\x04p\x97\x95P\x86\x93P` \x92Pa\x04f\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86a\x04UV[P\x904a\x01OW` 6`\x03\x19\x01\x12a\x01JW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05!a\t%V[\x16\x81R`\x05\x84R T\x90Q\x90\x81R\xF3[P\x904a\x01OW` 6`\x03\x19\x01\x12a\x01JW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05Ya\t%V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90a\x05\x84a\tyV[\x90Q\x90\x81R\xF3[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\x01OW``6`\x03\x19\x01\x12a\x01JWa\x05\xE1a\t%V[\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEFa\x06\na\t@V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\x06vW[PPP\x86\x88R`\x03\x85R\x82\x88 a\x06W\x85\x82Ta\tVV[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\x06\x7F\x91a\tVV[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\x06?V[P4a\x01OW6`\x03\x19\x01\x12a\x01JW` \x90`\x02T\x90Q\x90\x81R\xF3[P\x914a\x01OW\x81`\x03\x196\x01\x12a\x01JW` \x92a\x06\xD3a\t%V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[P4a\x07\xCAW\x80`\x03\x196\x01\x12a\x01JW\x81Q\x90\x80\x80T\x90a\x07F\x82a\x08jV[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x04\xCCWP`\x01\x14a\x07sWa\x04p\x86\x88a\x04f\x82\x89\x03\x83a\x08\xA4V[\x80\x80\x95PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x83\x85\x10a\x07\xB7WPPPP\x81\x01` \x01a\x04f\x82a\x04p\x86a\x04UV[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x07\x9AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x08\x9AW[` \x83\x10\x14a\x08\x84WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x08yV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\xC6W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\t\x11WPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\x08\xEFV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t;WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t;WV[\x91\x90\x82\x03\x91\x82\x11a\tcWV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\t\xC7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x82\x91a\t\xD7\x82a\x08jV[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87`\x01\x82\x16\x91\x82`\0\x14a\x0B\x02WPP`\x01\x14a\n\xA9W[Pa\n\n\x92P\x03\x82a\x08\xA4V[Q\x90 \x91`@Q\x91\x82\x01\x92\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x84R`@\x83\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x83\x01RF`\x80\x83\x01R0`\xA0\x83\x01R`\xA0\x82R`\xC0\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\n\x95WP`@RQ\x90 \x90V[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x87\x80R\x86\x91P\x87\x90\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x85\x83\x10a\n\xEAWPPa\n\n\x93P\x82\x01\x018a\t\xFDV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\n\xD3V[`\xFF\x19\x16\x88Ra\n\n\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\t\xFD\x90PV\xFE\xA2dipfsX\"\x12 \x19q\x97\xEAS\x08\xE45\xA4\x82\xE0\xA9\xA5\t\x1D\xB5#\xC9\x19\xCC\xEE\x1D\x0BCn\xAD\xDCb\x8F\x86zLdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static COIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Coin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Coin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Coin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Coin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Coin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Coin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Coin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COIN_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                COIN_ABI.clone(),
                COIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CoinEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Coin<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum CoinEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for CoinEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CoinEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CoinEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CoinEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for CoinEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for CoinEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum CoinCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for CoinCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CoinCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CoinCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for CoinCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for CoinCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for CoinCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CoinCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for CoinCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<NameCall> for CoinCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for CoinCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for CoinCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for CoinCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for CoinCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for CoinCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for CoinCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TransferFromReturn(pub bool);
}
