//! The `ImmGetProperty` switch, `ime_is_unicode` and `EscapeRequiresWA` in `dlls/imm32/imm.c`.

pub const IGP_GETIMEVERSION: u32 = 0xffff_fffc;
pub const IGP_PROPERTY: u32 = 0x4;
pub const IGP_CONVERSION: u32 = 0x8;
pub const IGP_SENTENCE: u32 = 0xc;
pub const IGP_UI: u32 = 0x10;
pub const IGP_SETCOMPSTR: u32 = 0x14;
pub const IGP_SELECT: u32 = 0x18;

pub const IMEVER_0400: u32 = 0x0004_0000;
pub const IME_PROP_UNICODE: u32 = 0x0008_0000;

pub const IME_ESC_SEQUENCE_TO_INTERNAL: u32 = 0x1001;
pub const IME_ESC_GET_EUDC_DICTIONARY: u32 = 0x1003;
pub const IME_ESC_SET_EUDC_DICTIONARY: u32 = 0x1004;
pub const IME_ESC_MAX_KEY: u32 = 0x1005;
pub const IME_ESC_IME_NAME: u32 = 0x1006;
pub const IME_ESC_HANJA_MODE: u32 = 0x1008;
pub const IME_ESC_GETHELPFILENAME: u32 = 0x100b;

/// The fields of `IMEINFO` that `ImmGetProperty` reads.
pub struct ImeInfo {
    pub property: u32,
    pub conversion_caps: u32,
    pub sentence_caps: u32,
    pub scs_caps: u32,
    pub select_caps: u32,
}

/// `ImmGetProperty`. `IGP_UI` and unknown indices give 0.
pub fn get_property(info: &ImeInfo, index: u32) -> u32 {
    match index {
        IGP_PROPERTY => info.property,
        IGP_CONVERSION => info.conversion_caps,
        IGP_SENTENCE => info.sentence_caps,
        IGP_SETCOMPSTR => info.scs_caps,
        IGP_SELECT => info.select_caps,
        IGP_GETIMEVERSION => IMEVER_0400,
        _ => 0,
    }
}

/// `ime_is_unicode`: whether the IME property word has `IME_PROP_UNICODE`.
pub fn is_unicode(property: u32) -> bool {
    (property & IME_PROP_UNICODE) != 0
}

/// `EscapeRequiresWA`: whether an `ImmEscape` function takes ANSI strings that need conversion.
pub const fn escape_requires_wa(escape: u32) -> bool {
    matches!(
        escape,
        IME_ESC_GET_EUDC_DICTIONARY
            | IME_ESC_SET_EUDC_DICTIONARY
            | IME_ESC_IME_NAME
            | IME_ESC_GETHELPFILENAME
    )
}
