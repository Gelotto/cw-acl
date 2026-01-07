//! # Data Models
//!
//! Core data structures stored in contract state.

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

/// Placeholder for future configuration options.
///
/// Currently empty but returned in ACL queries for forward compatibility.
#[cw_serde]
pub struct Config {}

/// Metadata for a role.
#[cw_serde]
pub struct AuthRoleInfo {
    /// Optional description of the role's purpose
    pub description: Option<String>,
    /// Block timestamp when role was created
    pub created_at: Timestamp,
    /// Address that created the role
    pub created_by: Addr,
    /// Number of principals currently granted this role
    pub n_principals: u32,
}

/// Authorization record with optional expiration.
#[cw_serde]
pub struct AuthRecord {
    /// If set, authorization expires at this timestamp
    pub expires_at: Option<Timestamp>,
}
