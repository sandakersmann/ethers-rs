#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![warn(missing_docs)]

mod contract;
pub use contract::{Contract, ContractInstance};

mod base;
pub use base::{decode_function_data, encode_function_data, AbiError, BaseContract};

mod call;
pub use call::{ContractCall, ContractError, EthCall, FunctionCall};

mod error;
pub use error::EthError;

mod factory;
pub use factory::{ContractDeployer, ContractDeploymentTx, ContractFactory, DeploymentTxFactory};

mod event;
pub use event::{EthEvent, Event};

mod log;
pub use log::{decode_logs, EthLogDecode, LogMeta};

pub mod stream;

#[cfg(any(test, feature = "abigen"))]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
mod multicall;
#[cfg(any(test, feature = "abigen"))]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
pub use multicall::{
    constants::{MULTICALL_ADDRESS, MULTICALL_SUPPORTED_CHAIN_IDS},
    contract as multicall_contract,
    error::MulticallError,
    Call, Multicall, MulticallContract, MulticallVersion,
};

/// This module exposes low lever builder structures which are only consumed by the
/// type-safe ABI bindings generators.
#[doc(hidden)]
pub mod builders {
    pub use super::{
        call::ContractCall,
        event::Event,
        factory::{ContractDeployer, Deployer},
    };
}

#[cfg(any(test, feature = "abigen"))]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
pub use ethers_contract_abigen::{
    Abigen, ContractFilter, ExcludeContracts, InternalStructs, MultiAbigen, SelectContracts,
};

#[cfg(any(test, feature = "abigen"))]
#[cfg_attr(docsrs, doc(cfg(feature = "abigen")))]
pub use ethers_contract_derive::{
    abigen, EthAbiCodec, EthAbiType, EthCall, EthDisplay, EthError, EthEvent,
};

// Hide the Lazy re-export, it's just for convenience
#[doc(hidden)]
pub use once_cell::sync::Lazy;

#[cfg(feature = "eip712")]
pub use ethers_derive_eip712::*;
