use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure_eq, Addr, Empty, QuerierWrapper, StdError, StdResult};

use crate::msg::{IsAllowedParams, QueryMsg, TestRequirement};

#[cw_serde]
pub enum Operator {
    Address(Addr),
    Acl(Addr),
}

impl fmt::Display for Operator {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Address(addr) => write!(f, "{{\"address\": \"{}\"}}", addr.to_string()),
            Self::Acl(addr) => write!(f, "{{\"acl\": \"{}\"}}", addr.to_string()),
        }
    }
}

pub fn ensure_is_allowed<F>(
    querier: QuerierWrapper<Empty>,
    sender: &Addr,
    operator: Operator,
    path: F,
) -> StdResult<()>
where
    F: Fn() -> String,
{
    match operator {
        Operator::Address(operator_addr) => {
            ensure_eq!(
                *sender,
                operator_addr,
                StdError::generic_err(format!("{} is not the contract operator", sender))
            )
        },
        Operator::Acl(acl_addr) => {
            querier.query_wasm_smart(
                acl_addr,
                &QueryMsg::IsAllowed(IsAllowedParams {
                    paths: vec![path()],
                    principal: sender.to_string(),
                    raise: Some(true),
                    require: Some(TestRequirement::All),
                }),
            )?;
        },
    }

    Ok(())
}
