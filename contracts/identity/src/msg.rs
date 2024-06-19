use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::IdentityMetadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint { identity_data: IdentityMetadata },
    UpdateMetadata { identity_data: IdentityMetadata },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IdentityMetadata)]
    UserInfo { address: Addr },
    #[returns(Vec<(Addr, IdentityMetadata)>)]
    UserInfoAll {},
}
