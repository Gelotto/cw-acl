use crate::client::ensure_is_allowed;
use crate::error::ContractError;
use crate::execute::allow::exec_allow;
use crate::execute::allow_role::exec_allow_role;
use crate::execute::create_role::exec_create_role;
use crate::execute::deny::exec_deny;
use crate::execute::deny_role::exec_deny_role;
use crate::execute::grant_role::exec_grant_role;
use crate::execute::revoke_role::exec_revoke_role;
use crate::execute::{set_operator::exec_set_operator, Context};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, RoleExecuteMsg};
use crate::query::acl::query_acl;
use crate::query::is_allowed::query_is_allowed as query_allowed;
use crate::query::paths::query_paths;
use crate::query::role::query_role;
use crate::query::roles::query_roles;
use crate::query::ReadonlyContext;
use crate::state::{self, OP};
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-acl";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(state::init(Context { deps, env, info }, msg)?)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Only allow sender to make changes to ACL if operator. Note that the
    // operator may be either an arbitrary address or an address of another ACL.
    ensure_is_allowed(deps.querier, &info.sender, OP.load(deps.storage)?, || {
        format!("/acls/{}", env.contract.address)
    })?;

    let ctx = Context { deps, env, info };

    match msg {
        ExecuteMsg::SetOperator(operator) => exec_set_operator(ctx, operator),
        ExecuteMsg::Allow(msg) => exec_allow(ctx, msg),
        ExecuteMsg::Deny(msg) => exec_deny(ctx, msg),
        ExecuteMsg::Role(msg) => match msg {
            RoleExecuteMsg::Create(msg) => exec_create_role(ctx, msg),
            RoleExecuteMsg::Allow(msg) => exec_allow_role(ctx, msg),
            RoleExecuteMsg::Deny(msg) => exec_deny_role(ctx, msg),
            RoleExecuteMsg::Grant(msg) => exec_grant_role(ctx, msg),
            RoleExecuteMsg::Revoke(msg) => exec_revoke_role(ctx, msg),
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Acl {} => to_json_binary(&query_acl(ctx)?),
        QueryMsg::Roles { principal } => to_json_binary(&query_roles(ctx, principal)?),
        QueryMsg::Role(role) => to_json_binary(&query_role(ctx, role)?),
        QueryMsg::Paths(params) => to_json_binary(&query_paths(ctx, params)?),
        QueryMsg::IsAllowed(msg) => to_json_binary(&query_allowed(ctx, msg)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
