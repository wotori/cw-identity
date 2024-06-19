use cosmwasm_std::Addr;
use cw_storage_plus::Map;

use crate::models::IdentityMetadata;

pub const IDENTITIES: Map<&Addr, IdentityMetadata> = Map::new("identities");
