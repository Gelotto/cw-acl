use crate::{
    error::ContractError,
    msg::DenyMsg,
    state::PRINCIPAL_PATH_AUTHORIZATIONS,
    utils::{decrement_or_remove_path_ref_count, to_cannonical_path},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_deny(
    ctx: Context,
    msg: DenyMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let DenyMsg { principal, path } = msg;
    let cannonical_path = to_cannonical_path(&path);

    decrement_or_remove_path_ref_count(deps.storage, &cannonical_path)?;

    // Disassciate the path from the principal
    PRINCIPAL_PATH_AUTHORIZATIONS.remove(deps.storage, (&principal, &cannonical_path));

    Ok(Response::new().add_attributes(vec![
        attr("action", "deny"),
        attr("path", cannonical_path),
        attr("principal", principal),
    ]))
}
