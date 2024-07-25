use std::marker::PhantomData;

use cosmwasm_std::Order;
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    models::AuthRecord,
    msg::{PathsQueryParams, Subject},
    responses::{PathInfo, PathsResponse},
    state::{PATH_REF_COUNTS, PRINCIPAL_PATH_AUTHORIZATIONS, ROLE_PATHS},
};

use super::ReadonlyContext;

const MAX_LIMIT: u16 = 500;
const DEFAULT_LIMIT: u16 = 100;

pub fn query_paths(
    ctx: ReadonlyContext,
    params: PathsQueryParams,
) -> Result<PathsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let PathsQueryParams {
        subject,
        limit,
        cursor,
        start,
        stop,
    } = params;

    let limit = limit.unwrap_or(DEFAULT_LIMIT).clamp(0, MAX_LIMIT) as usize;
    let mut path_infos: Vec<PathInfo> = Vec::with_capacity(8);
    let mut min_bound_path_box: Box<String> = Box::new("".to_owned());
    let mut max_bound_path_box: Box<String> = Box::new("".to_owned());

    let min_bound = match cursor {
        Some(cursor_path) => {
            *min_bound_path_box = cursor_path;
            Some(Bound::Exclusive((min_bound_path_box.as_ref(), PhantomData)))
        },
        None => start.and_then(|path| {
            *min_bound_path_box = path;
            Some(Bound::Inclusive((min_bound_path_box.as_ref(), PhantomData)))
        }),
    };

    let max_bound = stop.and_then(|path| {
        *max_bound_path_box = path;
        Some(Bound::Inclusive((max_bound_path_box.as_ref(), PhantomData)))
    });

    match subject {
        Subject::Acl => {
            for result in PATH_REF_COUNTS
                .keys(deps.storage, min_bound, max_bound, Order::Ascending)
                .take(limit)
            {
                let path = result?;
                path_infos.push(PathInfo {
                    path,
                    expires_at: None,
                })
            }
        },
        Subject::Role(role) => {
            for result in ROLE_PATHS
                .prefix(&role)
                .keys(deps.storage, min_bound, max_bound, Order::Ascending)
                .take(limit)
            {
                let path = result?;
                path_infos.push(PathInfo {
                    path,
                    expires_at: None,
                })
            }
        },
        Subject::Principal(principal) => {
            for result in PRINCIPAL_PATH_AUTHORIZATIONS
                .prefix(&principal)
                .range(deps.storage, min_bound, max_bound, Order::Ascending)
                .take(limit)
            {
                let (path, AuthRecord { expires_at }) = result?;
                path_infos.push(PathInfo { path, expires_at })
            }
        },
    }

    let next_cursor = if path_infos.len() == limit {
        path_infos
            .last()
            .and_then(|info| Some(info.path.to_owned()))
    } else {
        None
    };

    Ok(PathsResponse {
        paths: path_infos,
        cursor: next_cursor,
    })
}
