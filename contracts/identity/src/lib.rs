use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use error::ContractError;
use models::InstantiateMsg;
use msg::{ExecuteMsg, QueryMsg};
pub mod error;
pub mod exec;
mod models;
pub mod msg;
pub mod que;
pub mod states;
pub mod tests;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMetadata { identity_data } => {
            exec::upsert_identity(deps, env, info, identity_data)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserInfo { address } => to_json_binary(&que::query_identity(deps, address)?),
        QueryMsg::UserInfoAll {} => to_json_binary(&que::query_all_identities(deps)?),
    }
}
