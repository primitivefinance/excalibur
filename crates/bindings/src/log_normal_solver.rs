pub use log_normal_solver::*;
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
pub mod log_normal_solver {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategy"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
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
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_BISECTION_ITERS",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct LogNormParameters"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
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
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
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
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapXIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_InvalidBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_RootOutsideBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowerResult"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upperResult"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LOGNORMALSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0*\x0F8\x03\x80b\0*\x0F\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\0\xA7V[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x01$V[`\0` \x82\x84\x03\x12\x15b\0\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\x1DW`\0\x80\xFD[\x93\x92PPPV[a(\xDB\x80b\0\x014`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xE0W`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xB3W\x80cme\"\x99\x14a\x01\xC5W\x80c\x85\x93\x10\xB6\x14a\x01\xCDW\x80c\xA8\xC6.v\x14a\x01\xD5W\x80c\xEC)\xD8\xE6\x14a\x02\0W\x80c\xF9\xC2\x82\x11\x14a\x02\x13Wa\0\xE0V[\x80c\n\xC304\x14a\x01EW\x80c\n\xE1\xC5\xA5\x14a\x01qW\x80c\x1DZ\xC3\xC9\x14a\x01\x92W\x80c\x1F\xCA8\xDB\x14a\x01\xB2W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01Xa\x01S6`\x04a$\x0BV[a\x02\x1BV[`@Qa\x01h\x94\x93\x92\x91\x90a$\x80V[`@Q\x80\x91\x03\x90\xF3[a\x01\x84a\x01\x7F6`\x04a$\xA7V[a\x08 V[`@Q\x90\x81R` \x01a\x01hV[a\x01\xA5a\x01\xA06`\x04a%TV[a\n\xA4V[`@Qa\x01h\x91\x90a%\xBCV[a\x01\x84a\x01\xC06`\x04a$\xA7V[a\n\xB9V[a\x01\x84`\x01\x81V[a\x01\x84a\r0V[`\0Ta\x01\xE8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01hV[a\x01\x84a\x02\x0E6`\x04a%\xCFV[a\x0E\xD8V[a\x01\x84`Z\x81V[`\0\x80`\0``a\x02F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x02j`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03,\x91\x90a%\xFEV[`@\x80\x86\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x82Qc\x04\xB5\xC7\xAF`\xE0\x1B\x81R\x92Q\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x04\xB5\xC7\xAF\x91`\x04\x80\x82\x01\x92``\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x03\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFC\x91\x90a&lV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cT\xCF*\xEB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC3\x91\x90a&\x8BV[\x90P`\0a\x04\xDE\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0E\xD8V[\x90P\x8B\x15a\x05\xEBW`\0a\x04\xF2\x8C\x84a\x10\x9DV[\x87Q\x90\x91P`\0\x90a\x05\x0E\x90a\x05\x08\x84\x86a\x10\x9DV[\x90a\x10\xB9V[\x90Pa\x05\x1B`\x01\x82a&\xBDV[\x88Q\x90\x91Pa\x05+\x90\x8E\x90a&\xBDV[\x87R`@\x88\x01Qa\x05=\x90\x82\x90a&\xBDV[`@\x88\x01\x81\x90R\x87Qa\x05O\x91a\x08 V[` \x88\x01\x81\x81R`\x01\x91a\x05d\x90\x83\x90a&\xBDV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x05\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x05\xE2\x91\x90a&\xD0V[\x94PPPa\x06\xE4V[`\0a\x05\xF7\x8C\x84a\x10\x9DV[\x90P`\0a\x06\x16\x88` \x01Qa\x05\x08\x85\x85a\x10\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06#`\x01\x82a&\xBDV[\x90P\x8C\x88` \x01Qa\x065\x91\x90a&\xBDV[` \x88\x01R`@\x88\x01Qa\x06J\x90\x82\x90a&\xBDV[`@\x88\x01\x81\x90R` \x88\x01Qa\x06_\x91a\n\xB9V[\x80\x88R`\x01\x90\x88\x90a\x06r\x90\x83\x90a&\xBDV[\x90RP\x87Q\x87Q\x10a\x06\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[\x86Q\x88Qa\x06\xDF\x91\x90a&\xD0V[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\xC1nP\xEF`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1nP\xEF\x90a\x07Q\x90\x85\x90`\x04\x01a%\xBCV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xDF\x91\x90a&\xE3V[PPPPP\x90P\x80\x83a\x08\t\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x10\xCEV[\x91\x9E\x90\x9DP\x90\x9BP\x91\x99P\x90\x97PPPPPPPPV[`\0\x80T`@\x80Qc\xEB\xAD\xEF\x01`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEB\xAD\xEF\x01\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDB\x91\x90a%\xFEV[P`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x93\x95P\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA1\x9C\xD3\xD1\x90a\t;\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC9\x91\x90a&\x8BV[\x90Pa\n\x98\x86\x86\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\noW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x93\x91\x90a&lV[a\x11\xBBV[\x93PPPP[\x92\x91PPV[``a\n\xB1\x84\x84\x84a\x12!V[\x94\x93PPPPV[`\0\x80T`@\x80Qc\xEB\xAD\xEF\x01`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEB\xAD\xEF\x01\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bt\x91\x90a%\xFEV[PP`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R``\x80\x82\x01\x87\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x0B\xD3\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ca\x91\x90a&\x8BV[\x90Pa\n\x98\x86\x86\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r+\x91\x90a&lV[a\x12\x92V[`\0\x80T`@\x80Qc\x04\xB5\xC7\xAF`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x04\xB5\xC7\xAF\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xEB\x91\x90a&lV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xB2\x91\x90a%\xFEV[\x92PP\x91Pa\x0E\xD0\x82\x82\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x10\xCEV[\x93PPPP\x90V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x0F5\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC3\x91\x90a&\x8BV[\x90Pa\x10\x93\x86\x86\x83\x87`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8E\x91\x90a&lV[a\x12\xDBV[\x96\x95PPPPPPV[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xB9V[\x93\x92PPPV[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xB9V[`\0\x80a\x10\xDB\x84\x84a\x13\xE7V[\x90P`\0a\x10\xE9\x85\x85a\x14\rV[\x90P`\0a\x10\xF7\x82\x86a\x14<V[\x90P`\0a\x11\x05\x8A\x8Aa\x14QV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11#W`\0\x94PPPPPa\x11\xB2V[`\0\x81\x13a\x119W`\0\x19\x94PPPPPa\x11\xB2V[`\0a\x11Ua\x11P\x83g\r\xE0\xB6\xB3\xA7d\0\0a'9V[a\x14fV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11m\x88\x85a'`V[a\x11w\x91\x90a'\xA6V[a\x11\x81\x91\x90a'9V[\x90P`\0a\x11\x8E\x82a\x15\x03V[\x90P`\0a\x11\x9B\x82a\x16\xACV[\x90Pa\x11\xA7\x8C\x82a\x10\x9DV[\x98PPPPPPPPP[\x95\x94PPPPPV[\x80Q`\0\x90`\n\x90\x82\x90\x82\x90a\x11\xD2\x90\x88\x90a\x10\x9DV[a\x11\xDC\x91\x90a&\xD0V[\x90Pa\x12\x16\x87\x87\x87\x87`@Q` \x01a\x11\xF8\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\na\x01\0a\x16\xF5a\x17\"V[\x97\x96PPPPPPPV[```\0a\x120\x85\x85\x85a\x183V[\x90P`\0a\x12?\x82\x86\x86a\x18xV[\x90P`\0a\x12O\x87\x83\x85\x88a\x18\xBAV[\x90Pa\x12^\x87\x83\x83\x86\x89a\x12\xDBV[\x92P\x86\x82\x84\x87`@Q` \x01a\x12w\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0`\n\x81a\x12\xA1\x82\x87a&\xD0V[\x90Pa\x12\x16\x87\x87\x87\x87`@Q` \x01a\x12\xBD\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\na\x01\0a\x19\xC7a\x17\"V[`\0\x80`\0\x80`\0a\x12\xFA\x86`\0\x01Q\x8Aa\x14Q\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\n\x88\x12\x80\x15a\x13\x14WPa\x13\x11`\na(\x08V[\x88\x13[\x15a\x13%W\x86\x94PPPPPa\x11\xB2V[`\0\x88\x12\x15a\x13^W\x86\x92P\x80\x8A\x11a\x13HWa\x13C\x81`\x01a&\xBDV[a\x13SV[a\x13S\x8A`\x01a&\xBDV[\x93P`\x80\x91Pa\x13uV[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93P`\x80\x91P[a\x13\xAB\x8A\x8A\x8A\x89`@Q` \x01a\x13\x8F\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n\x86a\x19\xF4a\x17\"V[\x9A\x99PPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xD1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13\xF3\x83a\x1A!V[a\x14\x01\x90c;\x9A\xCA\0a($V[\x90Pa\n\xB1\x84\x82a\x14<V[`\0\x80a\x14,\x83a\x14&\x86g\x1B\xC1mgN\xC8\0\0a\x1A\xC5V[\x90a\x14<V[\x90Pa\n\xB1g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF6V[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\xF6V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x14\x7FWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\xA7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xC8W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xD5\x83`\x02a'`V[\x90P`\0a\x14\xE2\x82a\x1B\x15V[\x90P`\0a\x14\xF8g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1D\x93V[\x90Pa\x11\xB2\x81a(\x08V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x1EWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05\xC5V[P\x90V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\x0F\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x83\x86\x84\x84a\x18\xBAV[`\0\x84\x86\x11\x15a\x17OW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x05\xC5V[`\0a\x17_\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17q\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x7F\x82\x84a'`V[\x13\x15a\x17\xA8W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05\xC5V[`\0a\x17\xB4\x89\x89a&\xD0V[\x90P`\0[`\x02a\x17\xC5\x8A\x8Ca&\xBDV[a\x17\xCF\x91\x90a(~V[\x94P`\0a\x17\xE1\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xEF\x86\x83a'`V[\x13a\x17\xFCW\x85\x99Pa\x18\x03V[\x85\x9AP\x80\x94P[a\x18\r\x8B\x8Ba&\xD0V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x18!WP\x86\x81\x10[a\x17\xB9WPPPP\x96\x95PPPPPPV[`\0\x80a\x18@\x84\x84a\x1D\xA8V[\x90P`\0a\x18M\x82a\x1E\x16V[\x90P`\0a\x18Z\x82a\x16\xACV[\x90Pa\x12\x16a\x18q\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xD0V[\x88\x90a\x10\xB9V[`\0\x80a\x18\x85\x84\x84a\x1E\x7FV[\x90P`\0a\x18\x92\x82a\x1E\x16V[\x90P`\0a\x18\x9F\x82a\x16\xACV[\x85Q\x90\x91Pa\x12\x16\x90\x82\x90a\x18\xB4\x90\x8Aa\x10\x9DV[\x90a\x10\x9DV[`\0\x82\x85\x10a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC5V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x19!\x88\x87a\x14QV[\x10a\x195W`\x01`\x01`\xFF\x1B\x03\x91Pa\x19EV[a\x19Ba\x11P\x88\x87a\x14QV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x19e\x87a\x19`\x87`\0\x01Q\x89a\x14<V[a\x14QV[\x10a\x19xWP`\x01`\x01`\xFF\x1B\x03a\x19\x90V[a\x19\x8Da\x11P\x87a\x19`\x87`\0\x01Q\x89a\x14<V[\x90P[`\0a\x19\xA4\x85` \x01Q\x86`@\x01Qa\x13\xE7V[\x90P\x80a\x19\xB1\x83\x85a(\x92V[a\x19\xBB\x91\x90a(\x92V[\x98\x97PPPPPPPPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x19\xE1\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x85\x84\x84\x84a\x18\xBAV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1A\x0E\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x83\x83\x87\x84a\x18\xBAV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1A:W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1AVW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1AnW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1A\x84W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x10\xB2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1A\xDD\x86a\x1E\xC4V[a\x1A\xE7\x91\x90a'`V[a\x1A\xF1\x91\x90a'\xA6V[a\x15\x03V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\x0EW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x1B,WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1BJW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1BkW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1B\x93W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1B\x9EW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1B\xC6Wa\x1B\xC1\x83g\x1B\xC1mgN\xC8\0\0a'9V[a\x1B\xC8V[\x82[\x90P`\0a\x1B\xDE\x82g\x1B\xC1mgN\xC8\0\0a \x9FV[\x90P\x80`\0\x03a\x1C\x01W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\x0C\x82a\x1E\xC4V[\x90P`\0c;\x9A\xCA\0a\x1C7a\x1C2a\x1C,g\x1B\xC1mgN\xC8\0\0a(\x08V[\x85a\x1D\x93V[a\x1A!V[a\x1CA\x91\x90a'`V[\x90P`\0\x80a\x1CX\x83g\x03\xC1f\\z\xAB \0a\x1D\x93V[a\x1Cj\x90g \x05\xFEO&\x8E\xA0\0a(\x92V[\x90P`\0a\x1C\x9A\x84a\x1C\x83\x86f\x9F2u$b\xA0\0a\x1D\x93V[a\x1C\x95\x90g\r\xC5R\x7Fd, \0a(\x92V[a\x1D\x93V[a\x1C\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\x92V[\x90Pa\x1C\xD0g\t\xD0(\xCCo _\xFF\x19\x85a\x1C\xC6\x85\x85a \x9FV[a\x1C\x95\x91\x90a'9V[\x92PPP`\0[`\x02\x81\x10\x15a\x1DkW`\0\x86a\x1C\xEC\x84a \xB4V[a\x1C\xF6\x91\x90a'9V[\x90P`\0a\x1D\x04\x84\x85a\x1D\x93V[a\x1D\r\x90a(\x08V[\x90P`\0a\x1D\x1A\x82a\x15\x03V[\x90P`\0a\x1D(\x86\x85a\x1D\x93V[a\x1D:g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1D\x93V[a\x1DD\x91\x90a'9V[\x90Pa\x1DP\x84\x82a \x9FV[a\x1DZ\x90\x87a(\x92V[\x95P\x84`\x01\x01\x94PPPPPa\x1C\xD7V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1D\x88Wa\x1D\x83\x82a(\x08V[a\x19\xBBV[P\x96\x95PPPPPPV[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\x98V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1D\xC4\x83\x83a\x13\xE7V[\x90P`\0a\x1D\xD2\x88\x86a\"\xB7V[\x90P`\0a\x1D\xE0\x85\x85a\x14\rV[\x90P\x82a\x1D\xED\x82\x84a(\x92V[a\x1D\xFF\x90g\r\xE0\xB6\xB3\xA7d\0\0a'`V[a\x1E\t\x91\x90a'\xA6V[\x99\x98PPPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1E4g\r\xE0\xB6\xB3\xA7d\0\0\x85a'`V[a\x1E>\x91\x90a'\xA6V[\x90P`\0a\x1EK\x82a(\x08V[\x90P`\0a\x1EX\x82a \xB4V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1Eug\r\xE0\xB6\xB3\xA7d\0\0\x83a'`V[a\x11\xB2\x91\x90a'\xA6V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1E\x9B\x83\x83a\x13\xE7V[\x90P`\0a\x1E\xA9\x88\x86a\"\xB7V[\x90P`\0a\x1E\xB7\x85\x85a\x14\rV[\x90P\x82a\x1D\xED\x82\x84a'9V[`\0\x80\x82\x13a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[`\0``a\x1F\x0E\x84a\"\xCBV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\x98V[`\0\x81`\0\x03a \xCDWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a \xE4WP`\0\x91\x90PV[a \xF5gV\x98\xEE\xF0fp\0\0a(\x08V[\x82\x13a!\nWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a!\x15\x83a#sV[\x90P`\0a!Ng\r\xE0\xB6\xB3\xA7d\0\0a!7\x84g\x1B\xC1mgN\xC8\0\0a\x14QV[a!I\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\x92V[a \x9FV[\x90P`\0\x80\x82a!\xAA\x81a!\x97\x81a!\x85\x81a!r\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1D\x93V[a\x1C\x95\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a(\x92V[a\x1C\x95\x90g\x14\xA8EL\x19\xE1\xAC\0a(\x92V[a\x1C\x95\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a(\x92V[a!\xBC\x90g\x03\xDE\xBD\x08;\x8C|\0a(\x92V[\x91P\x83\x90Pa\"$\x81a\"\x12\x81a\"\0\x81a!\xEE\x81a!\xDB\x81\x8Ba\x1D\x93V[a\x1C\x95\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a(\x92V[a\x1C\x95\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a(\x92V[a\x1C\x95\x90g\x051\n\xA7\xD5!0\0a(\x92V[a\x1C\x95\x90g\r\xE0\xCC=\x15a\0\0a(\x92V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\":\x87\x88a\x1D\x93V[a\"F\x90`\0\x19a'`V[a\"P\x91\x90a'9V[a\"Z\x91\x90a(\x92V[\x92PP`\0a\"h\x83a\x15\x03V[\x90P`\0a\"v\x85\x83a\x1D\x93V[\x90P`\0\x88\x12a\"\x86W\x80a\x19\xBBV[a\x19\xBB\x81g\x1B\xC1mgN\xC8\0\0a'9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"\xB0W`\0\x80\xFD[\x05\x92\x91PPV[`\0a\x10\xB2a\"\xC6\x84\x84a\x10\xB9V[a\x1E\xC4V[`\0\x80\x82\x11a#\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a#\x99W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xF1WP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a$\x08W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a$!Wa$!a#\xAAV[\x825a$,\x81a#\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$`W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$DV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10\x93`\x80\x83\x01\x84a$:V[`\0\x80`@\x83\x85\x03\x12\x15a$\xBDWa$\xBDa#\xAAV[PP\x805\x92` \x90\x91\x015\x91PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%NWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a%mWa%ma#\xAAV[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a%\x8DWa%\x8Da$\xCCV[Pa%\x96a%\x1DV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R\x80\x91PP\x92P\x92P\x92V[` \x81R`\0a\x10\xB2` \x83\x01\x84a$:V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xE7Wa%\xE7a#\xAAV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x16Wa&\x16a#\xAAV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a&DWa&Da$\xCCV[a&La%\x1DV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a&\x81Wa&\x81a#\xAAV[a\x10\xB2\x83\x83a&/V[`\0` \x82\x84\x03\x12\x15a&\xA0Wa&\xA0a#\xAAV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\x9EWa\n\x9Ea&\xA7V[\x81\x81\x03\x81\x81\x11\x15a\n\x9EWa\n\x9Ea&\xA7V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a&\xFFWa&\xFFa#\xAAV[\x86Qa'\n\x81a#\xFAV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a'YWa'Ya&\xA7V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a'|Wa'|a&\xA7V[\x81\x81\x05\x83\x14\x82\x15\x17a\n\x9EWa\n\x9Ea&\xA7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a'\xB5Wa'\xB5a'\x90V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a'\xCFWa'\xCFa&\xA7V[P\x05\x90V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x11\xB2V[`\0`\x01`\xFF\x1B\x82\x01a(\x1DWa(\x1Da&\xA7V[P`\0\x03\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\x9EWa\n\x9Ea&\xA7V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a(TWa(Ta#\xAAV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa(s\x86``\x87\x01a&/V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a(\x8DWa(\x8Da'\x90V[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xB2Wa(\xB2a&\xA7V[PP\x92\x91PPV\xFETarget contract does not contain";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xE0W`\x005`\xE0\x1C\x80cme\"\x99\x11a\0\xB3W\x80cme\"\x99\x14a\x01\xC5W\x80c\x85\x93\x10\xB6\x14a\x01\xCDW\x80c\xA8\xC6.v\x14a\x01\xD5W\x80c\xEC)\xD8\xE6\x14a\x02\0W\x80c\xF9\xC2\x82\x11\x14a\x02\x13Wa\0\xE0V[\x80c\n\xC304\x14a\x01EW\x80c\n\xE1\xC5\xA5\x14a\x01qW\x80c\x1DZ\xC3\xC9\x14a\x01\x92W\x80c\x1F\xCA8\xDB\x14a\x01\xB2W[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81Rtnor receive functions`X\x1B`d\x83\x01R`\x84\x82\xFD[a\x01Xa\x01S6`\x04a$\x0BV[a\x02\x1BV[`@Qa\x01h\x94\x93\x92\x91\x90a$\x80V[`@Q\x80\x91\x03\x90\xF3[a\x01\x84a\x01\x7F6`\x04a$\xA7V[a\x08 V[`@Q\x90\x81R` \x01a\x01hV[a\x01\xA5a\x01\xA06`\x04a%TV[a\n\xA4V[`@Qa\x01h\x91\x90a%\xBCV[a\x01\x84a\x01\xC06`\x04a$\xA7V[a\n\xB9V[a\x01\x84`\x01\x81V[a\x01\x84a\r0V[`\0Ta\x01\xE8\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01hV[a\x01\x84a\x02\x0E6`\x04a%\xCFV[a\x0E\xD8V[a\x01\x84`Z\x81V[`\0\x80`\0``a\x02F`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x02j`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x02\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03,\x91\x90a%\xFEV[`@\x80\x86\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x82Qc\x04\xB5\xC7\xAF`\xE0\x1B\x81R\x92Q\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x04\xB5\xC7\xAF\x91`\x04\x80\x82\x01\x92``\x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x03\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x03\xD8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xFC\x91\x90a&lV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cT\xCF*\xEB`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC3\x91\x90a&\x8BV[\x90P`\0a\x04\xDE\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0E\xD8V[\x90P\x8B\x15a\x05\xEBW`\0a\x04\xF2\x8C\x84a\x10\x9DV[\x87Q\x90\x91P`\0\x90a\x05\x0E\x90a\x05\x08\x84\x86a\x10\x9DV[\x90a\x10\xB9V[\x90Pa\x05\x1B`\x01\x82a&\xBDV[\x88Q\x90\x91Pa\x05+\x90\x8E\x90a&\xBDV[\x87R`@\x88\x01Qa\x05=\x90\x82\x90a&\xBDV[`@\x88\x01\x81\x90R\x87Qa\x05O\x91a\x08 V[` \x88\x01\x81\x81R`\x01\x91a\x05d\x90\x83\x90a&\xBDV[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x05\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x05\xE2\x91\x90a&\xD0V[\x94PPPa\x06\xE4V[`\0a\x05\xF7\x8C\x84a\x10\x9DV[\x90P`\0a\x06\x16\x88` \x01Qa\x05\x08\x85\x85a\x10\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06#`\x01\x82a&\xBDV[\x90P\x8C\x88` \x01Qa\x065\x91\x90a&\xBDV[` \x88\x01R`@\x88\x01Qa\x06J\x90\x82\x90a&\xBDV[`@\x88\x01\x81\x90R` \x88\x01Qa\x06_\x91a\n\xB9V[\x80\x88R`\x01\x90\x88\x90a\x06r\x90\x83\x90a&\xBDV[\x90RP\x87Q\x87Q\x10a\x06\xD1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xC5V[\x86Q\x88Qa\x06\xDF\x91\x90a&\xD0V[\x94PPP[PP\x82Q` \x80\x85\x01Q`@\x80\x87\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\xC1nP\xEF`\xE0\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xC1nP\xEF\x90a\x07Q\x90\x85\x90`\x04\x01a%\xBCV[`\xC0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x07\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x07\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xDF\x91\x90a&\xE3V[PPPPP\x90P\x80\x83a\x08\t\x87`\0\x01Q\x88`@\x01Q\x88`\0\x01Q\x89` \x01Q\x8A`@\x01Qa\x10\xCEV[\x91\x9E\x90\x9DP\x90\x9BP\x91\x99P\x90\x97PPPPPPPPV[`\0\x80T`@\x80Qc\xEB\xAD\xEF\x01`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEB\xAD\xEF\x01\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x08\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x08\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDB\x91\x90a%\xFEV[P`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x93\x95P\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA1\x9C\xD3\xD1\x90a\t;\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xC9\x91\x90a&\x8BV[\x90Pa\n\x98\x86\x86\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\noW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x93\x91\x90a&lV[a\x11\xBBV[\x93PPPP[\x92\x91PPV[``a\n\xB1\x84\x84\x84a\x12!V[\x94\x93PPPPV[`\0\x80T`@\x80Qc\xEB\xAD\xEF\x01`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xEB\xAD\xEF\x01\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0BPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bt\x91\x90a%\xFEV[PP`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x87\x90R``\x80\x82\x01\x87\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x0B\xD3\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0C=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ca\x91\x90a&\x8BV[\x90Pa\n\x98\x86\x86\x83`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r+\x91\x90a&lV[a\x12\x92V[`\0\x80T`@\x80Qc\x04\xB5\xC7\xAF`\xE0\x1B\x81R\x90Q\x83\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\x04\xB5\xC7\xAF\x91`\x04\x80\x83\x01\x92``\x92\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15a\r\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xEB\x91\x90a&lV[\x90P`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEB\xAD\xEF\x01`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0EzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0E\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xB2\x91\x90a%\xFEV[\x92PP\x91Pa\x0E\xD0\x82\x82\x85`\0\x01Q\x86` \x01Q\x87`@\x01Qa\x10\xCEV[\x93PPPP\x90V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tc\xA1\x9C\xD3\xD1`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA1\x9C\xD3\xD1\x90a\x0F5\x90\x85\x90`\x84\x01a%\xBCV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0F\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC3\x91\x90a&\x8BV[\x90Pa\x10\x93\x86\x86\x83\x87`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x04\xB5\xC7\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` a(\xBB\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x10jW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\x8E\x91\x90a&lV[a\x12\xDBV[\x96\x95PPPPPPV[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xB9V[\x93\x92PPPV[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xB9V[`\0\x80a\x10\xDB\x84\x84a\x13\xE7V[\x90P`\0a\x10\xE9\x85\x85a\x14\rV[\x90P`\0a\x10\xF7\x82\x86a\x14<V[\x90P`\0a\x11\x05\x8A\x8Aa\x14QV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11#W`\0\x94PPPPPa\x11\xB2V[`\0\x81\x13a\x119W`\0\x19\x94PPPPPa\x11\xB2V[`\0a\x11Ua\x11P\x83g\r\xE0\xB6\xB3\xA7d\0\0a'9V[a\x14fV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11m\x88\x85a'`V[a\x11w\x91\x90a'\xA6V[a\x11\x81\x91\x90a'9V[\x90P`\0a\x11\x8E\x82a\x15\x03V[\x90P`\0a\x11\x9B\x82a\x16\xACV[\x90Pa\x11\xA7\x8C\x82a\x10\x9DV[\x98PPPPPPPPP[\x95\x94PPPPPV[\x80Q`\0\x90`\n\x90\x82\x90\x82\x90a\x11\xD2\x90\x88\x90a\x10\x9DV[a\x11\xDC\x91\x90a&\xD0V[\x90Pa\x12\x16\x87\x87\x87\x87`@Q` \x01a\x11\xF8\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\na\x01\0a\x16\xF5a\x17\"V[\x97\x96PPPPPPPV[```\0a\x120\x85\x85\x85a\x183V[\x90P`\0a\x12?\x82\x86\x86a\x18xV[\x90P`\0a\x12O\x87\x83\x85\x88a\x18\xBAV[\x90Pa\x12^\x87\x83\x83\x86\x89a\x12\xDBV[\x92P\x86\x82\x84\x87`@Q` \x01a\x12w\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0`\n\x81a\x12\xA1\x82\x87a&\xD0V[\x90Pa\x12\x16\x87\x87\x87\x87`@Q` \x01a\x12\xBD\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x83`\na\x01\0a\x19\xC7a\x17\"V[`\0\x80`\0\x80`\0a\x12\xFA\x86`\0\x01Q\x8Aa\x14Q\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\n\x88\x12\x80\x15a\x13\x14WPa\x13\x11`\na(\x08V[\x88\x13[\x15a\x13%W\x86\x94PPPPPa\x11\xB2V[`\0\x88\x12\x15a\x13^W\x86\x92P\x80\x8A\x11a\x13HWa\x13C\x81`\x01a&\xBDV[a\x13SV[a\x13S\x8A`\x01a&\xBDV[\x93P`\x80\x91Pa\x13uV[k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x92P\x86\x93P`\x80\x91P[a\x13\xAB\x8A\x8A\x8A\x89`@Q` \x01a\x13\x8F\x94\x93\x92\x91\x90a'\xD4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x85`\n\x86a\x19\xF4a\x17\"V[\x9A\x99PPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xD1W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x13\xF3\x83a\x1A!V[a\x14\x01\x90c;\x9A\xCA\0a($V[\x90Pa\n\xB1\x84\x82a\x14<V[`\0\x80a\x14,\x83a\x14&\x86g\x1B\xC1mgN\xC8\0\0a\x1A\xC5V[\x90a\x14<V[\x90Pa\n\xB1g\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF6V[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\xF6V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x14\x7FWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x14\xA7W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xC8W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xD5\x83`\x02a'`V[\x90P`\0a\x14\xE2\x82a\x1B\x15V[\x90P`\0a\x14\xF8g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1D\x93V[\x90Pa\x11\xB2\x81a(\x08V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\x1EWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xC5V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x12\x15a\x16\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05\xC5V[P\x90V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\x0F\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x83\x86\x84\x84a\x18\xBAV[`\0\x84\x86\x11\x15a\x17OW`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x05\xC5V[`\0a\x17_\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17q\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\x7F\x82\x84a'`V[\x13\x15a\x17\xA8W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05\xC5V[`\0a\x17\xB4\x89\x89a&\xD0V[\x90P`\0[`\x02a\x17\xC5\x8A\x8Ca&\xBDV[a\x17\xCF\x91\x90a(~V[\x94P`\0a\x17\xE1\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xEF\x86\x83a'`V[\x13a\x17\xFCW\x85\x99Pa\x18\x03V[\x85\x9AP\x80\x94P[a\x18\r\x8B\x8Ba&\xD0V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x18!WP\x86\x81\x10[a\x17\xB9WPPPP\x96\x95PPPPPPV[`\0\x80a\x18@\x84\x84a\x1D\xA8V[\x90P`\0a\x18M\x82a\x1E\x16V[\x90P`\0a\x18Z\x82a\x16\xACV[\x90Pa\x12\x16a\x18q\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xD0V[\x88\x90a\x10\xB9V[`\0\x80a\x18\x85\x84\x84a\x1E\x7FV[\x90P`\0a\x18\x92\x82a\x1E\x16V[\x90P`\0a\x18\x9F\x82a\x16\xACV[\x85Q\x90\x91Pa\x12\x16\x90\x82\x90a\x18\xB4\x90\x8Aa\x10\x9DV[\x90a\x10\x9DV[`\0\x82\x85\x10a\x19\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xC5V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x19!\x88\x87a\x14QV[\x10a\x195W`\x01`\x01`\xFF\x1B\x03\x91Pa\x19EV[a\x19Ba\x11P\x88\x87a\x14QV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x19e\x87a\x19`\x87`\0\x01Q\x89a\x14<V[a\x14QV[\x10a\x19xWP`\x01`\x01`\xFF\x1B\x03a\x19\x90V[a\x19\x8Da\x11P\x87a\x19`\x87`\0\x01Q\x89a\x14<V[\x90P[`\0a\x19\xA4\x85` \x01Q\x86`@\x01Qa\x13\xE7V[\x90P\x80a\x19\xB1\x83\x85a(\x92V[a\x19\xBB\x91\x90a(\x92V[\x98\x97PPPPPPPPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x19\xE1\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x85\x84\x84\x84a\x18\xBAV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1A\x0E\x91\x90a(;V[\x93PP\x92P\x92Pa\n\x98\x83\x83\x87\x84a\x18\xBAV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1A:W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1AVW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1AnW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1A\x84W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x10\xB2g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1A\xDD\x86a\x1E\xC4V[a\x1A\xE7\x91\x90a'`V[a\x1A\xF1\x91\x90a'\xA6V[a\x15\x03V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B\x0EW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x1B,WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1BJW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1BkW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1B\x93W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1B\x9EW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1B\xC6Wa\x1B\xC1\x83g\x1B\xC1mgN\xC8\0\0a'9V[a\x1B\xC8V[\x82[\x90P`\0a\x1B\xDE\x82g\x1B\xC1mgN\xC8\0\0a \x9FV[\x90P\x80`\0\x03a\x1C\x01W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\x0C\x82a\x1E\xC4V[\x90P`\0c;\x9A\xCA\0a\x1C7a\x1C2a\x1C,g\x1B\xC1mgN\xC8\0\0a(\x08V[\x85a\x1D\x93V[a\x1A!V[a\x1CA\x91\x90a'`V[\x90P`\0\x80a\x1CX\x83g\x03\xC1f\\z\xAB \0a\x1D\x93V[a\x1Cj\x90g \x05\xFEO&\x8E\xA0\0a(\x92V[\x90P`\0a\x1C\x9A\x84a\x1C\x83\x86f\x9F2u$b\xA0\0a\x1D\x93V[a\x1C\x95\x90g\r\xC5R\x7Fd, \0a(\x92V[a\x1D\x93V[a\x1C\xAC\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\x92V[\x90Pa\x1C\xD0g\t\xD0(\xCCo _\xFF\x19\x85a\x1C\xC6\x85\x85a \x9FV[a\x1C\x95\x91\x90a'9V[\x92PPP`\0[`\x02\x81\x10\x15a\x1DkW`\0\x86a\x1C\xEC\x84a \xB4V[a\x1C\xF6\x91\x90a'9V[\x90P`\0a\x1D\x04\x84\x85a\x1D\x93V[a\x1D\r\x90a(\x08V[\x90P`\0a\x1D\x1A\x82a\x15\x03V[\x90P`\0a\x1D(\x86\x85a\x1D\x93V[a\x1D:g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1D\x93V[a\x1DD\x91\x90a'9V[\x90Pa\x1DP\x84\x82a \x9FV[a\x1DZ\x90\x87a(\x92V[\x95P\x84`\x01\x01\x94PPPPPa\x1C\xD7V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1D\x88Wa\x1D\x83\x82a(\x08V[a\x19\xBBV[P\x96\x95PPPPPPV[`\0a\x10\xB2\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"\x98V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1D\xC4\x83\x83a\x13\xE7V[\x90P`\0a\x1D\xD2\x88\x86a\"\xB7V[\x90P`\0a\x1D\xE0\x85\x85a\x14\rV[\x90P\x82a\x1D\xED\x82\x84a(\x92V[a\x1D\xFF\x90g\r\xE0\xB6\xB3\xA7d\0\0a'`V[a\x1E\t\x91\x90a'\xA6V[\x99\x98PPPPPPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1E4g\r\xE0\xB6\xB3\xA7d\0\0\x85a'`V[a\x1E>\x91\x90a'\xA6V[\x90P`\0a\x1EK\x82a(\x08V[\x90P`\0a\x1EX\x82a \xB4V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x1Eug\r\xE0\xB6\xB3\xA7d\0\0\x83a'`V[a\x11\xB2\x91\x90a'\xA6V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x1E\x9B\x83\x83a\x13\xE7V[\x90P`\0a\x1E\xA9\x88\x86a\"\xB7V[\x90P`\0a\x1E\xB7\x85\x85a\x14\rV[\x90P\x82a\x1D\xED\x82\x84a'9V[`\0\x80\x82\x13a\x1F\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[`\0``a\x1F\x0E\x84a\"\xCBV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\x10\xB2\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"\x98V[`\0\x81`\0\x03a \xCDWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a \xE4WP`\0\x91\x90PV[a \xF5gV\x98\xEE\xF0fp\0\0a(\x08V[\x82\x13a!\nWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a!\x15\x83a#sV[\x90P`\0a!Ng\r\xE0\xB6\xB3\xA7d\0\0a!7\x84g\x1B\xC1mgN\xC8\0\0a\x14QV[a!I\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\x92V[a \x9FV[\x90P`\0\x80\x82a!\xAA\x81a!\x97\x81a!\x85\x81a!r\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1D\x93V[a\x1C\x95\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a(\x92V[a\x1C\x95\x90g\x14\xA8EL\x19\xE1\xAC\0a(\x92V[a\x1C\x95\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a(\x92V[a!\xBC\x90g\x03\xDE\xBD\x08;\x8C|\0a(\x92V[\x91P\x83\x90Pa\"$\x81a\"\x12\x81a\"\0\x81a!\xEE\x81a!\xDB\x81\x8Ba\x1D\x93V[a\x1C\x95\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a(\x92V[a\x1C\x95\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a(\x92V[a\x1C\x95\x90g\x051\n\xA7\xD5!0\0a(\x92V[a\x1C\x95\x90g\r\xE0\xCC=\x15a\0\0a(\x92V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\":\x87\x88a\x1D\x93V[a\"F\x90`\0\x19a'`V[a\"P\x91\x90a'9V[a\"Z\x91\x90a(\x92V[\x92PP`\0a\"h\x83a\x15\x03V[\x90P`\0a\"v\x85\x83a\x1D\x93V[\x90P`\0\x88\x12a\"\x86W\x80a\x19\xBBV[a\x19\xBB\x81g\x1B\xC1mgN\xC8\0\0a'9V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"\xB0W`\0\x80\xFD[\x05\x92\x91PPV[`\0a\x10\xB2a\"\xC6\x84\x84a\x10\xB9V[a\x1E\xC4V[`\0\x80\x82\x11a#\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xC5V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a#\x99W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x16\xF1WP\x19`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80\x15\x15\x81\x14a$\x08W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a$!Wa$!a#\xAAV[\x825a$,\x81a#\xFAV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a$`W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a$DV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x10\x93`\x80\x83\x01\x84a$:V[`\0\x80`@\x83\x85\x03\x12\x15a$\xBDWa$\xBDa#\xAAV[PP\x805\x92` \x90\x91\x015\x91PV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x81\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%NWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a%mWa%ma#\xAAV[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a%\x8DWa%\x8Da$\xCCV[Pa%\x96a%\x1DV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R\x80\x91PP\x92P\x92P\x92V[` \x81R`\0a\x10\xB2` \x83\x01\x84a$:V[`\0\x80`\0``\x84\x86\x03\x12\x15a%\xE7Wa%\xE7a#\xAAV[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x16Wa&\x16a#\xAAV[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a&DWa&Da$\xCCV[a&La%\x1DV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R\x92\x91PPV[`\0``\x82\x84\x03\x12\x15a&\x81Wa&\x81a#\xAAV[a\x10\xB2\x83\x83a&/V[`\0` \x82\x84\x03\x12\x15a&\xA0Wa&\xA0a#\xAAV[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\n\x9EWa\n\x9Ea&\xA7V[\x81\x81\x03\x81\x81\x11\x15a\n\x9EWa\n\x9Ea&\xA7V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a&\xFFWa&\xFFa#\xAAV[\x86Qa'\n\x81a#\xFAV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a'YWa'Ya&\xA7V[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a'|Wa'|a&\xA7V[\x81\x81\x05\x83\x14\x82\x15\x17a\n\x9EWa\n\x9Ea&\xA7V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a'\xB5Wa'\xB5a'\x90V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a'\xCFWa'\xCFa&\xA7V[P\x05\x90V[\x84\x81R` \x80\x82\x01\x85\x90R`@\x80\x83\x01\x85\x90R\x83Q``\x84\x01R\x90\x83\x01Q`\x80\x83\x01R\x82\x01Q`\xA0\x82\x01R`\xC0\x81\x01a\x11\xB2V[`\0`\x01`\xFF\x1B\x82\x01a(\x1DWa(\x1Da&\xA7V[P`\0\x03\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\n\x9EWa\n\x9Ea&\xA7V[`\0\x80`\0\x80`\xC0\x85\x87\x03\x12\x15a(TWa(Ta#\xAAV[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa(s\x86``\x87\x01a&/V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a(\x8DWa(\x8Da'\x90V[P\x04\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xB2Wa(\xB2a&\xA7V[PP\x92\x91PPV\xFETarget contract does not contain";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct LogNormalSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LOGNORMALSOLVER_ABI.clone(),
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
                LOGNORMALSOLVER_ABI.clone(),
                LOGNORMALSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_BISECTION_ITERS` (0xf9c28211) function
        pub fn max_bisection_iters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 194, 130, 17], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0x1d5ac3c9) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormParameters,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([29, 90, 195, 201], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xec29d8e6) function
        pub fn get_next_liquidity(
            &self,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 41, 216, 230], (rx, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x1fca38db) function
        pub fn get_next_reserve_x(
            &self,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 202, 56, 219], (ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0x0ae1c5a5) function
        pub fn get_next_reserve_y(
            &self,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 225, 197, 165], (rx, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalPrice` (0x859310b6) function
        pub fn internal_price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 147, 16, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x0ac33034) function
        pub fn simulate_swap(
            &self,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([10, 195, 48, 52], (swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LogNormalSolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BisectionLib_InvalidBounds` with signature `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `BisectionLib_RootOutsideBounds` with signature `BisectionLib_RootOutsideBounds(int256,int256)` and selector `0x1bc6f974`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
    }
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
    ///Container type for all of the contract's custom errors
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
    pub enum LogNormalSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) = <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalSolverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BisectionLib_InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BisectionLib_RootOutsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds>
    for LogNormalSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalSolverErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalSolverErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalSolverErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalSolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    ///Container type for all input parameters for the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
    ///Container type for all input parameters for the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))` and selector `0x1d5ac3c9`
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: LogNormParameters,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256)` and selector `0x1fca38db`
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
    #[ethcall(name = "getNextReserveX", abi = "getNextReserveX(uint256,uint256)")]
    pub struct GetNextReserveXCall {
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256)` and selector `0x0ae1c5a5`
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
    #[ethcall(name = "getNextReserveY", abi = "getNextReserveY(uint256,uint256)")]
    pub struct GetNextReserveYCall {
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
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
    #[ethcall(name = "internalPrice", abi = "internalPrice()")]
    pub struct InternalPriceCall;
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(bool,uint256)")]
    pub struct SimulateSwapCall {
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum LogNormalSolverCalls {
        BisectionEpsilon(BisectionEpsilonCall),
        MaxBisectionIters(MaxBisectionItersCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        InternalPrice(InternalPriceCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) = <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) = <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) = <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BisectionEpsilon(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxBisectionIters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalSolverCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalSolverCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for LogNormalSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for LogNormalSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for LogNormalSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `BISECTION_EPSILON` function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_BISECTION_ITERS` function with signature `MAX_BISECTION_ITERS()` and selector `0xf9c28211`
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
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256))` and selector `0x1d5ac3c9`
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256)` and selector `0x1fca38db`
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
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256)` and selector `0x0ae1c5a5`
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
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice()` and selector `0x859310b6`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(bool,uint256)` and selector `0x0ac33034`
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
}
