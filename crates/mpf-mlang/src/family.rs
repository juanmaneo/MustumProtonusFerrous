//! Codepage families of `mlang_data` in `dlls/mlang/mlang.c`: the codepages that each family's
//! `MIME_CP_INFO` list names. The MIME descriptions, charsets and flags are not ported.
//! The KOI8-R entry 878 sits under `#if 0` in `mlang.c`, so it is not listed.

use crate::CP_UNICODE;

/// Each family codepage with the codepages of its group, in the order of `mlang_data`.
const FAMILIES: &[(u32, &[u32])] = &[
    (1256, &[864, 1006, 1256, 28596]),
    (1257, &[775, 1257, 28594, 28603]),
    (936, &[50936, 936, 20936, 52936, 54936, 936]),
    (950, &[50950, 950, 20000]),
    (1250, &[852, 1250, 10029, 28592]),
    (1251, &[855, 866, 1251, 10007, 20866, 21866, 28595]),
    (1253, &[737, 869, 875, 1253, 10006, 28597]),
    (1255, &[424, 856, 862, 1255, 28598]),
    (932, &[50932, 51932, 50220, 20932, 50221, 50222, 10001, 932]),
    (949, &[949]),
    (874, &[874]),
    (1254, &[857, 1026, 1254, 10081, 28593, 28599]),
    (1258, &[1258]),
    (
        1252,
        &[
            37, 437, 500, 850, 860, 861, 863, 865, 1252, 10000, 10079, 20127, 28591, 28605,
        ],
    ),
    (CP_UNICODE, &[CP_UNICODE, 65000, 65001]),
];

/// `GetFamilyCodePage`: the family codepage of `codepage`, or `None` when no family lists it.
pub fn family_codepage(codepage: u32) -> Option<u32> {
    FAMILIES
        .iter()
        .find(|(_, codepages)| codepages.contains(&codepage))
        .map(|(family, _)| *family)
}
