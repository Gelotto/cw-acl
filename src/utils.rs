use cosmwasm_std::Storage;

use crate::{error::ContractError, math::sub_u32, state::PATH_REF_COUNTS};

pub fn to_cannonical_path(raw_path: &String) -> String {
    let mut path = raw_path.clone();
    path = path.trim_matches('/').to_owned();
    path = format!("/{}", path);
    remove_non_printables(&path.replace(" ", "-"))
}

pub fn to_cannonical_path_from_crumbs(crumbs: &[&str]) -> String {
    format!("/{}", crumbs.join("/"))
}

pub fn remove_non_printables(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .collect::<String>()
}

/// Remove path from global path lookup table or decrement its ref count
pub fn decrement_or_remove_path_ref_count(
    store: &mut dyn Storage,
    cannonical_path: &String,
) -> Result<(), ContractError> {
    // Remove path from global path lookup table or decrement its ref count
    if let Some(n) = PATH_REF_COUNTS.may_load(store, &cannonical_path)? {
        if n == 1 {
            PATH_REF_COUNTS.remove(store, cannonical_path);
        } else {
            PATH_REF_COUNTS.save(store, cannonical_path, &sub_u32(n, 1)?)?;
        }
    }
    Ok(())
}
