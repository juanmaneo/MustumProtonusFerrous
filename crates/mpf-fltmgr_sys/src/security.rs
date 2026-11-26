//! `FltBuildDefaultSecurityDescriptor` from Wine's `dlls/fltmgr.sys/main.c`.
//!
//! Wine places the DACL inline after the security descriptor in one pool
//! allocation. Here the DACL is an owned value, and `alloc_len` keeps the byte
//! count Wine requests so the allocation sizes stay observable.

use crate::{NtStatus, STATUS_INSUFFICIENT_RESOURCES, STATUS_NO_MEMORY};

pub const SECURITY_DESCRIPTOR_REVISION: u8 = 1;
pub const SID_REVISION: u8 = 1;
pub const ACL_REVISION: u8 = 2;
pub const SE_DACL_PRESENT: u16 = 0x0004;

/// `sizeof(SECURITY_DESCRIPTOR)` on LP64: Revision, Sbz1, Control and four pointers.
pub const SECURITY_DESCRIPTOR_MIN_LENGTH: u32 = 40;
/// `sizeof(ACL)`.
const ACL_SIZE: u32 = 8;
/// The ACE header and access mask, before the SID.
const ACE_HEADER_AND_MASK_SIZE: u32 = 8;
/// `sizeof(ACCESS_ALLOWED_ACE)`, which includes the first DWORD of the SID.
const ACCESS_ALLOWED_ACE_SIZE: u32 = 12;
/// `sizeof(SID)` with one sub-authority.
const SID_BASE_SIZE: u32 = 12;

pub const SECURITY_NULL_SID_AUTHORITY: [u8; 6] = [0; 6];
pub const SECURITY_BUILTIN_DOMAIN_RID: u32 = 0x0000_0020;
pub const DOMAIN_GROUP_RID_ADMINS: u32 = 0x0000_0200;
pub const SECURITY_LOCAL_SYSTEM_RID: u32 = 0x0000_0012;

/// `RtlLengthRequiredSid`. Uses DWORD wrap-around as Wine does.
pub fn length_required_sid(sub_authority_count: u32) -> u32 {
    sub_authority_count
        .wrapping_sub(1)
        .wrapping_mul(4)
        .wrapping_add(SID_BASE_SIZE)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sid {
    pub revision: u8,
    pub authority: [u8; 6],
    pub sub_authority: Vec<u32>,
}

impl Sid {
    /// `RtlLengthSid`.
    pub fn length(&self) -> u32 {
        length_required_sid(self.sub_authority.len() as u32)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessAllowedAce {
    pub mask: u32,
    pub sid: Sid,
}

impl AccessAllowedAce {
    /// `AceSize`: the header and mask, followed by the SID.
    pub fn size(&self) -> u32 {
        ACE_HEADER_AND_MASK_SIZE + self.sid.length()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Acl {
    pub revision: u8,
    pub size: u16,
    pub aces: Vec<AccessAllowedAce>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityDescriptor {
    pub revision: u8,
    pub control: u16,
    pub dacl: Option<Acl>,
    /// Bytes requested from the pool for the descriptor and its inline DACL.
    pub alloc_len: u32,
}

pub fn build_default_security_descriptor(access: u32) -> Result<SecurityDescriptor, NtStatus> {
    build_default_security_descriptor_with(access, |_| true)
}

/// `alloc_ok` answers the three `ExAllocatePool` calls in order, given each
/// requested size. A `false` answer takes the same exit as a NULL return.
pub fn build_default_security_descriptor_with(
    access: u32,
    mut alloc_ok: impl FnMut(u32) -> bool,
) -> Result<SecurityDescriptor, NtStatus> {
    let admins = Sid {
        revision: SID_REVISION,
        authority: SECURITY_NULL_SID_AUTHORITY,
        sub_authority: vec![SECURITY_BUILTIN_DOMAIN_RID, DOMAIN_GROUP_RID_ADMINS],
    };
    if !alloc_ok(length_required_sid(2)) {
        return Err(STATUS_INSUFFICIENT_RESOURCES);
    }

    let system = Sid {
        revision: SID_REVISION,
        authority: SECURITY_NULL_SID_AUTHORITY,
        sub_authority: vec![SECURITY_LOCAL_SYSTEM_RID],
    };
    if !alloc_ok(length_required_sid(1)) {
        return Err(STATUS_INSUFFICIENT_RESOURCES);
    }

    let alloc_len = SECURITY_DESCRIPTOR_MIN_LENGTH
        + ACL_SIZE
        + ACCESS_ALLOWED_ACE_SIZE
        + admins.length()
        + ACCESS_ALLOWED_ACE_SIZE
        + system.length();
    if !alloc_ok(alloc_len) {
        return Err(STATUS_NO_MEMORY);
    }

    let dacl = Acl {
        revision: ACL_REVISION,
        size: (alloc_len - SECURITY_DESCRIPTOR_MIN_LENGTH) as u16,
        aces: vec![
            AccessAllowedAce {
                mask: access,
                sid: admins,
            },
            AccessAllowedAce {
                mask: access,
                sid: system,
            },
        ],
    };
    Ok(SecurityDescriptor {
        revision: SECURITY_DESCRIPTOR_REVISION,
        control: SE_DACL_PRESENT,
        dacl: Some(dacl),
        alloc_len,
    })
}
