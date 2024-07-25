pub mod acl;
pub mod is_allowed;
pub mod paths;
pub mod role;
pub mod roles;

use cosmwasm_std::{Deps, Env};

pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
