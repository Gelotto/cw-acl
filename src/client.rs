//! # Operator Authorization
//!
//! Defines the Operator type and authorization checking logic.
//!
//! An operator can be:
//! - `Address`: Direct address check (sender must match)
//! - `Acl`: Delegated to another ACL contract (queries that ACL)

use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ensure_eq, Addr, Empty, QuerierWrapper, StdError, StdResult};

use crate::msg::{IsAllowedParams, QueryMsg, TestRequirement};

/// The entity authorized to modify an ACL.
///
/// Can be either a direct address or another ACL contract for hierarchical control.
#[cw_serde]
pub enum Operator {
    /// Direct address control
    Address(Addr),
    /// Delegated control via another ACL contract
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

/// Ensures the sender is authorized according to the operator configuration.
///
/// # Operator Types:
/// - `Address(addr)`: Sender must exactly match the address
/// - `Acl(acl_addr)`: Queries the ACL contract to check if sender is authorized
///   to the given path
///
/// # Parameters:
/// - `path`: Lazy closure that constructs the path to check. Only evaluated
///   for ACL operators to avoid unnecessary string allocation.
///
/// # Returns:
/// - `Ok(())` if authorized
/// - `Err` with descriptive message if not authorized
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
