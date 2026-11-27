//! Oracle-derived test vectors for the ported qmgrprxy logic.
//!
//! Vectors were captured by compiling the verbatim `include/rpcproxy.h` macros and the verbatim
//! `libs/winecrt0/register.c` bodies, with the export and GUID tables generated from
//! `dlls/qmgrprxy/qmgrprxy.spec`, `qmgrprxy.idl` and `Makefile.in`. The build was
//! `gcc -m32 -fsanitize=address,undefined -g -DWINE_REGISTER_DLL oracle.c` under
//! `target/canon/qmgrprxy/`, and the harness printed 19 rows. None of them trips ASan or UBSan,
//! so no row is excluded.
//!
//! Rows 18 and 19 also record that the C body enumerates `WINE_REGISTRY` resources once and
//! passes a `do_register` flag. The port does not model that enumeration, because it is a Windows
//! API call and this DLL has no resources. Those rows are checked only through their return values.

use super::*;

#[test]
fn oracle_export_rows() {
    // rows 01-05: `export=<name> params=<n> private=1 ptr=4 stack=<bytes>`
    let rows: [(&str, u32, u32); 5] = [
        ("DllCanUnloadNow", 0, 0),
        ("DllGetClassObject", 3, 12),
        ("DllRegisterServer", 0, 0),
        ("DllUnregisterServer", 0, 0),
        ("GetProxyDllInfo", 2, 8),
    ];
    assert_eq!(EXPORTS.len(), rows.len());
    for (entry, (name, params, stack)) in EXPORTS.iter().zip(rows) {
        assert_eq!(entry.name, name);
        assert_eq!(entry.params, params);
        assert!(entry.private);
        assert_eq!(args_size(entry.params, 4), stack);
    }
}

#[test]
fn oracle_lookup_rows() {
    // rows 06-08: `lookup="<name>" found=0`
    assert!(export("DllMain").is_none());
    assert!(export("dllgetclassobject").is_none());
    assert!(export("").is_none());
}

#[test]
fn oracle_module_and_imports_rows() {
    // row 09: `module=qmgrprxy.dll len=12`
    assert_eq!(MODULE, "qmgrprxy.dll");
    assert_eq!(MODULE.len(), 12);
    // row 10: `imports=rpcrt4`
    assert_eq!(IMPORTS, &["rpcrt4"]);
}

#[test]
fn oracle_idl_uuid_rows() {
    // row 11: `parsed=11 data1=0x5ce34c0d data2=0x0dc9 data3=0x4c1f data4=897cdaa1b78cee7c`
    assert_eq!(
        PS_FACTORY_BUFFER_CLSID,
        Guid {
            data1: 0x5ce3_4c0d,
            data2: 0x0dc9,
            data3: 0x4c1f,
            data4: [0x89, 0x7c, 0xda, 0xa1, 0xb7, 0x8c, 0xee, 0x7c],
        }
    );
    // row 12: `guid_text=5ce34c0d-0dc9-4c1f-897c-daa1b78cee7c roundtrip=1`
    assert_eq!(
        guid_string(&PS_FACTORY_BUFFER_CLSID),
        "5ce34c0d-0dc9-4c1f-897c-daa1b78cee7c"
    );
}

#[test]
fn oracle_dll_main_rows() {
    // rows 13-17: `DllMain reason=<n> ret=1 disable_calls=<0|1> stores_instance=<0|1>`
    for (reason, disable, stores) in [
        (0, false, false),
        (1, true, true),
        (2, false, false),
        (3, false, false),
        (5, false, false),
    ] {
        assert_eq!(
            dll_main(reason),
            DllMainOutcome {
                ret: 1,
                disable_thread_library_calls: disable,
                stores_instance: stores,
            }
        );
    }
}

#[test]
fn oracle_register_rows() {
    // row 18: `DllRegisterServer ret=0 enum_calls=1 do_register=1 result=0 type=WINE_REGISTRY`
    // row 19: `DllUnregisterServer ret=0 enum_calls=1 do_register=0 result=0 type=WINE_REGISTRY`
    assert_eq!(register_route(true), RegisterRoute::WineResources);
    assert_eq!(wine_register_resources(), 0);
    assert_eq!(wine_unregister_resources(), 0);
}
