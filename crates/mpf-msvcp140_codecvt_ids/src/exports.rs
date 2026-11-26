/// The `locale_id` type from `dlls/msvcp140_codecvt_ids/main.c`: one `size_t` id. The port uses
/// `usize`, so the layout follows the 64-bit `size_t` of the x64 build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocaleId {
    pub id: usize,
}

/// One `@ extern` line of `msvcp140_codecvt_ids.spec`: the exported name, the C symbol that
/// `main.c` defines for it, and the id that symbol starts with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocaleIdExport {
    pub name: &'static str,
    pub symbol: &'static str,
    pub initial: LocaleId,
}

/// The four `@ extern` exports, in spec order. Each `main.c` definition starts with `id` = 0.
pub static EXPORTS: [LocaleIdExport; 4] = [
    LocaleIdExport {
        name: "?id@?$codecvt@_SDU_Mbstatet@@@std@@2V0locale@2@A",
        symbol: "codecvt_char16_id",
        initial: LocaleId { id: 0 },
    },
    LocaleIdExport {
        name: "?id@?$codecvt@_S_QU_Mbstatet@@@std@@2V0locale@2@A",
        symbol: "codecvt_char16_char8_id",
        initial: LocaleId { id: 0 },
    },
    LocaleIdExport {
        name: "?id@?$codecvt@_UDU_Mbstatet@@@std@@2V0locale@2@A",
        symbol: "codecvt_char32_id",
        initial: LocaleId { id: 0 },
    },
    LocaleIdExport {
        name: "?id@?$codecvt@_U_QU_Mbstatet@@@std@@2V0locale@2@A",
        symbol: "codecvt_char32_char8_id",
        initial: LocaleId { id: 0 },
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive
/// and must match the whole mangled name.
pub fn export_by_name(name: &str) -> Option<&'static LocaleIdExport> {
    EXPORTS.iter().find(|export| export.name == name)
}
