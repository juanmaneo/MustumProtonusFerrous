//! `DsRoleGetPrimaryDomainInformation` logic that does not touch the LSA or the heap.

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_OUTOFMEMORY: u32 = 14;
pub const ERROR_INVALID_PARAMETER: u32 = 87;
pub const ERROR_CALL_NOT_IMPLEMENTED: u32 = 120;

/// `DSROLE_PRIMARY_DOMAIN_INFO_LEVEL` values from `dsrole.h`.
pub const PRIMARY_DOMAIN_INFO_BASIC: i32 = 1;
pub const PRIMARY_DOMAIN_INFO_OPERATION_STATE: i32 = 3;

/// `DsRole_RoleStandaloneWorkstation`, the only role the Wine code reports.
pub const ROLE_STANDALONE_WORKSTATION: u32 = 0;

/// Code path taken after the argument checks pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfoRoute {
    Basic,
    NotImplemented,
}

/// Argument checks, in the order Wine applies them.
pub fn route(has_buffer: bool, info_level: i32) -> Result<InfoRoute, u32> {
    if !has_buffer
        || !(PRIMARY_DOMAIN_INFO_BASIC..=PRIMARY_DOMAIN_INFO_OPERATION_STATE).contains(&info_level)
    {
        return Err(ERROR_INVALID_PARAMETER);
    }
    if info_level == PRIMARY_DOMAIN_INFO_BASIC {
        Ok(InfoRoute::Basic)
    } else {
        Ok(InfoRoute::NotImplemented)
    }
}

/// `sizeof(DSROLE_PRIMARY_DOMAIN_INFO_BASIC)`: `MachineRole` and `Flags` (8 bytes),
/// three `LPWSTR` pointers, and a 16-byte `GUID`.
pub fn basic_struct_size(pointer_size: usize) -> usize {
    8 + 3 * pointer_size + 16
}

/// Length up to the first NUL, or the slice length when there is none.
pub fn wide_strlen(s: &[u16]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

/// `lstrcpyW` of the domain name: the characters up to the first NUL, plus the terminator.
pub fn copy_domain_name(domain: &[u16]) -> Vec<u16> {
    let mut out = domain[..wide_strlen(domain)].to_vec();
    out.push(0);
    out
}

/// Size Wine passes to `HeapAlloc`: the fixed struct plus `(strlen + 1)` WCHARs for the flat name.
pub fn basic_alloc_size(pointer_size: usize, domain: &[u16]) -> usize {
    basic_struct_size(pointer_size) + (wide_strlen(domain) + 1) * 2
}

/// Caller inputs plus the results of `LsaOpenPolicy` and `HeapAlloc`.
#[derive(Debug, Clone, Copy)]
pub struct Query<'a> {
    pub has_buffer: bool,
    pub info_level: i32,
    pub policy_open: bool,
    pub alloc_ok: bool,
    pub pointer_size: usize,
    /// `DomainInfo->DomainName.Buffer` as returned by the LSA, NUL-terminated.
    pub domain_name: &'a [u16],
}

/// The basic-level buffer that Wine fills in on success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimaryDomainBasic {
    pub machine_role: u32,
    pub domain_name_flat: Vec<u16>,
    pub alloc_size: usize,
}

/// Return value and `*Buffer` after the call. `buffer` is `None` when Wine leaves `*Buffer` NULL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub ret: u32,
    pub buffer: Option<PrimaryDomainBasic>,
}

pub fn get_primary_domain_information(q: &Query) -> Reply {
    let fail = |ret| Reply { ret, buffer: None };
    match route(q.has_buffer, q.info_level) {
        Err(ret) => fail(ret),
        Ok(InfoRoute::NotImplemented) => fail(ERROR_CALL_NOT_IMPLEMENTED),
        Ok(InfoRoute::Basic) if !q.policy_open || !q.alloc_ok => fail(ERROR_OUTOFMEMORY),
        Ok(InfoRoute::Basic) => Reply {
            ret: ERROR_SUCCESS,
            buffer: Some(PrimaryDomainBasic {
                machine_role: ROLE_STANDALONE_WORKSTATION,
                domain_name_flat: copy_domain_name(q.domain_name),
                alloc_size: basic_alloc_size(q.pointer_size, q.domain_name),
            }),
        },
    }
}
