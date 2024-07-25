use crate::{
    error::ContractError,
    state::{PATH_ROLES, ROLE_INFOS, ROLE_PATHS},
    utils::decrement_or_remove_path_ref_count,
};
use cosmwasm_std::{attr, Order, Response};

use super::Context;

pub fn exec_remove_role(
    ctx: Context,
    role: String,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;

    let paths_to_remove: Vec<String> = PATH_ROLES
        .prefix(&role)
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|k| k.unwrap())
        .collect();

    ROLE_INFOS.remove(deps.storage, &role);

    for path in paths_to_remove.iter() {
        decrement_or_remove_path_ref_count(deps.storage, &path)?;
        ROLE_PATHS.remove(deps.storage, (&role, &path));
        PATH_ROLES.remove(deps.storage, (&path, &role));
    }

    Ok(Response::new().add_attributes(vec![attr("action", "remove_role"), attr("role", role)]))
}
