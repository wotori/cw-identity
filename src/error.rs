use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),

    #[error("The project is closed to collaboration. Please reach out to the project owner.")]
    Unauthorized {},

    #[error("Identity already exists for this address")]
    IdentityAlreadyExists {},

    #[error("Identity not found. Create new identity before modify.")]
    IdentityNotFound {},
}
