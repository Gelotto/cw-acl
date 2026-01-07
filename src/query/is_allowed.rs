//! # Authorization Query - Hierarchical Path Checking
//!
//! Implements the core authorization logic with hierarchical path matching.
//!
//! ## Algorithm:
//!
//! For each path to check:
//! 1. Split path into segments ("crumbs"): `/foo/bar/baz` → `["foo", "bar", "baz"]`
//! 2. Check from most specific to least specific:
//!    - First check `/foo/bar/baz`
//!    - Then check `/foo/bar`
//!    - Then check `/foo`
//! 3. At each level, check both:
//!    a. Direct principal authorization to that path
//!    b. Role-based authorization (principal has role with that path)
//! 4. First match wins (most specific authorization)
//!
//! ## Examples:
//!
//! If Alice is authorized to `/foo`, she is also authorized to:
//! - `/foo/bar`
//! - `/foo/bar/baz`
//! - Any path under `/foo/*`
//!
//! If Alice has role `admin` which is authorized to `/users`, and the role
//! hasn't expired, Alice can access `/users/123/profile`.

use cosmwasm_std::{Order, Storage, Timestamp};

use crate::{
    error::ContractError,
    models::AuthRecord,
    msg::{IsAllowedParams, TestRequirement},
    state::{PATH_ROLES, PRINCIPAL_PATH_AUTHORIZATIONS, PRINCIPAL_ROLE_AUTHORIZATIONS},
    utils::{to_canonical_path, to_canonical_path_from_crumbs},
};

use super::ReadonlyContext;

/// Checks if a principal is authorized to one or more paths.
///
/// # Parameters:
/// - `principal`: Address to check authorization for
/// - `paths`: List of paths to check
/// - `require`: Combination logic (Any = OR, All = AND)
/// - `raise`: If true, returns error on failure; if false, returns boolean
///
/// # Returns:
/// - `Ok(true)` if authorized according to `require` logic
/// - `Ok(false)` if not authorized and `raise` is false
/// - `Err(NotAuthorized)` if not authorized and `raise` is true
pub fn query_is_allowed(
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

/// Attempts to authorize a principal to a specific path using hierarchical path matching.
///
/// Checks from most specific path to least specific (walking up the tree), checking both
/// direct authorization and role-based authorization at each level.
fn try_authorize_path(
    store: &dyn Storage,
    time: Timestamp,
    principal: &String,
    path: &String,
) -> Result<(), String> {
    // Split path into segments for hierarchical traversal
    // e.g., "/foo/bar/baz" -> ["foo", "bar", "baz"]
    let mut crumbs: Vec<&str> = path.trim_matches('/').split("/").collect();

    // Walk from full path up to root, checking authorization at each level.
    // Most specific path wins (e.g., check /foo/bar/baz, then /foo/bar, then /foo)
    while !crumbs.is_empty() {
        let canonical_path = to_canonical_path_from_crumbs(&crumbs);

        // Check for direct path authorization
        let maybe_assignment = PRINCIPAL_PATH_AUTHORIZATIONS
            .load(store, (principal, &canonical_path))
            .ok();

        if let Some(assignment) = maybe_assignment {
            // Found direct authorization - check if it's expired
            if let Some(expiry) = assignment.expires_at {
                if time >= expiry {
                    return Err(format!(
                        "{} access to {} has expired",
                        principal, canonical_path
                    ));
                }
            }
            return Ok(()); // Authorized via direct path grant
        } else {
            // No direct authorization - check if principal has any roles
            // that grant access to this path
            let roles: Vec<String> = PATH_ROLES
                .prefix(&canonical_path)
                .keys(store, None, None, Order::Ascending)
                .map(|r| r.unwrap())
                .collect();

            // Check if principal has any of these roles and the role grant hasn't expired
            for role in roles {
                if let Some(AuthRecord { expires_at: expiry }) = PRINCIPAL_ROLE_AUTHORIZATIONS
                    .may_load(store, (principal, &role))
                    .unwrap_or(None)
                {
                    if let Some(expiry) = expiry {
                        if time < expiry {
                            return Ok(()); // Not expired yet - authorized
                        }
                        // If expired, continue checking other roles
                    } else {
                        return Ok(()); // No expiry - authorized
                    }
                }
            }
        }

        // Move up one level in the path hierarchy
        crumbs.pop();
    }

    Err(format!(
        "{} not authorized to {}",
        principal,
        to_canonical_path(path)
    ))
}
