use crate::nt::{set_ntstatus, Handle, Nt, INVALID_HANDLE_VALUE};
use crate::timeout::get_nt_timeout;
use crate::unicode::{init_unicode_string, UnicodeString};

/// `OBJ_INHERIT` from `include/winternl.h`.
pub const OBJ_INHERIT: u32 = 0x0000_0002;

/// `OBJ_CASE_INSENSITIVE` from `include/winternl.h`.
pub const OBJ_CASE_INSENSITIVE: u32 = 0x0000_0040;

/// The `SECURITY_ATTRIBUTES` fields that `CreateTransaction` reads. `security_descriptor` is the
/// opaque `lpSecurityDescriptor` address, with `None` for NULL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecurityAttributes {
    pub inherit_handle: bool,
    pub security_descriptor: Option<usize>,
}

/// A `GUID` that `CreateTransaction` passes on as the `Uow` argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// The `OBJECT_ATTRIBUTES` that `CreateTransaction` builds with `InitializeObjectAttributes`. The
/// root directory and the object name are NULL, as is the quality of service.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectAttributes {
    pub attributes: u32,
    pub security_descriptor: Option<usize>,
}

/// The arguments of the `NtCreateTransaction` call that `CreateTransaction` makes. `DesiredAccess`
/// is always 0 (`/* FIXME */` in the C source), and `TmHandle` is always NULL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NtCreateTransactionArgs<'a> {
    pub object_attributes: ObjectAttributes,
    pub uow: Option<Guid>,
    pub options: u32,
    pub level: u32,
    pub flags: u32,
    pub timeout: Option<i64>,
    pub description: UnicodeString<'a>,
}

/// `CommitTransaction` from `dlls/ktmw32/ktmw32_main.c`.
pub fn commit_transaction(nt: &mut impl Nt, transaction: Handle) -> bool {
    let status = nt.nt_commit_transaction(transaction, true);
    set_ntstatus(nt, status)
}

/// `RollbackTransaction` from `dlls/ktmw32/ktmw32_main.c`.
pub fn rollback_transaction(nt: &mut impl Nt, transaction: Handle) -> bool {
    let status = nt.nt_rollback_transaction(transaction, true);
    set_ntstatus(nt, status)
}

/// `CreateTransaction` from `dlls/ktmw32/ktmw32_main.c`. Returns `INVALID_HANDLE_VALUE` when the
/// NT call fails, whatever handle the call wrote.
#[allow(clippy::too_many_arguments)]
pub fn create_transaction(
    nt: &mut impl Nt,
    sa: Option<&SecurityAttributes>,
    guid: Option<Guid>,
    options: u32,
    level: u32,
    flags: u32,
    timeout: u32,
    desc: Option<&[u16]>,
) -> Handle {
    let mut attributes = OBJ_CASE_INSENSITIVE;
    if sa.is_some_and(|sa| sa.inherit_handle) {
        attributes |= OBJ_INHERIT;
    }
    let args = NtCreateTransactionArgs {
        object_attributes: ObjectAttributes {
            attributes,
            security_descriptor: sa.and_then(|sa| sa.security_descriptor),
        },
        uow: guid,
        options,
        level,
        flags,
        timeout: get_nt_timeout(timeout),
        description: init_unicode_string(desc),
    };
    let (status, handle) = nt.nt_create_transaction(&args);
    if set_ntstatus(nt, status) {
        handle
    } else {
        INVALID_HANDLE_VALUE
    }
}
