//! # Query Response Types
//!
//! Response structures returned by query handlers.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

use crate::{client::Operator, models::Config};

#[cw_serde]
pub struct AclResponse {
    pub operator: Operator,
    pub created_by: Addr,
    pub created_at: Timestamp,
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Config,
}

#[cw_serde]
pub struct RoleResponse {
    pub name: String,
    pub description: Option<String>,
    pub created_at: Timestamp,
    pub created_by: Addr,
    pub n_principals: u32,
    pub expires_at: Option<Timestamp>,
}

#[cw_serde]
pub struct RolesResponse(pub Vec<RoleResponse>);

#[cw_serde]
pub struct PathsResponse {
    pub cursor: Option<String>,
    pub paths: Vec<PathInfo>,
}

#[cw_serde]
pub struct PathInfo {
    pub path: String,
    pub expires_at: Option<Timestamp>,
}
