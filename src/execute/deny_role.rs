use crate::{
    error::ContractError,
    msg::DenyRoleMsg,
    state::{PATH_ROLES, ROLE_PATHS},
    utils::{decrement_or_remove_path_ref_count, to_cannonical_path},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_deny_role(
    ctx: Context,
    msg: DenyRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let DenyRoleMsg { role, path } = msg;

    let cannonical_path = to_cannonical_path(&path);

    decrement_or_remove_path_ref_count(deps.storage, &cannonical_path)?;

    ROLE_PATHS.remove(deps.storage, (&role, &cannonical_path));
    PATH_ROLES.remove(deps.storage, (&cannonical_path, &role));

    Ok(Response::new().add_attributes(vec![
        attr("action", "deny_role"),
        attr("role", role),
        attr("path", cannonical_path),
    ]))
}
