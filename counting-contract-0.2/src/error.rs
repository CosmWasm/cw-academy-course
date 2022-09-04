use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },

    #[error("Invalid contract to migrate from: {contract}")]
    InvalidContract { contract: String },

    #[error("Unsupported contract version for migration: {version}")]
    InvalidContractVersion { version: String },
}
