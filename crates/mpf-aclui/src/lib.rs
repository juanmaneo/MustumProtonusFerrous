//! Port of the portable logic in Wine's `dlls/aclui/aclui_main.c`.
//!
//! Ported:
//! - `get_sid_from_ace`: [`sid_from_ace`]
//! - `compute_access_masks`: [`compute_access_masks`], returning [`AccessMasks`]
//! - The row logic of `update_access_list`, with its `SI_ACCESS_GENERAL` filter and the
//!   `(mask & bits) == mask` test behind each X/- cell: [`access_rows`]
//!
//! Skipped:
//! - `GetSecurityDescriptorDacl`, `GetAce`, `EqualSid`: OS security calls. The port takes the
//!   DACL as a slice and compares SIDs through `PartialEq`. The `GetAce` failure `break` is
//!   not modelled because a slice cannot fail.
//! - `load_formatstr`, `get_sid_name` (`LookupAccountSidW`), `add_user`, `init_users`:
//!   resource text, account lookup and listview insertion.
//! - `init_access_list`, `create_image_list`, `security_page_*`, `CreateSecurityPage`,
//!   `EditSecurity`, `DllMain`: dialog, image list, property sheet and module plumbing.
//! - The `FIXME` and `ERR` traces.
//!
//! Deviation: Wine dereferences a DACL that is present but NULL (a null DACL grants everyone
//! access). The port maps any absent DACL to `None`, which yields zero masks instead of a crash.

/// `ACCESS_ALLOWED_ACE_TYPE` from `include/winnt.h`.
pub const ACCESS_ALLOWED_ACE_TYPE: u8 = 0x0;
/// `ACCESS_DENIED_ACE_TYPE` from `include/winnt.h`.
pub const ACCESS_DENIED_ACE_TYPE: u8 = 0x1;
/// `SI_ACCESS_GENERAL` from `include/aclui.h`.
pub const SI_ACCESS_GENERAL: u32 = 0x0002_0000;

/// An ACE, reduced to the fields the C code reads. `sid` is the operand of `EqualSid`, so
/// SID equality is whatever `S: PartialEq` defines.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ace<S> {
    pub ace_type: u8,
    pub sid: S,
    pub mask: u32,
}

/// The `allowed` and `denied` out-parameters of `compute_access_masks`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AccessMasks {
    pub allowed: u32,
    pub denied: u32,
}

/// One `SI_ACCESS` entry. The name is a UI resource and is not modelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessEntry {
    pub mask: u32,
    pub flags: u32,
}

/// One access-list row. `true` shows "X" and `false` shows "-".
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AccessRow {
    pub allowed: bool,
    pub denied: bool,
}

/// `get_sid_from_ace`: the SID of an allowed or denied ACE, `None` for any other type.
pub fn sid_from_ace<S>(ace: &Ace<S>) -> Option<&S> {
    match ace.ace_type {
        ACCESS_ALLOWED_ACE_TYPE | ACCESS_DENIED_ACE_TYPE => Some(&ace.sid),
        _ => None,
    }
}

/// `compute_access_masks`: ORs the masks of the allowed and denied ACEs that name `sid`.
/// `dacl` is `None` when the descriptor has no DACL or the query failed; both give zero masks.
pub fn compute_access_masks<S: PartialEq>(dacl: Option<&[Ace<S>]>, sid: &S) -> AccessMasks {
    let mut masks = AccessMasks::default();
    for ace in dacl.unwrap_or(&[]) {
        let Some(ace_sid) = sid_from_ace(ace) else {
            continue;
        };
        if ace_sid != sid {
            continue;
        }
        if ace.ace_type == ACCESS_ALLOWED_ACE_TYPE {
            masks.allowed |= ace.mask;
        } else {
            masks.denied |= ace.mask;
        }
    }
    masks
}

/// The row logic of `update_access_list`: one row per `SI_ACCESS_GENERAL` entry, in table
/// order. A cell is "X" when every bit of the entry's mask is set in the column's mask.
pub fn access_rows(access: &[AccessEntry], masks: AccessMasks) -> Vec<AccessRow> {
    access
        .iter()
        .filter(|entry| (entry.flags & SI_ACCESS_GENERAL) != 0)
        .map(|entry| AccessRow {
            allowed: (entry.mask & masks.allowed) == entry.mask,
            denied: (entry.mask & masks.denied) == entry.mask,
        })
        .collect()
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
