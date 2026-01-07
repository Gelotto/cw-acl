//! # Query Message Handlers
//!
//! Handlers for all QueryMsg variants. Each handler operates on a ReadonlyContext
//! containing Deps and Env.

pub mod acl;
pub mod is_allowed;
pub mod paths;
pub mod role;
pub mod roles;

use cosmwasm_std::{Deps, Env};

/// Query context bundling dependencies for query handlers.
pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
