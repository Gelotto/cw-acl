//! # Grant Role to Principal
//!
//! Assigns a role to a principal with optional TTL.

use crate::{
    error::ContractError,
    math::add_u32,
    models::AuthRecord,
    msg::GrantRoleMsg,
    state::{PRINCIPAL_ROLE_AUTHORIZATIONS, ROLE_INFOS},
};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Grants a role to a principal, incrementing the role's principal count.
///
/// Fails if role doesn't exist. Creates authorization record with optional expiration.
pub fn exec_grant_role(
    ctx: Context,
    msg: GrantRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let GrantRoleMsg {
        principal,
        role,
        ttl,
    } = msg;

    let auth = AuthRecord {
        expires_at: ttl.and_then(|n| Some(env.block.time.plus_seconds(n.into()))),
    };

    ROLE_INFOS.update(
        deps.storage,
        &role,
        |maybe_info| -> Result<_, ContractError> {
            if let Some(mut info) = maybe_info {
                info.n_principals = add_u32(info.n_principals, 1)?;
                Ok(info)
            } else {
                Err(ContractError::NotAuthorized {
                    reason: format!("role {} does not exist", role),
                })
            }
        },
    )?;

    PRINCIPAL_ROLE_AUTHORIZATIONS.save(deps.storage, (&principal, &role), &auth)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "grant_role"),
        attr("principal", principal),
        attr("role", role),
        attr(
            "expires_at",
            auth.expires_at
                .and_then(|t| Some(t.to_string()))
                .unwrap_or(String::from("null")),
        ),
    ]))
}
