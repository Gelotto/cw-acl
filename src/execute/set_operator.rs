//! # Set Operator
//!
//! Transfers control of the ACL to a new operator.

use crate::{client::Operator, error::ContractError, state::OP};
use cosmwasm_std::{attr, Response};

use super::Context;

/// Changes the ACL operator to a new address or ACL contract.
pub fn exec_set_operator(
    ctx: Context,
    new_operator: Operator,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let old_operator = OP.load(deps.storage)?;
    OP.save(deps.storage, &new_operator)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "set_operator"),
        attr("old_operator", old_operator.to_string()),
        attr("new_operator", new_operator.to_string()),
    ]))
}
