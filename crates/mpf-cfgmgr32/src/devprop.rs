//! `DEVPROPTYPE` validation of `devobject.c`.

pub const DEVPROP_TYPE_EMPTY: u32 = 0x00;
pub const DEVPROP_TYPE_NULL: u32 = 0x01;
pub const DEVPROP_TYPE_GUID: u32 = 0x0d;
pub const DEVPROP_TYPE_STRING: u32 = 0x12;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR: u32 = 0x13;
pub const DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING: u32 = 0x14;
pub const DEVPROP_TYPE_NTSTATUS: u32 = 0x18;
pub const DEVPROP_TYPE_STRING_INDIRECT: u32 = 0x19;
pub const DEVPROP_TYPEMOD_ARRAY: u32 = 0x1000;
pub const DEVPROP_TYPEMOD_LIST: u32 = 0x2000;
pub const DEVPROP_MASK_TYPE: u32 = 0x0fff;
pub const DEVPROP_MASK_TYPEMOD: u32 = 0xf000;

/// The byte size of each fixed-size type, indexed by base type from 0 to
/// `DEVPROP_TYPE_NTSTATUS`. Entries for the variable-size types are 0 and are never
/// read.
const TYPE_SIZE: [usize; 25] = [
    0, 0, 1, 1, 2, 2, 4, 4, 8, 8, 4, 8, 16, 16, 8, 8, 8, 1, 0, 0, 0, 20, 4, 4, 4,
];

/// `devprop_type_validate`: whether `ty` is a valid type for a value of `size` bytes.
///
/// Bits above the 16 type bits are ignored, as the C masks do.
pub fn type_validate(ty: u32, size: usize) -> bool {
    let modifier = ty & DEVPROP_MASK_TYPEMOD;
    if modifier != 0 && modifier != DEVPROP_TYPEMOD_ARRAY && modifier != DEVPROP_TYPEMOD_LIST {
        return false;
    }
    let base = ty & DEVPROP_MASK_TYPE;
    match base {
        DEVPROP_TYPE_EMPTY | DEVPROP_TYPE_NULL => modifier == 0,
        DEVPROP_TYPE_SECURITY_DESCRIPTOR | DEVPROP_TYPE_STRING_INDIRECT => {
            modifier == 0 && size != 0
        }
        DEVPROP_TYPE_STRING | DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING => {
            modifier != DEVPROP_TYPEMOD_ARRAY && size != 0
        }
        _ if base > DEVPROP_TYPE_NTSTATUS
            || (modifier != 0 && modifier != DEVPROP_TYPEMOD_ARRAY) =>
        {
            false
        }
        _ => {
            let fixed = TYPE_SIZE[base as usize];
            if modifier == DEVPROP_TYPEMOD_ARRAY {
                size >= fixed
            } else {
                size == fixed
            }
        }
    }
}
