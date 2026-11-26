//! Oracle-derived test vectors for the ported pstorec logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `dlls/pstorec/pstorec.c`,
//! `libs/winecrt0/dll_canunload.c` and `__wine_spec_unimplemented_stub` (`dlls/ntdll/exception.c`)
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/pstorec/`, and running the
//! harness. The harness printed 17 rows. Row 17 (`PStoreCreateInstance` with a NULL
//! `ppProvider`) is flagged by UBSan (`store to null pointer`) and ASan (SEGV), so it is excluded
//! here, which leaves 16 rows. The 19 `IPStore` methods share row 14.
//!
//! The harness passes sentinel out pointers and checks whether they change. The port has no
//! out pointers, so the sentinel is modelled as an existing `Some(...)` value that must survive.

use super::*;

fn provider(ref_count: i32) -> PStoreProvider {
    PStoreProvider { ref_count }
}

#[test]
fn oracle_dll_entry_rows() {
    // row 01 DllCanUnloadNow -> hr=0x00000001
    assert_eq!(dll_can_unload_now(), 0x0000_0001);
    // row 02 DllGetClassObject clsid=zero iid=IPStore out=sentinel -> hr=0x80040111 out=sentinel
    assert_eq!(dll_get_class_object(), 0x8004_0111);
}

#[test]
fn oracle_pstore_create_instance_rows() {
    // row 03 PStoreCreateInstance alloc=ok out=null -> hr=0x00000000 out=set ref=1 allocs=1
    let mut out = None;
    let mut allocs = 0;
    let hr = pstore_create_instance(&mut out, &mut || {
        allocs += 1;
        true
    });
    assert_eq!(hr, 0x0000_0000);
    assert_eq!(out, Some(provider(1)));
    assert_eq!(allocs, 1);

    // row 04 PStoreCreateInstance alloc=fail out=sentinel -> hr=0x8007000e out=sentinel allocs=1
    let mut out = Some(provider(42));
    let mut allocs = 0;
    let hr = pstore_create_instance(&mut out, &mut || {
        allocs += 1;
        false
    });
    assert_eq!(hr, 0x8007_000e);
    assert_eq!(out, Some(provider(42)));
    assert_eq!(allocs, 1);
}

#[test]
fn oracle_query_interface_rows() {
    // row 05 QueryInterface(IPStore) ref_in=1 -> hr=0x00000000 out=set ref_out=2
    let mut p = provider(1);
    assert_eq!(p.query_interface(&IID_IPSTORE), 0x0000_0000);
    assert_eq!(p.ref_count, 2);

    // row 06 QueryInterface(IUnknown) ref_in=1 -> hr=0x00000000 out=set ref_out=2
    let mut p = provider(1);
    assert_eq!(p.query_interface(&IID_IUNKNOWN), 0x0000_0000);
    assert_eq!(p.ref_count, 2);

    // row 07 QueryInterface(IPStore with Data4[7]^=1) ref_in=1 -> hr=0x80004002 out=null ref_out=1
    let near_miss = Guid {
        data4: [0x8c, 0x39, 0x00, 0xc0, 0x4f, 0xd9, 0x12, 0x6a],
        ..IID_IPSTORE
    };
    let mut p = provider(1);
    assert_eq!(p.query_interface(&near_miss), 0x8000_4002);
    assert_eq!(p.ref_count, 1);
}

#[test]
fn oracle_ref_count_rows() {
    // row 08 AddRef ref_in=1 -> ret=2 ref_out=2
    let mut p = provider(1);
    assert_eq!(p.add_ref(), 2);
    assert_eq!(p.ref_count, 2);

    // row 09 AddRef ref_in=0x7fffffff -> ret=0x80000000 ref_out=-2147483648
    let mut p = provider(i32::MAX);
    assert_eq!(p.add_ref(), 0x8000_0000);
    assert_eq!(p.ref_count, -2_147_483_648);

    // row 10 Release ref_in=2 -> ret=1 ref_out=1
    let mut p = provider(2);
    assert_eq!(p.release(), 1);
    assert_eq!(p.ref_count, 1);

    // row 11 Release ref_in=1 -> ret=0 freed=1 (the caller drops the object at 0)
    let mut p = provider(1);
    assert_eq!(p.release(), 0);

    // row 12 Release ref_in=0 -> ret=0xffffffff ref_out=-1
    let mut p = provider(0);
    assert_eq!(p.release(), 0xffff_ffff);
    assert_eq!(p.ref_count, -1);

    // row 13 Release ref_in=INT_MIN -> ret=0x7fffffff ref_out=2147483647
    let mut p = provider(i32::MIN);
    assert_eq!(p.release(), 0x7fff_ffff);
    assert_eq!(p.ref_count, 2_147_483_647);
}

#[test]
fn oracle_not_implemented_row() {
    // row 14 IPStore E_NOTIMPL methods via lpVtbl -> GetInfo=0x80004001 ... EnumItems=0x80004001
    let results = [
        get_info(),
        get_prov_param(),
        set_prov_param(),
        create_type(),
        get_type_info(),
        delete_type(),
        create_subtype(),
        get_subtype_info(),
        delete_subtype(),
        read_access_ruleset(),
        write_access_ruleset(),
        enum_types(),
        enum_subtypes(),
        delete_item(),
        read_item(),
        write_item(),
        open_item(),
        close_item(),
        enum_items(),
    ];
    assert_eq!(results, [0x8000_4001; 19]);
}

#[test]
fn oracle_stub_rows() {
    // row 15 GetPStoreProvider -> code=0x80000100 flags=0x01 nargs=2 p0=pstorec.dll p1=GetPStoreProvider
    assert_eq!(
        get_pstore_provider(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "pstorec.dll",
            function: "GetPStoreProvider",
        }
    );

    // row 16 PStoreEnumProviders -> code=0x80000100 flags=0x01 nargs=2 p0=pstorec.dll p1=PStoreEnumProviders
    assert_eq!(
        pstore_enum_providers(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "pstorec.dll",
            function: "PStoreEnumProviders",
        }
    );
}
