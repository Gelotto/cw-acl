//! # Allow Path to Role
//!
//! Adds a path authorization to a role definition.

use crate::{
    error::ContractError,
    msg::AllowRoleMsg,
    state::{PATH_REF_COUNTS, PATH_ROLES, ROLE_PATHS},
    utils::to_canonical_path,
};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Authorizes a role to access a specific path.
///
/// All principals with this role inherit access to the path.
/// Creates bidirectional role-path indices.
pub fn exec_allow_role(
    ctx: Context,
    msg: AllowRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let AllowRoleMsg { role, path } = msg;

    let canonical_path = to_canonical_path(&path);

    PATH_REF_COUNTS.save(deps.storage, &canonical_path, &0)?;
    ROLE_PATHS.save(deps.storage, (&role, &canonical_path), &0)?;
    PATH_ROLES.save(deps.storage, (&canonical_path, &role), &0)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "allow_role"),
        attr("role", role),
        attr("path", canonical_path),
    ]))
}
