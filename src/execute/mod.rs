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

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
