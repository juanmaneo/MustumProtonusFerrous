//! Byte classes of the Japanese codec in `dlls/mlang/mlang.c`, one function per macro.

/// `HANKATA`: a half-width katakana lead byte.
pub(crate) fn is_hankata(byte: u8) -> bool {
    (161..=223).contains(&byte)
}

/// `ISEUC`: a byte in the EUC-JP range.
pub(crate) fn is_euc(byte: u8) -> bool {
    (161..=254).contains(&byte)
}

/// `NOTEUC`: a lead and trail pair that only Shift-JIS can hold.
pub(crate) fn is_not_euc_pair(lead: u8, trail: u8) -> bool {
    (129..=159).contains(&lead) && (64..=160).contains(&trail)
}

/// `SJIS1`: a Shift-JIS lead byte.
pub(crate) fn is_sjis_lead(byte: u8) -> bool {
    (129..=159).contains(&byte) || (224..=239).contains(&byte)
}

/// `SJIS2`: a Shift-JIS trail byte.
pub(crate) fn is_sjis_trail(byte: u8) -> bool {
    (64..=252).contains(&byte)
}

/// `ISMARU`: a katakana that takes the semi-voiced mark.
pub(crate) fn is_maru(byte: u8) -> bool {
    (202..=206).contains(&byte)
}

/// `ISNIGORI`: a katakana that takes the voiced mark.
pub(crate) fn is_nigori(byte: u8) -> bool {
    (182..=196).contains(&byte) || (202..=206).contains(&byte)
}
