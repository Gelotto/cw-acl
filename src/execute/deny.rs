//! # Deny Path from Principal
//!
//! Removes direct path authorization from a principal.

use crate::{
    error::ContractError,
    msg::DenyMsg,
    state::PRINCIPAL_PATH_AUTHORIZATIONS,
    utils::{decrement_or_remove_path_ref_count, to_canonical_path},
};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Removes a principal's authorization to a specific path.
///
/// Decrements path reference count and removes the authorization record.
pub fn exec_deny(
    ctx: Context,
    msg: DenyMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let DenyMsg { principal, path } = msg;
    let canonical_path = to_canonical_path(&path);

    decrement_or_remove_path_ref_count(deps.storage, &canonical_path)?;

    // Disassociate the path from the principal
    PRINCIPAL_PATH_AUTHORIZATIONS.remove(deps.storage, (&principal, &canonical_path));

    Ok(Response::new().add_attributes(vec![
        attr("action", "deny"),
        attr("path", canonical_path),
        attr("principal", principal),
    ]))
}
