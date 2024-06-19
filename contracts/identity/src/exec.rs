use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, models::IdentityMetadata, states::IDENTITIES};

pub fn mint_identity(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    identity_data: IdentityMetadata,
) -> Result<Response, ContractError> {
    let address = info.sender;

    if IDENTITIES.may_load(deps.storage, &address)?.is_some() {
        return Err(ContractError::IdentityAlreadyExists {});
    }

    IDENTITIES.save(deps.storage, &address, &identity_data)?;
    Ok(Response::new().add_attribute("action", "mint_identity"))
}

pub fn update_metadata(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    identity_data: IdentityMetadata,
) -> Result<Response, ContractError> {
    let address = info.sender;

    if IDENTITIES.may_load(deps.storage, &address)?.is_none() {
        return Err(ContractError::IdentityNotFound {});
    }

    IDENTITIES.save(deps.storage, &address, &identity_data)?;
    Ok(Response::new().add_attribute("action", "update_metadata"))
}
