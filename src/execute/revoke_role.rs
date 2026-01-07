//! # Revoke Role from Principal
//!
//! Removes a role assignment from a principal.

use cosmwasm_std::{attr, Response};

use crate::{
    error::ContractError,
    math::sub_u32,
    msg::RevokeRoleMsg,
    state::{PRINCIPAL_ROLE_AUTHORIZATIONS, ROLE_INFOS},
};

use super::Context;

/// Revokes a principal's role membership, decrementing the role's principal count.
pub fn exec_revoke_role(
    ctx: Context,
    msg: RevokeRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let RevokeRoleMsg { principal, role } = msg;

    // Decrement the total number of principals associated with the role
    ROLE_INFOS.update(
        deps.storage,
        &role,
        |maybe_info| -> Result<_, ContractError> {
            if let Some(mut info) = maybe_info {
                info.n_principals = sub_u32(info.n_principals, 1)?;
                Ok(info)
            } else {
                Err(ContractError::NotAuthorized {
                    reason: format!("role {} does not exist", role),
                })
            }
        },
    )?;

    // Disassociate the role from the principal
    PRINCIPAL_ROLE_AUTHORIZATIONS.remove(deps.storage, (&principal, &role));

    Ok(Response::new().add_attributes(vec![
        attr("action", "revoke_role"),
        attr("principal", principal),
        attr("role", role),
    ]))
}
