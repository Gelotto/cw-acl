//! # Deny Path from Role
//!
//! Removes a path authorization from a role definition.

use crate::{
    error::ContractError,
    msg::DenyRoleMsg,
    state::{PATH_ROLES, ROLE_PATHS},
    utils::{decrement_or_remove_path_ref_count, to_canonical_path},
};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Removes a path from a role's authorized paths.
///
/// Principals with this role lose access to the path (unless authorized via other means).
pub fn exec_deny_role(
    ctx: Context,
    msg: DenyRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let DenyRoleMsg { role, path } = msg;

    let canonical_path = to_canonical_path(&path);

    decrement_or_remove_path_ref_count(deps.storage, &canonical_path)?;

    ROLE_PATHS.remove(deps.storage, (&role, &canonical_path));
    PATH_ROLES.remove(deps.storage, (&canonical_path, &role));

    Ok(Response::new().add_attributes(vec![
        attr("action", "deny_role"),
        attr("role", role),
        attr("path", canonical_path),
    ]))
}
