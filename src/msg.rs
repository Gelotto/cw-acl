//! # Message Types
//!
//! Defines all InstantiateMsg, ExecuteMsg, and QueryMsg types for the ACL contract.

use cosmwasm_schema::cw_serde;

use crate::client::Operator;

#[cw_serde]
pub struct InstantiateMsg {
    pub operator: Option<Operator>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[cw_serde]
pub enum RoleExecuteMsg {
    /// Initialize a role with optional initial paths and principals.
    Create(CreateRoleMsg),
    /// Authorize a principal to a given path.
    Allow(AllowRoleMsg),
    /// Deny a path to an existing role (inverse of Allow).
    Deny(DenyRoleMsg),
    /// Grant a role to a given principal, allowing the principal to inherit all
    /// paths allowed to the role.
    Grant(GrantRoleMsg),
    // The inverse of Grant.
    Revoke(RevokeRoleMsg),
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change the operator of the ACL. This is the contract or account who can
    /// execute the ACL.
    SetOperator(Operator),
    /// Authorize a principal to a given path.
    Allow(AllowMsg),
    /// This is the inverse of Allow.
    Deny(DenyMsg),
    /// Execute a change pertaining to a role.
    Role(RoleExecuteMsg),
}

#[cw_serde]
pub enum QueryMsg {
    /// Get top-level ACL info and metadata.
    Acl {},
    /// List roles pertaining to a given principal or, if not provided, all
    /// roles defined by the ACL.
    Roles { principal: Option<String> },
    /// Get role information from role name.
    Role(String),
    /// List paths autorized to a principal, role, or the ACL as whole.
    Paths(PathsQueryParams),
    /// Test if a given principal is allowed with respect to one or more paths.
    IsAllowed(IsAllowedParams),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum Subject {
    Acl,
    Role(String),
    Principal(String),
}

#[cw_serde]
pub struct PathsQueryParams {
    pub subject: Subject,
    pub limit: Option<u16>,
    pub start: Option<String>,
    pub stop: Option<String>,
    pub cursor: Option<String>,
}

#[cw_serde]
pub struct AllowMsg {
    pub principal: String,
    pub path: String,
    pub ttl: Option<u32>,
}

#[cw_serde]
pub struct GrantRoleMsg {
    pub principal: String,
    pub role: String,
    pub ttl: Option<u32>,
}

#[cw_serde]
pub struct RevokeRoleMsg {
    pub principal: String,
    pub role: String,
}

#[cw_serde]
pub struct AllowRoleMsg {
    pub role: String,
    pub path: String,
}

#[cw_serde]
pub struct DenyMsg {
    pub principal: String,
    pub path: String,
}

#[cw_serde]
pub struct DenyRoleMsg {
    pub role: String,
    pub path: String,
}

#[cw_serde]
pub struct AuthorizationResourceParams {
    pub resource: AuthResource,
    pub ttl: Option<u32>,
}

#[cw_serde]
pub enum AuthResource {
    Role(String),
    Path(String),
}

/// Defines how multiple path checks are combined in IsAllowed queries.
#[cw_serde]
pub enum TestRequirement {
    /// At least one path must be authorized (OR logic)
    Any,
    /// All paths must be authorized (AND logic)
    All,
}

#[cw_serde]
pub struct IsAllowedParams {
    pub principal: String,
    pub require: Option<TestRequirement>,
    pub paths: Vec<String>,
    pub raise: Option<bool>,
}

#[cw_serde]
pub struct CreateRoleMsg {
    pub name: String,
    pub description: Option<String>,
    pub paths: Option<Vec<String>>,
}
