use crate::{
    error::ContractError,
    msg::AllowRoleMsg,
    state::{PATH_REF_COUNTS, PATH_ROLES, ROLE_PATHS},
    utils::to_cannonical_path,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_allow_role(
    ctx: Context,
    msg: AllowRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let AllowRoleMsg { role, path } = msg;

    let cannonical_path = to_cannonical_path(&path);

    PATH_REF_COUNTS.save(deps.storage, &cannonical_path, &0)?;
    ROLE_PATHS.save(deps.storage, (&role, &cannonical_path), &0)?;
    PATH_ROLES.save(deps.storage, (&cannonical_path, &role), &0)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "allow_role"),
        attr("role", role),
        attr("path", cannonical_path),
    ]))
}
