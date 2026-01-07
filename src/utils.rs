//! # Utility Functions
//!
//! Path canonicalization and reference counting utilities.
//!
//! Paths are normalized to ensure consistent storage and matching:
//! - Leading/trailing slashes trimmed, then single leading slash added
//! - Spaces converted to hyphens
//! - Non-printable characters removed

use cosmwasm_std::Storage;

use crate::{error::ContractError, math::sub_u32, state::PATH_REF_COUNTS};

/// Normalizes a raw path string to canonical form.
///
/// # Normalization rules:
/// - Trims leading and trailing slashes
/// - Adds single leading slash
/// - Replaces spaces with hyphens
/// - Removes non-ASCII-graphic characters
///
/// # Examples:
/// - `"//foo/bar//"` becomes `"/foo/bar"`
/// - `"foo bar"` becomes `"/foo-bar"`
pub fn to_canonical_path(raw_path: &String) -> String {
    let mut path = raw_path.clone();
    path = path.trim_matches('/').to_owned();
    path = format!("/{}", path);
    remove_non_printables(&path.replace(" ", "-"))
}

/// Builds a canonical path from path segments (crumbs).
///
/// # Example:
/// - `&["foo", "bar"]` becomes `"/foo/bar"`
pub fn to_canonical_path_from_crumbs(crumbs: &[&str]) -> String {
    format!("/{}", crumbs.join("/"))
}

pub fn remove_non_printables(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_graphic())
        .collect::<String>()
}

/// Decrements the reference count for a path or removes it if count reaches 1.
///
/// Reference counts track how many principals and roles are authorized to a path.
/// When the last authorization is removed, the path is deleted from PATH_REF_COUNTS.
pub fn decrement_or_remove_path_ref_count(
    store: &mut dyn Storage,
    canonical_path: &String,
) -> Result<(), ContractError> {
    if let Some(n) = PATH_REF_COUNTS.may_load(store, &canonical_path)? {
        if n == 1 {
            PATH_REF_COUNTS.remove(store, canonical_path);
        } else {
            PATH_REF_COUNTS.save(store, canonical_path, &sub_u32(n, 1)?)?;
        }
    }
    Ok(())
}
