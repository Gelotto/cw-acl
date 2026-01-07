//! # Execute Message Handlers
//!
//! Handlers for all ExecuteMsg variants. Each handler operates on a Context
//! containing DepsMut, Env, and MessageInfo.

pub mod allow;
pub mod allow_role;
pub mod create_role;
pub mod deny;
pub mod deny_role;
pub mod grant_role;
pub mod remove_role;
pub mod revoke_role;
pub mod set_operator;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

/// Execution context bundling dependencies for execute handlers.
pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
