//! # Roles Query
//!
//! Lists all roles or roles for a specific principal.

use cosmwasm_std::Order;

use crate::{
    error::ContractError,
    models::{AuthRecord, AuthRoleInfo},
    responses::{RoleResponse, RolesResponse},
    state::{PRINCIPAL_ROLE_AUTHORIZATIONS, ROLE_INFOS},
};

use super::ReadonlyContext;

pub fn query_roles(
    ctx: ReadonlyContext,
    principal: Option<String>,
) -> Result<RolesResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let mut role_resps: Vec<RoleResponse> = Vec::with_capacity(2);

    if let Some(principal) = principal {
        // Return roles associated with the given principal
        for result in PRINCIPAL_ROLE_AUTHORIZATIONS.prefix(&principal).range(
            deps.storage,
            None,
            None,
            Order::Ascending,
        ) {
            let (name, AuthRecord { expires_at }) = result?;
            let AuthRoleInfo {
                description,
                created_at,
                created_by,
                n_principals,
            } = ROLE_INFOS.load(deps.storage, &name)?;
            role_resps.push(RoleResponse {
                expires_at,
                description,
                created_at,
                created_by,
                n_principals,
                name,
            });
        }
    } else {
        // Return ALL roles in the ACL
        for result in ROLE_INFOS.range(deps.storage, None, None, Order::Ascending) {
            let (
                name,
                AuthRoleInfo {
                    description,
                    created_at,
                    created_by,
                    n_principals,
                },
            ) = result?;

            role_resps.push(RoleResponse {
                expires_at: None,
                description,
                created_at,
                created_by,
                n_principals,
                name,
            });
        }
    }

    Ok(RolesResponse(role_resps))
}
