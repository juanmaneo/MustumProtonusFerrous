/// `NDDE_NOT_IMPLEMENTED` from `dlls/nddeapi/nddeapi.c`. Network DDE was removed in Windows
/// Vista, so the C file replicates this value because the SDK header no longer has it.
pub const NDDE_NOT_IMPLEMENTED: u32 = 14;

/// `NDdeShareAddA`, exported as `stdcall` in `dlls/nddeapi/nddeapi.spec`. The C body only logs
/// its arguments and returns [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_add_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareAddW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_add_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareDelA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_del_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareDelW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_del_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareEnumA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_enum_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareEnumW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_enum_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareGetInfoA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_get_info_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareGetInfoW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_get_info_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareSetInfoA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_set_info_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeShareSetInfoW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_share_set_info_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetErrorStringA`, exported as `stdcall`. The C body only logs its arguments, never
/// writes the buffer, and returns [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_error_string_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetErrorStringW`, exported as `stdcall`. The C body only traces a `FIXME`, never writes
/// the buffer, and returns [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_error_string_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetShareSecurityA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_share_security_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetShareSecurityW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_share_security_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeSetShareSecurityA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_set_share_security_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeSetShareSecurityW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_set_share_security_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetTrustedShareA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_trusted_share_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeGetTrustedShareW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_get_trusted_share_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeSetTrustedShareA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_set_trusted_share_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeSetTrustedShareW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_set_trusted_share_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeTrustedShareEnumA`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_trusted_share_enum_a() -> u32 {
    NDDE_NOT_IMPLEMENTED
}

/// `NDdeTrustedShareEnumW`, exported as `stdcall`. The C body only logs its arguments and returns
/// [`NDDE_NOT_IMPLEMENTED`].
pub fn ndde_trusted_share_enum_w() -> u32 {
    NDDE_NOT_IMPLEMENTED
}
