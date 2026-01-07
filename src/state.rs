//! # State Management
//!
//! Storage schema for the ACL contract:
//! - `OP`: Contract operator (can modify ACL)
//! - `CREATED_BY`, `CREATED_AT`, `NAME`, `DESCRIPTION`: Metadata
//! - `PATH_REF_COUNTS`: Tracks how many principals/roles reference each path
//! - `PRINCIPAL_PATH_AUTHORIZATIONS`: Direct path grants to principals
//! - `PRINCIPAL_ROLE_AUTHORIZATIONS`: Role memberships for principals
//! - `ROLE_INFOS`: Role metadata and principal counts
//! - `ROLE_PATHS`: Paths allowed for each role (bidirectional with PATH_ROLES)
//! - `PATH_ROLES`: Roles allowed for each path (bidirectional with ROLE_PATHS)

use cosmwasm_std::{attr, Addr, Response, Timestamp};
use cw_storage_plus::{Item, Map};

use crate::{
    client::Operator,
    error::ContractError,
    execute::Context,
    models::{AuthRecord, AuthRoleInfo},
    msg::InstantiateMsg,
};

/// Role name identifier
type Role = String;
/// Principal address or identifier
type Principal = String;
/// Canonical path string (normalized with leading slash)
type Path = String;

pub const MAX_NAME_LEN: usize = 100;
pub const MAX_DESC_LEN: usize = 1000;

/// The operator (address or ACL) who can execute changes to this ACL
pub const OP: Item<Operator> = Item::new("op");
/// Address that instantiated the contract
pub const CREATED_BY: Item<Addr> = Item::new("created_by");
/// Timestamp when contract was instantiated
pub const CREATED_AT: Item<Timestamp> = Item::new("created_at");
/// Optional ACL name (max 100 chars)
pub const NAME: Item<String> = Item::new("name");
/// Optional ACL description (max 1000 chars)
pub const DESCRIPTION: Item<String> = Item::new("desc");

/// Reference count for each path across all principals and roles.
/// Used to track when a path can be safely removed from PATH_REF_COUNTS.
pub const PATH_REF_COUNTS: Map<&Path, u32> = Map::new("prc");

/// Direct path authorizations for principals (not via roles).
pub const PRINCIPAL_PATH_AUTHORIZATIONS: Map<(&Principal, &Path), AuthRecord> = Map::new("ppa");

/// Role memberships for principals.
pub const PRINCIPAL_ROLE_AUTHORIZATIONS: Map<(&Principal, &Role), AuthRecord> = Map::new("pra");

/// Metadata for each role.
pub const ROLE_INFOS: Map<&Role, AuthRoleInfo> = Map::new("ri");

/// Paths authorized to each role. The u8 value is unused (just a marker).
/// Bidirectional index with PATH_ROLES.
pub const ROLE_PATHS: Map<(&Role, &Path), u8> = Map::new("rp");

/// Roles authorized to each path. The u8 value is unused (just a marker).
/// Bidirectional index with ROLE_PATHS.
pub const PATH_ROLES: Map<(&Path, &Role), u8> = Map::new("pr");

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, info } = ctx;
    let InstantiateMsg {
        operator,
        name,
        description,
    } = msg;

    // Validate operator
    let operator = if let Some(op) = &operator {
        deps.api.addr_validate(
            match op {
                Operator::Address(addr) => addr,
                Operator::Acl(addr) => addr,
            }
            .as_str(),
        )?;
        op.to_owned()
    } else {
        Operator::Address(info.sender.clone())
    };

    // Set ACL name
    if let Some(name) = &name {
        if name.len() > MAX_NAME_LEN {
            return Err(ContractError::ValidationError {
                reason: format!("ACL name cannot be longer than {} characters", MAX_NAME_LEN),
            });
        }
        NAME.save(deps.storage, name)?;
    }

    // Set ACL description
    if let Some(desc) = &description {
        if desc.len() > MAX_DESC_LEN {
            return Err(ContractError::ValidationError {
                reason: format!(
                    "ACL description cannot be longer than {} characters",
                    MAX_DESC_LEN
                ),
            });
        }
        DESCRIPTION.save(deps.storage, desc)?;
    }

    OP.save(deps.storage, &operator)?;
    CREATED_AT.save(deps.storage, &env.block.time)?;
    CREATED_BY.save(deps.storage, &info.sender)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("acl_operator", operator.to_string()),
        attr("acl_name", name.unwrap_or_default()),
    ]))
}
