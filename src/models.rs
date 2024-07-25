use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct AuthRoleInfo {
    pub description: Option<String>,
    pub created_at: Timestamp,
    pub created_by: Addr,
    pub n_principals: u32,
}

#[cw_serde]
pub struct AuthRecord {
    pub expires_at: Option<Timestamp>,
}
