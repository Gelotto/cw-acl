use cosmwasm_std::{attr, Addr, Response, Timestamp};
use cw_storage_plus::{Item, Map};

use crate::{
    client::Operator,
    error::ContractError,
    execute::Context,
    models::{AuthRecord, AuthRoleInfo},
    msg::InstantiateMsg,
};

type Role = String;
type Principal = String;
type Path = String;

pub const MAX_NAME_LEN: usize = 100;
pub const MAX_DESC_LEN: usize = 1000;

pub const OP: Item<Operator> = Item::new("op");
pub const CREATED_BY: Item<Addr> = Item::new("created_by");
pub const CREATED_AT: Item<Timestamp> = Item::new("created_at");
pub const NAME: Item<String> = Item::new("name");
pub const DESCRIPTION: Item<String> = Item::new("desc");

pub const PATH_REF_COUNTS: Map<&Path, u32> = Map::new("prc");
pub const PRINCIPAL_PATH_AUTHORIZATIONS: Map<(&Principal, &Path), AuthRecord> = Map::new("ppa");
pub const PRINCIPAL_ROLE_AUTHORIZATIONS: Map<(&Principal, &Role), AuthRecord> = Map::new("pra");

pub const ROLE_INFOS: Map<&Role, AuthRoleInfo> = Map::new("ri");
pub const ROLE_PATHS: Map<(&Role, &Path), u8> = Map::new("rp");
pub const PATH_ROLES: Map<(&Path, &Role), u8> = Map::new("pr");

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, env, info } = ctx;
    let InstantiateMsg {
        operator,
        name,
        description,
    } = msg;

    // Validate operator
    let operator = if let Some(op) = &operator {
        deps.api.addr_validate(
            match op {
                Operator::Address(addr) => addr,
                Operator::Acl(addr) => addr,
            }
            .as_str(),
        )?;
        op.to_owned()
    } else {
        Operator::Address(info.sender.clone())
    };

    // Set ACL name
    if let Some(name) = &name {
        if name.len() > MAX_NAME_LEN {
            return Err(ContractError::ValidationError {
                reason: format!("ACL name cannot be longer than {} characters", MAX_NAME_LEN),
            });
        }
        NAME.save(deps.storage, name)?;
    }

    // Set ACL description
    if let Some(desc) = &description {
        if desc.len() > MAX_DESC_LEN {
            return Err(ContractError::ValidationError {
                reason: format!(
                    "ACL description cannot be longer than {} characters",
                    MAX_DESC_LEN
                ),
            });
        }
        DESCRIPTION.save(deps.storage, desc)?;
    }

    OP.save(deps.storage, &operator)?;
    CREATED_AT.save(deps.storage, &env.block.time)?;
    CREATED_BY.save(deps.storage, &info.sender)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("acl_operator", operator.to_string()),
        attr("acl_name", name.unwrap_or_default()),
    ]))
}
