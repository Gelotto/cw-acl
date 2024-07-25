use crate::{
    error::ContractError,
    models::AuthRoleInfo,
    msg::CreateRoleMsg,
    state::{PATH_REF_COUNTS, PATH_ROLES, ROLE_INFOS, ROLE_PATHS},
    utils::to_cannonical_path,
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_create_role(
    ctx: Context,
    msg: CreateRoleMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, info } = ctx;
    let CreateRoleMsg {
        name: role,
        description,
        paths,
    } = msg;

    ROLE_INFOS.update(
        deps.storage,
        &role,
        |maybe_info| -> Result<_, ContractError> {
            if maybe_info.is_some() {
                return Err(ContractError::NotAuthorized {
                    reason: format!("role {} already exists", role),
                });
            }
            Ok(AuthRoleInfo {
                created_at: env.block.time,
                created_by: info.sender,
                n_principals: 0,
                description,
            })
        },
    )?;

    for path in paths.unwrap_or_default().iter() {
        let cannonical_path = to_cannonical_path(path);

        PATH_REF_COUNTS.save(deps.storage, &cannonical_path, &0)?;
        ROLE_PATHS.save(deps.storage, (&role, &cannonical_path), &0)?;
        PATH_ROLES.save(deps.storage, (&cannonical_path, &role), &0)?;
    }

    Ok(Response::new().add_attributes(vec![attr("action", "create_role"), attr("role", role)]))
}
