use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IdentityMetadata {
    pub address: Addr,  // user wallet address
    pub name: String,   // user displayed nickname
    pub about: String,  // text about the user
    pub pic: String,    // ipfs image hash
    pub avatar: String, // 3d animated model ipfs hash
}
