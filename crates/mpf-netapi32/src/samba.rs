//! The security descriptor, ACE and access mask mapping from Wine's `unixlib.c`, from the Wine
//! values to the Samba values.
//!
//! The Samba values match the Wine values for each bit they keep. A bit that Samba has no name for
//! is dropped, and an unknown bit makes the ACE flags or the access mask 0 as a whole.

/// The control bits that Samba names. `SE_SELF_RELATIVE` (0x8000) is not among them.
const SD_CONTROL_KNOWN: u16 = 0x7f3f;

/// The ACE flag bits that Samba names. An ACE flag outside this set makes the result 0.
const ACE_FLAGS_KNOWN: u8 = 0xdf;

/// Wine ACE flag bits and the Samba bit each one maps to.
const ACE_FLAG_MAP: [(u8, u8); 7] = [
    (0x01, 0x01),
    (0x02, 0x04),
    (0x04, 0x04),
    (0x08, 0x08),
    (0x10, 0x10),
    (0x40, 0x40),
    (0x80, 0x80),
];

/// The generic rights bits, which Samba names with the same values.
const GENERIC_RIGHTS_KNOWN: u32 = 0xf000_0000;

/// `sd_control_to_samba`: keeps the control bits that Samba names.
pub fn sd_control_to_samba(control: u16) -> u16 {
    control & SD_CONTROL_KNOWN
}

/// `ace_type_to_samba`: the access types 0 to 3 keep their value. Any other type gives 0.
pub fn ace_type_to_samba(ace_type: u8) -> u8 {
    match ace_type {
        0..=3 => ace_type,
        _ => 0,
    }
}

/// `ace_flags_to_samba`: maps the ACE flag bits. Wine's `CONTAINER_INHERIT` maps to Samba's
/// `NO_PROPAGATE_INHERIT`, as in the C code. Flags outside the known set give 0.
pub fn ace_flags_to_samba(flags: u8) -> u8 {
    if flags & !ACE_FLAGS_KNOWN != 0 {
        return 0;
    }
    ACE_FLAG_MAP
        .iter()
        .filter(|&&(wine, _)| flags & wine != 0)
        .fold(0, |ret, &(_, samba)| ret | samba)
}

/// `access_mask_to_samba`: keeps the generic rights bits. A mask with any other bit gives 0.
pub fn access_mask_to_samba(mask: u32) -> u32 {
    if mask & !GENERIC_RIGHTS_KNOWN != 0 {
        return 0;
    }
    mask
}
