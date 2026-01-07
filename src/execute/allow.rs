//! # Allow Path to Principal
//!
//! Grants direct path authorization to a principal with optional TTL.

use crate::{
    error::ContractError,
    models::AuthRecord,
    msg::AllowMsg,
    state::{PATH_REF_COUNTS, PRINCIPAL_PATH_AUTHORIZATIONS},
    utils::to_canonical_path,
};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Authorizes a principal to access a specific path.
///
/// Creates both:
/// - Path reference count entry (or increments existing)
/// - Principal-path authorization record with optional expiration
pub fn exec_allow(
    ctx: Context,
    msg: AllowMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;
    let AllowMsg {
        principal,
        path,
        ttl,
    } = msg;

    let auth = AuthRecord {
        expires_at: ttl.and_then(|n| Some(env.block.time.plus_seconds(n.into()))),
    };

    let canonical_path = to_canonical_path(&path);

    PATH_REF_COUNTS.save(deps.storage, &canonical_path, &0)?;
    PRINCIPAL_PATH_AUTHORIZATIONS.save(deps.storage, (&principal, &canonical_path), &auth)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "allow"),
        attr("principal", principal),
        attr("path", canonical_path),
        attr(
            "expires_at",
            auth.expires_at
                .and_then(|t| Some(t.to_string()))
                .unwrap_or(String::from("null")),
        ),
    ]))
}
