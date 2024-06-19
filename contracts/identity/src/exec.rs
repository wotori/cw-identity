use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, models::IdentityMetadata, states::IDENTITIES};

pub fn upsert_identity(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    identity_data: IdentityMetadata,
) -> Result<Response, ContractError> {
    let address = info.sender;

    let existing_identity = IDENTITIES.may_load(deps.storage, &address)?;

    if existing_identity.is_some() {
        // Identity exists, update it
        IDENTITIES.save(deps.storage, &address, &identity_data)?;
        Ok(Response::new().add_attribute("action", "update_metadata"))
    } else {
        // Identity does not exist, create new one
        IDENTITIES.save(deps.storage, &address, &identity_data)?;
        Ok(Response::new().add_attribute("action", "mint_identity"))
    }
}
