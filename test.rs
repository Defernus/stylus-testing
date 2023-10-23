#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::{Arc, Mutex};
use ethers::{
    middleware::SignerMiddleware, prelude::abigen, signers::{LocalWallet, Signer},
    types::{Address, U256},
};
use stylus_testing::{
    contract::ContractState, private_key::key_from_index,
    provider::{TestInnerProvider, TestProvider},
    utils::contract_call_helper::send,
};
static CONTRACT_BYTES: &'static [u8] = b"...";
pub use erc_20::*;
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
pub mod erc_20 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("allowance"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("owner"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("spender"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approve"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("approve"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("spender"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("amount"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("account"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("burn"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("burn"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("amount"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: ::alloc::vec::Vec::new(),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("decimals"),
                                inputs: ::alloc::vec::Vec::new(),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("spender"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("subtracted_value"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("spender"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("added_value"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("init"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("gov"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("name"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: ::alloc::vec::Vec::new(),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("mint"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("account"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("amount"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: ::alloc::vec::Vec::new(),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("name"),
                                inputs: ::alloc::vec::Vec::new(),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setGov"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("setGov"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("gov"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: ::alloc::vec::Vec::new(),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setMinter"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("setMinter"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("minter"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("is_active"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: ::alloc::vec::Vec::new(),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("symbol"),
                                inputs: ::alloc::vec::Vec::new(),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("totalSupply"),
                                inputs: ::alloc::vec::Vec::new(),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("transfer"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("recipient"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("amount"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            ::ethers::core::abi::ethabi::Function {
                                name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                                inputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("sender"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("recipient"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                            internal_type: ::core::option::Option::None,
                                        },
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::borrow::ToOwned::to_owned("amount"),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                                256usize,
                                            ),
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                outputs: <[_]>::into_vec(
                                    #[rustc_box]
                                    ::alloc::boxed::Box::new([
                                        ::ethers::core::abi::ethabi::Param {
                                            name: ::std::string::String::new(),
                                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                            internal_type: ::core::option::Option::None,
                                        },
                                    ]),
                                ),
                                constant: ::core::option::Option::None,
                                state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                            },
                        ]),
                    ),
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed human-readable ABI of the contract.
    pub static ERC20_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    pub struct Erc20<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Erc20<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Erc20<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Erc20<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Erc20<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("Erc20").field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Erc20<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC20_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
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
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burn` (0x42966c68) function
        pub fn burn(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 150, 108, 104], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x3bf73798) function
        pub fn init(
            &self,
            gov: ::ethers::core::types::Address,
            name: ::std::string::String,
            symbol: ::std::string::String,
            decimals: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 247, 55, 152], (gov, name, symbol, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x40c10f19) function
        pub fn mint(
            &self,
            account: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (account, amount))
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
        ///Calls the contract's `setGov` (0xcfad57a2) function
        pub fn set_gov(
            &self,
            gov: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 173, 87, 162], gov)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setMinter` (0xcf456ae7) function
        pub fn set_minter(
            &self,
            minter: ::ethers::core::types::Address,
            is_active: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 69, 106, 231], (minter, is_active))
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
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            sender: ::ethers::core::types::Address,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Erc20<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowanceCall {
        #[inline]
        fn clone(&self) -> AllowanceCall {
            AllowanceCall {
                owner: ::core::clone::Clone::clone(&self.owner),
                spender: ::core::clone::Clone::clone(&self.spender),
            }
        }
    }
    impl ::ethers::contract::EthCall for AllowanceCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "allowance".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [221, 98, 237, 62]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "allowance(address,address)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for AllowanceCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for AllowanceCall {}
    impl ::ethers::core::abi::Tokenizable for AllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    owner: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    spender: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.owner.into_token(),
                        self.spender.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for AllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for AllowanceCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Address,
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllowanceCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for AllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.owner, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Debug::fmt(&self.spender, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for AllowanceCall {
        #[inline]
        fn default() -> AllowanceCall {
            AllowanceCall {
                owner: ::core::default::Default::default(),
                spender: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "AllowanceCall",
                "owner",
                &self.owner,
                "spender",
                &&self.spender,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowanceCall {
        #[inline]
        fn eq(&self, other: &AllowanceCall) -> bool {
            self.owner == other.owner && self.spender == other.spender
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for AllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowanceCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AllowanceCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.owner, state);
            ::core::hash::Hash::hash(&self.spender, state)
        }
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ApproveCall {
        #[inline]
        fn clone(&self) -> ApproveCall {
            ApproveCall {
                spender: ::core::clone::Clone::clone(&self.spender),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl ::ethers::contract::EthCall for ApproveCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "approve".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [9, 94, 167, 179]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "approve(address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for ApproveCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for ApproveCall {}
    impl ::ethers::core::abi::Tokenizable for ApproveCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    spender: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    amount: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.spender.into_token(),
                        self.amount.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for ApproveCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for ApproveCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for ApproveCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for ApproveCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.spender, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.amount, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ApproveCall {
        #[inline]
        fn default() -> ApproveCall {
            ApproveCall {
                spender: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ApproveCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ApproveCall",
                "spender",
                &self.spender,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ApproveCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ApproveCall {
        #[inline]
        fn eq(&self, other: &ApproveCall) -> bool {
            self.spender == other.spender && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ApproveCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ApproveCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ApproveCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.spender, state);
            ::core::hash::Hash::hash(&self.amount, state)
        }
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BalanceOfCall {
        #[inline]
        fn clone(&self) -> BalanceOfCall {
            BalanceOfCall {
                account: ::core::clone::Clone::clone(&self.account),
            }
        }
    }
    impl ::ethers::contract::EthCall for BalanceOfCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "balanceOf".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [112, 160, 130, 49]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "balanceOf(address)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for BalanceOfCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for BalanceOfCall {}
    impl ::ethers::core::abi::Tokenizable for BalanceOfCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    account: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.account.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for BalanceOfCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for BalanceOfCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [::ethers::core::abi::ParamType::Address];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalanceOfCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for BalanceOfCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.account, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BalanceOfCall {
        #[inline]
        fn default() -> BalanceOfCall {
            BalanceOfCall {
                account: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BalanceOfCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "BalanceOfCall",
                "account",
                &&self.account,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BalanceOfCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BalanceOfCall {
        #[inline]
        fn eq(&self, other: &BalanceOfCall) -> bool {
            self.account == other.account
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for BalanceOfCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for BalanceOfCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BalanceOfCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.account, state)
        }
    }
    ///Container type for all input parameters for the `burn` function with signature `burn(uint256)` and selector `0x42966c68`
    #[ethcall(name = "burn", abi = "burn(uint256)")]
    pub struct BurnCall {
        pub amount: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BurnCall {
        #[inline]
        fn clone(&self) -> BurnCall {
            BurnCall {
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl ::ethers::contract::EthCall for BurnCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "burn".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [66, 150, 108, 104]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "burn(uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for BurnCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for BurnCall {}
    impl ::ethers::core::abi::Tokenizable for BurnCall
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    amount: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.amount.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for BurnCall
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for BurnCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [::ethers::core::abi::ParamType::Uint(256usize)];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for BurnCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for BurnCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Display::fmt(&self.amount, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BurnCall {
        #[inline]
        fn default() -> BurnCall {
            BurnCall {
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BurnCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "BurnCall",
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BurnCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BurnCall {
        #[inline]
        fn eq(&self, other: &BurnCall) -> bool {
            self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for BurnCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for BurnCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BurnCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.amount, state)
        }
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[automatically_derived]
    impl ::core::clone::Clone for DecimalsCall {
        #[inline]
        fn clone(&self) -> DecimalsCall {
            DecimalsCall
        }
    }
    impl ::ethers::contract::EthCall for DecimalsCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "decimals".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [49, 60, 229, 103]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "decimals()".into()
        }
    }
    impl ::ethers::core::abi::Tokenizable for DecimalsCall {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if !tokens.is_empty() {
                    Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected empty tuple, got {0:?}", tokens),
                            );
                            res
                        }),
                    )
                } else {
                    Ok(DecimalsCall {})
                }
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
        }
    }
    impl ::ethers::core::abi::TokenizableItem for DecimalsCall {}
    impl ::ethers::core::abi::AbiDecode for DecimalsCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for DecimalsCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for DecimalsCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DecimalsCall {
        #[inline]
        fn default() -> DecimalsCall {
            DecimalsCall {}
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DecimalsCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "DecimalsCall")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DecimalsCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DecimalsCall {
        #[inline]
        fn eq(&self, other: &DecimalsCall) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DecimalsCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DecimalsCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DecimalsCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DecreaseAllowanceCall {
        #[inline]
        fn clone(&self) -> DecreaseAllowanceCall {
            DecreaseAllowanceCall {
                spender: ::core::clone::Clone::clone(&self.spender),
                subtracted_value: ::core::clone::Clone::clone(&self.subtracted_value),
            }
        }
    }
    impl ::ethers::contract::EthCall for DecreaseAllowanceCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "decreaseAllowance".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [164, 87, 194, 215]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "decreaseAllowance(address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for DecreaseAllowanceCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for DecreaseAllowanceCall {}
    impl ::ethers::core::abi::Tokenizable for DecreaseAllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    spender: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    subtracted_value: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.spender.into_token(),
                        self.subtracted_value.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for DecreaseAllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for DecreaseAllowanceCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for DecreaseAllowanceCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for DecreaseAllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.spender, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.subtracted_value, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DecreaseAllowanceCall {
        #[inline]
        fn default() -> DecreaseAllowanceCall {
            DecreaseAllowanceCall {
                spender: ::core::default::Default::default(),
                subtracted_value: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DecreaseAllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "DecreaseAllowanceCall",
                "spender",
                &self.spender,
                "subtracted_value",
                &&self.subtracted_value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DecreaseAllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DecreaseAllowanceCall {
        #[inline]
        fn eq(&self, other: &DecreaseAllowanceCall) -> bool {
            self.spender == other.spender
                && self.subtracted_value == other.subtracted_value
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DecreaseAllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DecreaseAllowanceCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DecreaseAllowanceCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.spender, state);
            ::core::hash::Hash::hash(&self.subtracted_value, state)
        }
    }
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IncreaseAllowanceCall {
        #[inline]
        fn clone(&self) -> IncreaseAllowanceCall {
            IncreaseAllowanceCall {
                spender: ::core::clone::Clone::clone(&self.spender),
                added_value: ::core::clone::Clone::clone(&self.added_value),
            }
        }
    }
    impl ::ethers::contract::EthCall for IncreaseAllowanceCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "increaseAllowance".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [57, 80, 147, 81]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "increaseAllowance(address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for IncreaseAllowanceCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for IncreaseAllowanceCall {}
    impl ::ethers::core::abi::Tokenizable for IncreaseAllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    spender: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    added_value: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.spender.into_token(),
                        self.added_value.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for IncreaseAllowanceCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for IncreaseAllowanceCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for IncreaseAllowanceCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for IncreaseAllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.spender, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.added_value, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for IncreaseAllowanceCall {
        #[inline]
        fn default() -> IncreaseAllowanceCall {
            IncreaseAllowanceCall {
                spender: ::core::default::Default::default(),
                added_value: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IncreaseAllowanceCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "IncreaseAllowanceCall",
                "spender",
                &self.spender,
                "added_value",
                &&self.added_value,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IncreaseAllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IncreaseAllowanceCall {
        #[inline]
        fn eq(&self, other: &IncreaseAllowanceCall) -> bool {
            self.spender == other.spender && self.added_value == other.added_value
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for IncreaseAllowanceCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for IncreaseAllowanceCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for IncreaseAllowanceCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.spender, state);
            ::core::hash::Hash::hash(&self.added_value, state)
        }
    }
    ///Container type for all input parameters for the `init` function with signature `init(address,string,string,uint8)` and selector `0x3bf73798`
    #[ethcall(name = "init", abi = "init(address,string,string,uint8)")]
    pub struct InitCall {
        pub gov: ::ethers::core::types::Address,
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
        pub decimals: u8,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InitCall {
        #[inline]
        fn clone(&self) -> InitCall {
            InitCall {
                gov: ::core::clone::Clone::clone(&self.gov),
                name: ::core::clone::Clone::clone(&self.name),
                symbol: ::core::clone::Clone::clone(&self.symbol),
                decimals: ::core::clone::Clone::clone(&self.decimals),
            }
        }
    }
    impl ::ethers::contract::EthCall for InitCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "init".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [59, 247, 55, 152]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "init(address,string,string,uint8)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for InitCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::std::string::String as ::ethers::core::abi::AbiType>::param_type(),
                        <::std::string::String as ::ethers::core::abi::AbiType>::param_type(),
                        <u8 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for InitCall {}
    impl ::ethers::core::abi::Tokenizable for InitCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::std::string::String: ::ethers::core::abi::Tokenize,
        ::std::string::String: ::ethers::core::abi::Tokenize,
        u8: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 4usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 4 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    gov: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    name: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    symbol: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    decimals: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.gov.into_token(),
                        self.name.into_token(),
                        self.symbol.into_token(),
                        self.decimals.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for InitCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::std::string::String: ::ethers::core::abi::Tokenize,
        ::std::string::String: ::ethers::core::abi::Tokenize,
        u8: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for InitCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::String,
                ::ethers::core::abi::ParamType::String,
                ::ethers::core::abi::ParamType::Uint(8usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for InitCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for InitCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.gov, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.name, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.symbol, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.decimals, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for InitCall {
        #[inline]
        fn default() -> InitCall {
            InitCall {
                gov: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                symbol: ::core::default::Default::default(),
                decimals: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InitCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "InitCall",
                "gov",
                &self.gov,
                "name",
                &self.name,
                "symbol",
                &self.symbol,
                "decimals",
                &&self.decimals,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for InitCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for InitCall {
        #[inline]
        fn eq(&self, other: &InitCall) -> bool {
            self.gov == other.gov && self.name == other.name
                && self.symbol == other.symbol && self.decimals == other.decimals
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for InitCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for InitCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::std::string::String>;
            let _: ::core::cmp::AssertParamIsEq<::std::string::String>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for InitCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.gov, state);
            ::core::hash::Hash::hash(&self.name, state);
            ::core::hash::Hash::hash(&self.symbol, state);
            ::core::hash::Hash::hash(&self.decimals, state)
        }
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256)` and selector `0x40c10f19`
    #[ethcall(name = "mint", abi = "mint(address,uint256)")]
    pub struct MintCall {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MintCall {
        #[inline]
        fn clone(&self) -> MintCall {
            MintCall {
                account: ::core::clone::Clone::clone(&self.account),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl ::ethers::contract::EthCall for MintCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "mint".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [64, 193, 15, 25]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "mint(address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for MintCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for MintCall {}
    impl ::ethers::core::abi::Tokenizable for MintCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    account: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    amount: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.account.into_token(),
                        self.amount.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for MintCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for MintCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for MintCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for MintCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.account, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.amount, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for MintCall {
        #[inline]
        fn default() -> MintCall {
            MintCall {
                account: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MintCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "MintCall",
                "account",
                &self.account,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MintCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MintCall {
        #[inline]
        fn eq(&self, other: &MintCall) -> bool {
            self.account == other.account && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for MintCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for MintCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MintCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.account, state);
            ::core::hash::Hash::hash(&self.amount, state)
        }
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[automatically_derived]
    impl ::core::clone::Clone for NameCall {
        #[inline]
        fn clone(&self) -> NameCall {
            NameCall
        }
    }
    impl ::ethers::contract::EthCall for NameCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "name".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [6, 253, 222, 3]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "name()".into()
        }
    }
    impl ::ethers::core::abi::Tokenizable for NameCall {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if !tokens.is_empty() {
                    Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected empty tuple, got {0:?}", tokens),
                            );
                            res
                        }),
                    )
                } else {
                    Ok(NameCall {})
                }
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
        }
    }
    impl ::ethers::core::abi::TokenizableItem for NameCall {}
    impl ::ethers::core::abi::AbiDecode for NameCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for NameCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for NameCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for NameCall {
        #[inline]
        fn default() -> NameCall {
            NameCall {}
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NameCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "NameCall")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NameCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NameCall {
        #[inline]
        fn eq(&self, other: &NameCall) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for NameCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for NameCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for NameCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    ///Container type for all input parameters for the `setGov` function with signature `setGov(address)` and selector `0xcfad57a2`
    #[ethcall(name = "setGov", abi = "setGov(address)")]
    pub struct SetGovCall {
        pub gov: ::ethers::core::types::Address,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SetGovCall {
        #[inline]
        fn clone(&self) -> SetGovCall {
            SetGovCall {
                gov: ::core::clone::Clone::clone(&self.gov),
            }
        }
    }
    impl ::ethers::contract::EthCall for SetGovCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "setGov".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [207, 173, 87, 162]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "setGov(address)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for SetGovCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for SetGovCall {}
    impl ::ethers::core::abi::Tokenizable for SetGovCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    gov: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.gov.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for SetGovCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for SetGovCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [::ethers::core::abi::ParamType::Address];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetGovCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for SetGovCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.gov, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SetGovCall {
        #[inline]
        fn default() -> SetGovCall {
            SetGovCall {
                gov: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SetGovCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SetGovCall",
                "gov",
                &&self.gov,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SetGovCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SetGovCall {
        #[inline]
        fn eq(&self, other: &SetGovCall) -> bool {
            self.gov == other.gov
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SetGovCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SetGovCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SetGovCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.gov, state)
        }
    }
    ///Container type for all input parameters for the `setMinter` function with signature `setMinter(address,bool)` and selector `0xcf456ae7`
    #[ethcall(name = "setMinter", abi = "setMinter(address,bool)")]
    pub struct SetMinterCall {
        pub minter: ::ethers::core::types::Address,
        pub is_active: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SetMinterCall {
        #[inline]
        fn clone(&self) -> SetMinterCall {
            SetMinterCall {
                minter: ::core::clone::Clone::clone(&self.minter),
                is_active: ::core::clone::Clone::clone(&self.is_active),
            }
        }
    }
    impl ::ethers::contract::EthCall for SetMinterCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "setMinter".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [207, 69, 106, 231]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "setMinter(address,bool)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for SetMinterCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for SetMinterCall {}
    impl ::ethers::core::abi::Tokenizable for SetMinterCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    minter: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    is_active: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.minter.into_token(),
                        self.is_active.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for SetMinterCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for SetMinterCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Bool,
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetMinterCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for SetMinterCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.minter, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.is_active, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SetMinterCall {
        #[inline]
        fn default() -> SetMinterCall {
            SetMinterCall {
                minter: ::core::default::Default::default(),
                is_active: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SetMinterCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SetMinterCall",
                "minter",
                &self.minter,
                "is_active",
                &&self.is_active,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SetMinterCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SetMinterCall {
        #[inline]
        fn eq(&self, other: &SetMinterCall) -> bool {
            self.minter == other.minter && self.is_active == other.is_active
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SetMinterCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SetMinterCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SetMinterCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.minter, state);
            ::core::hash::Hash::hash(&self.is_active, state)
        }
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[automatically_derived]
    impl ::core::clone::Clone for SymbolCall {
        #[inline]
        fn clone(&self) -> SymbolCall {
            SymbolCall
        }
    }
    impl ::ethers::contract::EthCall for SymbolCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "symbol".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [149, 216, 155, 65]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "symbol()".into()
        }
    }
    impl ::ethers::core::abi::Tokenizable for SymbolCall {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if !tokens.is_empty() {
                    Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected empty tuple, got {0:?}", tokens),
                            );
                            res
                        }),
                    )
                } else {
                    Ok(SymbolCall {})
                }
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
        }
    }
    impl ::ethers::core::abi::TokenizableItem for SymbolCall {}
    impl ::ethers::core::abi::AbiDecode for SymbolCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for SymbolCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for SymbolCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SymbolCall {
        #[inline]
        fn default() -> SymbolCall {
            SymbolCall {}
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SymbolCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "SymbolCall")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SymbolCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SymbolCall {
        #[inline]
        fn eq(&self, other: &SymbolCall) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SymbolCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SymbolCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SymbolCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[automatically_derived]
    impl ::core::clone::Clone for TotalSupplyCall {
        #[inline]
        fn clone(&self) -> TotalSupplyCall {
            TotalSupplyCall
        }
    }
    impl ::ethers::contract::EthCall for TotalSupplyCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "totalSupply".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [24, 22, 13, 221]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "totalSupply()".into()
        }
    }
    impl ::ethers::core::abi::Tokenizable for TotalSupplyCall {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if !tokens.is_empty() {
                    Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected empty tuple, got {0:?}", tokens),
                            );
                            res
                        }),
                    )
                } else {
                    Ok(TotalSupplyCall {})
                }
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(::std::vec::Vec::new())
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TotalSupplyCall {}
    impl ::ethers::core::abi::AbiDecode for TotalSupplyCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for TotalSupplyCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for TotalSupplyCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TotalSupplyCall {
        #[inline]
        fn default() -> TotalSupplyCall {
            TotalSupplyCall {}
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TotalSupplyCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "TotalSupplyCall")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TotalSupplyCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TotalSupplyCall {
        #[inline]
        fn eq(&self, other: &TotalSupplyCall) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TotalSupplyCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TotalSupplyCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TotalSupplyCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
    }
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TransferCall {
        #[inline]
        fn clone(&self) -> TransferCall {
            TransferCall {
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl ::ethers::contract::EthCall for TransferCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "transfer".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [169, 5, 156, 187]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "transfer(address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for TransferCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for TransferCall {}
    impl ::ethers::core::abi::Tokenizable for TransferCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 2usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 2 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    recipient: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    amount: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.recipient.into_token(),
                        self.amount.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TransferCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for TransferCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransferCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for TransferCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.recipient, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.amount, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TransferCall {
        #[inline]
        fn default() -> TransferCall {
            TransferCall {
                recipient: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransferCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TransferCall",
                "recipient",
                &self.recipient,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TransferCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TransferCall {
        #[inline]
        fn eq(&self, other: &TransferCall) -> bool {
            self.recipient == other.recipient && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TransferCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TransferCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TransferCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.recipient, state);
            ::core::hash::Hash::hash(&self.amount, state)
        }
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TransferFromCall {
        #[inline]
        fn clone(&self) -> TransferFromCall {
            TransferFromCall {
                sender: ::core::clone::Clone::clone(&self.sender),
                recipient: ::core::clone::Clone::clone(&self.recipient),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    impl ::ethers::contract::EthCall for TransferFromCall {
        fn function_name() -> ::std::borrow::Cow<'static, str> {
            "transferFrom".into()
        }
        fn selector() -> ::ethers::core::types::Selector {
            [35, 184, 114, 221]
        }
        fn abi_signature() -> ::std::borrow::Cow<'static, str> {
            "transferFrom(address,address,uint256)".into()
        }
    }
    impl ::ethers::core::abi::AbiType for TransferFromCall {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::Address as ::ethers::core::abi::AbiType>::param_type(),
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for TransferFromCall {}
    impl ::ethers::core::abi::Tokenizable for TransferFromCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 3usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 3 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(Self {
                    sender: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    recipient: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                    amount: ::ethers::core::abi::Tokenizable::from_token(
                        iter
                            .next()
                            .expect(
                                "The iter is guaranteed to be something due to the size check",
                            ),
                    )?,
                })
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        self.sender.into_token(),
                        self.recipient.into_token(),
                        self.amount.into_token(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TransferFromCall
    where
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::Address: ::ethers::core::abi::Tokenize,
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for TransferFromCall {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let bytes = bytes.as_ref();
            if bytes.len() < 4
                || bytes[..4] != <Self as ::ethers::contract::EthCall>::selector()
            {
                return Err(::ethers::contract::AbiError::WrongSelector);
            }
            let data_types = [
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Address,
                ::ethers::core::abi::ParamType::Uint(256usize),
            ];
            let data_tokens = ::ethers::core::abi::decode(&data_types, &bytes[4..])?;
            Ok(
                <Self as ::ethers::core::abi::Tokenizable>::from_token(
                    ::ethers::core::abi::Token::Tuple(data_tokens),
                )?,
            )
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransferFromCall {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            let selector = <Self as ::ethers::contract::EthCall>::selector();
            let encoded = ::ethers::core::abi::encode(&tokens);
            selector.iter().copied().chain(encoded.into_iter()).collect()
        }
    }
    impl ::core::fmt::Display for TransferFromCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            ::core::fmt::Debug::fmt(&self.sender, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Debug::fmt(&self.recipient, f)?;
            ::core::fmt::Write::write_str(f, ", ")?;
            ::core::fmt::Display::fmt(&self.amount, f)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TransferFromCall {
        #[inline]
        fn default() -> TransferFromCall {
            TransferFromCall {
                sender: ::core::default::Default::default(),
                recipient: ::core::default::Default::default(),
                amount: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransferFromCall {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TransferFromCall",
                "sender",
                &self.sender,
                "recipient",
                &self.recipient,
                "amount",
                &&self.amount,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TransferFromCall {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TransferFromCall {
        #[inline]
        fn eq(&self, other: &TransferFromCall) -> bool {
            self.sender == other.sender && self.recipient == other.recipient
                && self.amount == other.amount
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TransferFromCall {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TransferFromCall {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::Address>;
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TransferFromCall {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.sender, state);
            ::core::hash::Hash::hash(&self.recipient, state);
            ::core::hash::Hash::hash(&self.amount, state)
        }
    }
    ///Container type for all of the contract's call
    pub enum Erc20Calls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Burn(BurnCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        Init(InitCall),
        Mint(MintCall),
        Name(NameCall),
        SetGov(SetGovCall),
        SetMinter(SetMinterCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Erc20Calls {
        #[inline]
        fn clone(&self) -> Erc20Calls {
            match self {
                Erc20Calls::Allowance(__self_0) => {
                    Erc20Calls::Allowance(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Approve(__self_0) => {
                    Erc20Calls::Approve(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::BalanceOf(__self_0) => {
                    Erc20Calls::BalanceOf(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Burn(__self_0) => {
                    Erc20Calls::Burn(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Decimals(__self_0) => {
                    Erc20Calls::Decimals(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::DecreaseAllowance(__self_0) => {
                    Erc20Calls::DecreaseAllowance(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::IncreaseAllowance(__self_0) => {
                    Erc20Calls::IncreaseAllowance(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Init(__self_0) => {
                    Erc20Calls::Init(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Mint(__self_0) => {
                    Erc20Calls::Mint(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Name(__self_0) => {
                    Erc20Calls::Name(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::SetGov(__self_0) => {
                    Erc20Calls::SetGov(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::SetMinter(__self_0) => {
                    Erc20Calls::SetMinter(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Symbol(__self_0) => {
                    Erc20Calls::Symbol(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::TotalSupply(__self_0) => {
                    Erc20Calls::TotalSupply(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::Transfer(__self_0) => {
                    Erc20Calls::Transfer(::core::clone::Clone::clone(__self_0))
                }
                Erc20Calls::TransferFrom(__self_0) => {
                    Erc20Calls::TransferFrom(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    impl ::ethers::core::abi::Tokenizable for Erc20Calls {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Allowance(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Approve(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Burn(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Decimals(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Init(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Mint(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Name(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::SetGov(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::SetMinter(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Symbol(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(
                token.clone(),
            ) {
                return Ok(Erc20Calls::Transfer(decoded));
            }
            if let Ok(decoded) = ::ethers::core::abi::Tokenizable::from_token(token) {
                return Ok(Erc20Calls::TransferFrom(decoded));
            }
            Err(
                ::ethers::core::abi::InvalidOutputType(
                    "Failed to decode all type variants".to_string(),
                ),
            )
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            match self {
                Erc20Calls::Allowance(element) => element.into_token(),
                Erc20Calls::Approve(element) => element.into_token(),
                Erc20Calls::BalanceOf(element) => element.into_token(),
                Erc20Calls::Burn(element) => element.into_token(),
                Erc20Calls::Decimals(element) => element.into_token(),
                Erc20Calls::DecreaseAllowance(element) => element.into_token(),
                Erc20Calls::IncreaseAllowance(element) => element.into_token(),
                Erc20Calls::Init(element) => element.into_token(),
                Erc20Calls::Mint(element) => element.into_token(),
                Erc20Calls::Name(element) => element.into_token(),
                Erc20Calls::SetGov(element) => element.into_token(),
                Erc20Calls::SetMinter(element) => element.into_token(),
                Erc20Calls::Symbol(element) => element.into_token(),
                Erc20Calls::TotalSupply(element) => element.into_token(),
                Erc20Calls::Transfer(element) => element.into_token(),
                Erc20Calls::TransferFrom(element) => element.into_token(),
            }
        }
    }
    impl ::ethers::core::abi::TokenizableItem for Erc20Calls {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Erc20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Erc20Calls::Allowance(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Allowance",
                        &__self_0,
                    )
                }
                Erc20Calls::Approve(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Approve",
                        &__self_0,
                    )
                }
                Erc20Calls::BalanceOf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BalanceOf",
                        &__self_0,
                    )
                }
                Erc20Calls::Burn(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Burn",
                        &__self_0,
                    )
                }
                Erc20Calls::Decimals(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Decimals",
                        &__self_0,
                    )
                }
                Erc20Calls::DecreaseAllowance(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DecreaseAllowance",
                        &__self_0,
                    )
                }
                Erc20Calls::IncreaseAllowance(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IncreaseAllowance",
                        &__self_0,
                    )
                }
                Erc20Calls::Init(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Init",
                        &__self_0,
                    )
                }
                Erc20Calls::Mint(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Mint",
                        &__self_0,
                    )
                }
                Erc20Calls::Name(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Name",
                        &__self_0,
                    )
                }
                Erc20Calls::SetGov(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SetGov",
                        &__self_0,
                    )
                }
                Erc20Calls::SetMinter(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SetMinter",
                        &__self_0,
                    )
                }
                Erc20Calls::Symbol(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Symbol",
                        &__self_0,
                    )
                }
                Erc20Calls::TotalSupply(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TotalSupply",
                        &__self_0,
                    )
                }
                Erc20Calls::Transfer(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Transfer",
                        &__self_0,
                    )
                }
                Erc20Calls::TransferFrom(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TransferFrom",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Erc20Calls {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Erc20Calls {
        #[inline]
        fn eq(&self, other: &Erc20Calls) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        Erc20Calls::Allowance(__self_0),
                        Erc20Calls::Allowance(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (Erc20Calls::Approve(__self_0), Erc20Calls::Approve(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Erc20Calls::BalanceOf(__self_0),
                        Erc20Calls::BalanceOf(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (Erc20Calls::Burn(__self_0), Erc20Calls::Burn(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Erc20Calls::Decimals(__self_0), Erc20Calls::Decimals(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Erc20Calls::DecreaseAllowance(__self_0),
                        Erc20Calls::DecreaseAllowance(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        Erc20Calls::IncreaseAllowance(__self_0),
                        Erc20Calls::IncreaseAllowance(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (Erc20Calls::Init(__self_0), Erc20Calls::Init(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Erc20Calls::Mint(__self_0), Erc20Calls::Mint(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Erc20Calls::Name(__self_0), Erc20Calls::Name(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (Erc20Calls::SetGov(__self_0), Erc20Calls::SetGov(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Erc20Calls::SetMinter(__self_0),
                        Erc20Calls::SetMinter(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (Erc20Calls::Symbol(__self_0), Erc20Calls::Symbol(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Erc20Calls::TotalSupply(__self_0),
                        Erc20Calls::TotalSupply(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (Erc20Calls::Transfer(__self_0), Erc20Calls::Transfer(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        Erc20Calls::TransferFrom(__self_0),
                        Erc20Calls::TransferFrom(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Erc20Calls {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Erc20Calls {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<AllowanceCall>;
            let _: ::core::cmp::AssertParamIsEq<ApproveCall>;
            let _: ::core::cmp::AssertParamIsEq<BalanceOfCall>;
            let _: ::core::cmp::AssertParamIsEq<BurnCall>;
            let _: ::core::cmp::AssertParamIsEq<DecimalsCall>;
            let _: ::core::cmp::AssertParamIsEq<DecreaseAllowanceCall>;
            let _: ::core::cmp::AssertParamIsEq<IncreaseAllowanceCall>;
            let _: ::core::cmp::AssertParamIsEq<InitCall>;
            let _: ::core::cmp::AssertParamIsEq<MintCall>;
            let _: ::core::cmp::AssertParamIsEq<NameCall>;
            let _: ::core::cmp::AssertParamIsEq<SetGovCall>;
            let _: ::core::cmp::AssertParamIsEq<SetMinterCall>;
            let _: ::core::cmp::AssertParamIsEq<SymbolCall>;
            let _: ::core::cmp::AssertParamIsEq<TotalSupplyCall>;
            let _: ::core::cmp::AssertParamIsEq<TransferCall>;
            let _: ::core::cmp::AssertParamIsEq<TransferFromCall>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Erc20Calls {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state);
            match self {
                Erc20Calls::Allowance(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::Approve(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::BalanceOf(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::Burn(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::Decimals(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::DecreaseAllowance(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::IncreaseAllowance(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::Init(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::Mint(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::Name(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::SetGov(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::SetMinter(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::Symbol(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                Erc20Calls::TotalSupply(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::Transfer(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                Erc20Calls::TransferFrom(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
            }
        }
    }
    impl ::ethers::core::abi::AbiDecode for Erc20Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <BurnCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Burn(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <SetGovCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetGov(decoded));
            }
            if let Ok(decoded) = <SetMinterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetMinter(decoded));
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
    impl ::ethers::core::abi::AbiEncode for Erc20Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Burn(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetGov(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetMinter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::fmt::Display for Erc20Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Burn(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetGov(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetMinter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for Erc20Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for Erc20Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for Erc20Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BurnCall> for Erc20Calls {
        fn from(value: BurnCall) -> Self {
            Self::Burn(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for Erc20Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for Erc20Calls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for Erc20Calls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitCall> for Erc20Calls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<MintCall> for Erc20Calls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for Erc20Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<SetGovCall> for Erc20Calls {
        fn from(value: SetGovCall) -> Self {
            Self::SetGov(value)
        }
    }
    impl ::core::convert::From<SetMinterCall> for Erc20Calls {
        fn from(value: SetMinterCall) -> Self {
            Self::SetMinter(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for Erc20Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for Erc20Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for Erc20Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for Erc20Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    #[automatically_derived]
    impl ::core::clone::Clone for AllowanceReturn {
        #[inline]
        fn clone(&self) -> AllowanceReturn {
            AllowanceReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for AllowanceReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for AllowanceReturn {}
    impl ::ethers::core::abi::Tokenizable for AllowanceReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for AllowanceReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for AllowanceReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<AllowanceReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <AllowanceReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <AllowanceReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AllowanceReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for AllowanceReturn {
        #[inline]
        fn default() -> AllowanceReturn {
            AllowanceReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AllowanceReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "AllowanceReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AllowanceReturn {
        #[inline]
        fn eq(&self, other: &AllowanceReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for AllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for AllowanceReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AllowanceReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    pub struct ApproveReturn(pub bool);
    #[automatically_derived]
    impl ::core::clone::Clone for ApproveReturn {
        #[inline]
        fn clone(&self) -> ApproveReturn {
            ApproveReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for ApproveReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for ApproveReturn {}
    impl ::ethers::core::abi::Tokenizable for ApproveReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for ApproveReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for ApproveReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<ApproveReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <ApproveReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <ApproveReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ApproveReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ApproveReturn {
        #[inline]
        fn default() -> ApproveReturn {
            ApproveReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ApproveReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "ApproveReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ApproveReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ApproveReturn {
        #[inline]
        fn eq(&self, other: &ApproveReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ApproveReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ApproveReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ApproveReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    #[automatically_derived]
    impl ::core::clone::Clone for BalanceOfReturn {
        #[inline]
        fn clone(&self) -> BalanceOfReturn {
            BalanceOfReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for BalanceOfReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for BalanceOfReturn {}
    impl ::ethers::core::abi::Tokenizable for BalanceOfReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for BalanceOfReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for BalanceOfReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<BalanceOfReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <BalanceOfReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <BalanceOfReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalanceOfReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for BalanceOfReturn {
        #[inline]
        fn default() -> BalanceOfReturn {
            BalanceOfReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BalanceOfReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "BalanceOfReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BalanceOfReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BalanceOfReturn {
        #[inline]
        fn eq(&self, other: &BalanceOfReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for BalanceOfReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for BalanceOfReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for BalanceOfReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    pub struct DecimalsReturn(pub u8);
    #[automatically_derived]
    impl ::core::clone::Clone for DecimalsReturn {
        #[inline]
        fn clone(&self) -> DecimalsReturn {
            DecimalsReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for DecimalsReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <u8 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for DecimalsReturn {}
    impl ::ethers::core::abi::Tokenizable for DecimalsReturn
    where
        u8: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for DecimalsReturn
    where
        u8: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for DecimalsReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<DecimalsReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <DecimalsReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <DecimalsReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DecimalsReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DecimalsReturn {
        #[inline]
        fn default() -> DecimalsReturn {
            DecimalsReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DecimalsReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "DecimalsReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DecimalsReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DecimalsReturn {
        #[inline]
        fn eq(&self, other: &DecimalsReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DecimalsReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DecimalsReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DecimalsReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
    pub struct DecreaseAllowanceReturn(pub bool);
    #[automatically_derived]
    impl ::core::clone::Clone for DecreaseAllowanceReturn {
        #[inline]
        fn clone(&self) -> DecreaseAllowanceReturn {
            DecreaseAllowanceReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for DecreaseAllowanceReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for DecreaseAllowanceReturn {}
    impl ::ethers::core::abi::Tokenizable for DecreaseAllowanceReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for DecreaseAllowanceReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for DecreaseAllowanceReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<
                DecreaseAllowanceReturn,
                ::ethers::core::abi::AbiError,
            > {
                let ::ethers::core::abi::ParamType::Tuple(params) = <DecreaseAllowanceReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <DecreaseAllowanceReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DecreaseAllowanceReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DecreaseAllowanceReturn {
        #[inline]
        fn default() -> DecreaseAllowanceReturn {
            DecreaseAllowanceReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DecreaseAllowanceReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "DecreaseAllowanceReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DecreaseAllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DecreaseAllowanceReturn {
        #[inline]
        fn eq(&self, other: &DecreaseAllowanceReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for DecreaseAllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for DecreaseAllowanceReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for DecreaseAllowanceReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
    pub struct IncreaseAllowanceReturn(pub bool);
    #[automatically_derived]
    impl ::core::clone::Clone for IncreaseAllowanceReturn {
        #[inline]
        fn clone(&self) -> IncreaseAllowanceReturn {
            IncreaseAllowanceReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for IncreaseAllowanceReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for IncreaseAllowanceReturn {}
    impl ::ethers::core::abi::Tokenizable for IncreaseAllowanceReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for IncreaseAllowanceReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for IncreaseAllowanceReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<
                IncreaseAllowanceReturn,
                ::ethers::core::abi::AbiError,
            > {
                let ::ethers::core::abi::ParamType::Tuple(params) = <IncreaseAllowanceReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <IncreaseAllowanceReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IncreaseAllowanceReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for IncreaseAllowanceReturn {
        #[inline]
        fn default() -> IncreaseAllowanceReturn {
            IncreaseAllowanceReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IncreaseAllowanceReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "IncreaseAllowanceReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IncreaseAllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IncreaseAllowanceReturn {
        #[inline]
        fn eq(&self, other: &IncreaseAllowanceReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for IncreaseAllowanceReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for IncreaseAllowanceReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for IncreaseAllowanceReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    pub struct NameReturn(pub ::std::string::String);
    #[automatically_derived]
    impl ::core::clone::Clone for NameReturn {
        #[inline]
        fn clone(&self) -> NameReturn {
            NameReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for NameReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::std::string::String as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for NameReturn {}
    impl ::ethers::core::abi::Tokenizable for NameReturn
    where
        ::std::string::String: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for NameReturn
    where
        ::std::string::String: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for NameReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<NameReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <NameReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <NameReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NameReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for NameReturn {
        #[inline]
        fn default() -> NameReturn {
            NameReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NameReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "NameReturn", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NameReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NameReturn {
        #[inline]
        fn eq(&self, other: &NameReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for NameReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for NameReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::std::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for NameReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    pub struct SymbolReturn(pub ::std::string::String);
    #[automatically_derived]
    impl ::core::clone::Clone for SymbolReturn {
        #[inline]
        fn clone(&self) -> SymbolReturn {
            SymbolReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for SymbolReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::std::string::String as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for SymbolReturn {}
    impl ::ethers::core::abi::Tokenizable for SymbolReturn
    where
        ::std::string::String: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for SymbolReturn
    where
        ::std::string::String: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for SymbolReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<SymbolReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <SymbolReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <SymbolReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SymbolReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for SymbolReturn {
        #[inline]
        fn default() -> SymbolReturn {
            SymbolReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SymbolReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "SymbolReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SymbolReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SymbolReturn {
        #[inline]
        fn eq(&self, other: &SymbolReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for SymbolReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for SymbolReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::std::string::String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SymbolReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    #[automatically_derived]
    impl ::core::clone::Clone for TotalSupplyReturn {
        #[inline]
        fn clone(&self) -> TotalSupplyReturn {
            TotalSupplyReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for TotalSupplyReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <::ethers::core::types::U256 as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for TotalSupplyReturn {}
    impl ::ethers::core::abi::Tokenizable for TotalSupplyReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TotalSupplyReturn
    where
        ::ethers::core::types::U256: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for TotalSupplyReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<
                TotalSupplyReturn,
                ::ethers::core::abi::AbiError,
            > {
                let ::ethers::core::abi::ParamType::Tuple(params) = <TotalSupplyReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <TotalSupplyReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TotalSupplyReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TotalSupplyReturn {
        #[inline]
        fn default() -> TotalSupplyReturn {
            TotalSupplyReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TotalSupplyReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "TotalSupplyReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TotalSupplyReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TotalSupplyReturn {
        #[inline]
        fn eq(&self, other: &TotalSupplyReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TotalSupplyReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TotalSupplyReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<::ethers::core::types::U256>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TotalSupplyReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
    pub struct TransferReturn(pub bool);
    #[automatically_derived]
    impl ::core::clone::Clone for TransferReturn {
        #[inline]
        fn clone(&self) -> TransferReturn {
            TransferReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for TransferReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for TransferReturn {}
    impl ::ethers::core::abi::Tokenizable for TransferReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TransferReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for TransferReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<TransferReturn, ::ethers::core::abi::AbiError> {
                let ::ethers::core::abi::ParamType::Tuple(params) = <TransferReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <TransferReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransferReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TransferReturn {
        #[inline]
        fn default() -> TransferReturn {
            TransferReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransferReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "TransferReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TransferReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TransferReturn {
        #[inline]
        fn eq(&self, other: &TransferReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TransferReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TransferReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TransferReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    pub struct TransferFromReturn(pub bool);
    #[automatically_derived]
    impl ::core::clone::Clone for TransferFromReturn {
        #[inline]
        fn clone(&self) -> TransferFromReturn {
            TransferFromReturn(::core::clone::Clone::clone(&self.0))
        }
    }
    impl ::ethers::core::abi::AbiType for TransferFromReturn {
        fn param_type() -> ::ethers::core::abi::ParamType {
            ::ethers::core::abi::ParamType::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        <bool as ::ethers::core::abi::AbiType>::param_type(),
                    ]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::AbiArrayType for TransferFromReturn {}
    impl ::ethers::core::abi::Tokenizable for TransferFromReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {
        fn from_token(
            token: ::ethers::core::abi::Token,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::InvalidOutputType> {
            if let ::ethers::core::abi::Token::Tuple(tokens) = token {
                if tokens.len() != 1usize {
                    return Err(
                        ::ethers::core::abi::InvalidOutputType({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected 1 tokens, got {0}: {1:?}",
                                    tokens.len(),
                                    tokens,
                                ),
                            );
                            res
                        }),
                    );
                }
                let mut iter = tokens.into_iter();
                Ok(
                    Self(
                        ::ethers::core::abi::Tokenizable::from_token(
                            iter
                                .next()
                                .expect(
                                    "The iter is guaranteed to be something due to the size check",
                                ),
                        )?,
                    ),
                )
            } else {
                Err(
                    ::ethers::core::abi::InvalidOutputType({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected Tuple, got {0:?}", token),
                        );
                        res
                    }),
                )
            }
        }
        fn into_token(self) -> ::ethers::core::abi::Token {
            ::ethers::core::abi::Token::Tuple(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([self.0.into_token()]),
                ),
            )
        }
    }
    impl ::ethers::core::abi::TokenizableItem for TransferFromReturn
    where
        bool: ::ethers::core::abi::Tokenize,
    {}
    impl ::ethers::core::abi::AbiDecode for TransferFromReturn {
        fn decode(
            bytes: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            fn _decode(
                bytes: &[u8],
            ) -> ::core::result::Result<
                TransferFromReturn,
                ::ethers::core::abi::AbiError,
            > {
                let ::ethers::core::abi::ParamType::Tuple(params) = <TransferFromReturn as ::ethers::core::abi::AbiType>::param_type()
                else {
                    ::core::panicking::panic("internal error: entered unreachable code")
                };
                let min_len: usize = params
                    .iter()
                    .map(::ethers::core::abi::minimum_size)
                    .sum();
                if bytes.len() < min_len {
                    Err(
                        ::ethers::core::abi::AbiError::DecodingError(
                            ::ethers::core::abi::ethabi::Error::InvalidData,
                        ),
                    )
                } else {
                    let tokens = ::ethers::core::abi::decode(&params, bytes)?;
                    let tuple = ::ethers::core::abi::Token::Tuple(tokens);
                    let this = <TransferFromReturn as ::ethers::core::abi::Tokenizable>::from_token(
                        tuple,
                    )?;
                    Ok(this)
                }
            }
            _decode(bytes.as_ref())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransferFromReturn {
        fn encode(self) -> ::std::vec::Vec<u8> {
            let tokens = ::ethers::core::abi::Tokenize::into_tokens(self);
            ::ethers::core::abi::encode(&tokens)
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TransferFromReturn {
        #[inline]
        fn default() -> TransferFromReturn {
            TransferFromReturn(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TransferFromReturn {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "TransferFromReturn",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TransferFromReturn {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TransferFromReturn {
        #[inline]
        fn eq(&self, other: &TransferFromReturn) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for TransferFromReturn {}
    #[automatically_derived]
    impl ::core::cmp::Eq for TransferFromReturn {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for TransferFromReturn {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
}
fn main() {
    let body = async {
        let private_key = key_from_index(0);
        let wallet = LocalWallet::from_bytes(&private_key).unwrap().with_chain_id(4_u64);
        let inner_provider = TestInnerProvider::new();
        let contract_address = inner_provider.deploy_contract(CONTRACT_BYTES);
        {
            let contract = inner_provider.contract(contract_address);
            let mut contract = contract.lock().unwrap();
            contract.set_sender(wallet.address());
        }
        let test_provider = TestProvider::new(inner_provider);
        let client = Arc::new(SignerMiddleware::new(test_provider, wallet.clone()));
        let token = Erc20::new(Address::from_low_u64_be(1234), client.clone());
        {
            ::std::io::_print(format_args!("=== init ===\n"));
        };
        token.init(wallet.address(), "Bitcoin".into(), "BTC".into(), 8).await.unwrap();
        {
            ::std::io::_print(format_args!("=== set_minter ===\n"));
        };
        token.set_minter(wallet.address(), true).await.unwrap();
        {
            ::std::io::_print(format_args!("=== mint ===\n"));
        };
        token.mint(wallet.address(), U256::from(3)).await.unwrap();
        {
            ::std::io::_print(format_args!("=== balance_of ===\n"));
        };
        let balance = token.balance_of(wallet.address()).await.unwrap();
        match (&balance, &U256::from(3)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        {
            ::std::io::_print(format_args!("balance: {0:?}\n", balance));
        };
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
