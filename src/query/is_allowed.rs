use cosmwasm_std::{Order, Storage, Timestamp};

use crate::{
    error::ContractError,
    models::AuthRecord,
    msg::{IsAllowedParams, TestRequirement},
    state::{PATH_ROLES, PRINCIPAL_PATH_AUTHORIZATIONS, PRINCIPAL_ROLE_AUTHORIZATIONS},
    utils::{to_cannonical_path, to_cannonical_path_from_crumbs},
};

use super::ReadonlyContext;

/// Query that checks if a given principal is authorized to a list of given
/// roles and/or paths. In the case of paths, we check first for direct
/// authorization or authorization via any assigned roles. Authorization is
/// heirarchical, so for each path, we're checking first if the user is authed
/// to the full path and then checking again if they are authed via any ancestor
/// paths.
pub fn query_is_authorized(
    ctx: ReadonlyContext,
    msg: IsAllowedParams,
) -> Result<bool, ContractError> {
    let ReadonlyContext { deps, env, .. } = ctx;
    let IsAllowedParams {
        principal,
        paths,
        require,
        raise,
    } = msg;

    // Replace optional args with defaults
    let require = require.unwrap_or(TestRequirement::All);
    let raise = raise.unwrap_or(false);

    // Storage for error messages generated below
    let mut error_msgs: Vec<String> = Vec::with_capacity(paths.len());

    // Check if principal has authorization for each role or path provided.
    for p in paths.iter() {
        // Return a result containing a error message string in an Err if not
        // authorized to the given role or path.
        if let Err(error_msg) = try_authorize_path(deps.storage, env.block.time, &principal, &p) {
            // If we require ALL checks to pass, fail if we've got an error
            if require == TestRequirement::All {
                if raise {
                    return Err(ContractError::NotAuthorized { reason: error_msg });
                } else {
                    return Ok(false);
                }
            } else {
                error_msgs.push(error_msg)
            }
        }
    }

    // If we're here, it means that the test mode is ANY, implying that all we
    // require is a single test to pass. If none have passed, however, we fail
    // the aggregate auth check.
    if error_msgs.len() == paths.len() {
        if raise {
            return Err(ContractError::NotAuthorized {
                reason: error_msgs.join(", "),
            });
        } else {
            return Ok(false);
        }
    }

    // Principal is authorized!
    Ok(true)
}

/// First, ensure principal is authorized to the given path directly; however,
/// if there is no direct authorization, first check if prinicipal is authorized
/// transitively through any inherited roles. If not, then we return an error.
fn try_authorize_path(
    store: &dyn Storage,
    time: Timestamp,
    principal: &String,
    path: &String,
) -> Result<(), String> {
    let mut crumbs: Vec<&str> = path.trim_matches('/').split("/").collect();

    // Iterate from full path up the tree of parent paths so that the most
    // specific set of authorization parameters "overrides" the parameters of
    // its parents.
    while !crumbs.is_empty() {
        let cannonical_path = to_cannonical_path_from_crumbs(&crumbs);

        let maybe_assignment = PRINCIPAL_PATH_AUTHORIZATIONS
            .load(store, (principal, &cannonical_path))
            .ok();

        // If there's an auth record for principal to the path directly, ensure
        // that it is valid here.
        if let Some(assignment) = maybe_assignment {
            if let Some(expiry) = assignment.expires_at {
                if time >= expiry {
                    return Err(format!(
                        "{} access to {} has expired",
                        principal, cannonical_path
                    ));
                }
            }
            return Ok(()); // authorized
        } else {
            // Otherwise, check for authorization via any roles inherited by
            // prinicipal before erroring out.
            let roles: Vec<String> = PATH_ROLES
                .prefix(&cannonical_path)
                .keys(store, None, None, Order::Ascending)
                .map(|r| r.unwrap())
                .collect();

            // For any roles assigned this path, check if prinicap has it and
            // the assignment the role hasn't expired.
            for role in roles {
                if let Some(AuthRecord { expires_at: expiry }) = PRINCIPAL_ROLE_AUTHORIZATIONS
                    .may_load(store, (principal, &role))
                    .unwrap_or(None)
                {
                    if let Some(expiry) = expiry {
                        if time >= expiry {
                            return Err(format!("{} role {} has expired", principal, role));
                        }
                    } else {
                        return Ok(()); // authorized
                    }
                }
            }
        }

        crumbs.pop();
    }

    Err(format!(
        "{} not authorized to {}",
        principal,
        to_cannonical_path(path)
    ))
}
