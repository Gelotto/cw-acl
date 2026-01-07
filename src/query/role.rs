//! # Role Query
//!
//! Returns metadata for a specific role.

use crate::{
    error::ContractError, models::AuthRoleInfo, responses::RoleResponse, state::ROLE_INFOS,
};

use super::ReadonlyContext;

pub fn query_role(
    ctx: ReadonlyContext,
    role: String,
) -> Result<RoleResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let AuthRoleInfo {
        description,
        created_at,
        created_by,
        n_principals,
    } = ROLE_INFOS.load(deps.storage, &role)?;

    Ok(RoleResponse {
        expires_at: None,
        name: role,
        description,
        created_at,
        created_by,
        n_principals,
    })
}
