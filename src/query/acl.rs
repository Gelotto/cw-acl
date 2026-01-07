//! # ACL Metadata Query
//!
//! Returns ACL configuration and metadata.

use crate::{
    error::ContractError,
    models::Config,
    responses::AclResponse,
    state::{CREATED_AT, CREATED_BY, DESCRIPTION, NAME, OP},
};

use super::ReadonlyContext;

pub fn query_acl(ctx: ReadonlyContext) -> Result<AclResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(AclResponse {
        operator: OP.load(deps.storage)?,
        created_by: CREATED_BY.load(deps.storage)?,
        created_at: CREATED_AT.load(deps.storage)?,
        name: NAME.may_load(deps.storage)?,
        description: DESCRIPTION.may_load(deps.storage)?,
        config: Config {},
    })
}
