use crate::{
    error::ContractError,
    models::AuthRecord,
    msg::AllowMsg,
    state::{PATH_REF_COUNTS, PRINCIPAL_PATH_AUTHORIZATIONS},
    utils::to_cannonical_path,
};
use cosmwasm_std::{attr, Response};

use super::Context;

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

    let cannonical_path = to_cannonical_path(&path);

    PATH_REF_COUNTS.save(deps.storage, &cannonical_path, &0)?;
    PRINCIPAL_PATH_AUTHORIZATIONS.save(deps.storage, (&principal, &cannonical_path), &auth)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "allow"),
        attr("principal", principal),
        attr("path", cannonical_path),
        attr(
            "expires_at",
            auth.expires_at
                .and_then(|t| Some(t.to_string()))
                .unwrap_or(String::from("null")),
        ),
    ]))
}
