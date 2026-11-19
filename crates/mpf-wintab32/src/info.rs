//! The category predicates `context.c` uses to tell `WTInfo` string fields and logical-context
//! categories apart.

const WTI_INTERFACE: u32 = 1;
const IFC_WINTABID: u32 = 1;
const WTI_DEFCONTEXT: u32 = 3;
const WTI_DEFSYSCTX: u32 = 4;
const WTI_DDCTXS: u32 = 400;
const CTX_NAME: u32 = 1;
const WTI_DEVICES: u32 = 100;
const DVC_NAME: u32 = 1;
const DVC_PNPID: u32 = 19;
const WTI_CURSORS: u32 = 200;
const CSR_NAME: u32 = 1;
const CSR_BTNNAMES: u32 = 6;

/// Whether `category` is a logical-context category (`WTI_DEFSYSCTX`, `WTI_DEFCONTEXT`, `WTI_DDCTXS`).
pub fn is_logcontext_category(category: u32) -> bool {
    matches!(category, WTI_DEFCONTEXT | WTI_DEFSYSCTX | WTI_DDCTXS)
}

/// Whether field `index` of `category` holds a string.
pub fn is_string_field(category: u32, index: u32) -> bool {
    (category == WTI_INTERFACE && index == IFC_WINTABID)
        || (is_logcontext_category(category) && index == CTX_NAME)
        || ((WTI_CURSORS..=WTI_CURSORS + 9).contains(&category)
            && (index == CSR_NAME || index == CSR_BTNNAMES))
        || (category == WTI_DEVICES && (index == DVC_NAME || index == DVC_PNPID))
}
