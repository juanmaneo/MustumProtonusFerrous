//! Edge-case tests for the ported pstorec logic.
//!
//! These cover the constants, the IID comparison, the reference-count wrap and zero boundaries,
//! the allocation failure contract, and the export and stub tables. Not every expectation is
//! oracle-derived; the ones that aren't say so in a comment.

use super::*;

fn provider(ref_count: i32) -> PStoreProvider {
    PStoreProvider { ref_count }
}

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winerror.h, include/winternl.h and include/winnt.h.
    assert_eq!(S_OK, 0);
    assert_eq!(S_FALSE, 1);
    assert_eq!(E_NOINTERFACE, 0x8000_4002);
    assert_eq!(E_NOTIMPL, 0x8000_4001);
    assert_eq!(E_OUTOFMEMORY, 0x8007_000e);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn iid_values_are_pinned() {
    // Not oracle-derived: values from include/pstore.idl and include/unknwn.idl.
    assert_eq!(
        IID_IPSTORE,
        Guid {
            data1: 0x5a6f_1ec0,
            data2: 0x2db1,
            data3: 0x11d0,
            data4: [0x8c, 0x39, 0x00, 0xc0, 0x4f, 0xd9, 0x12, 0x6b],
        }
    );
    assert_eq!(
        IID_IUNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
}

#[test]
fn query_interface_rejects_a_guid_that_differs_in_any_field() {
    // Not oracle-derived: the harness only tried one near miss (row 07). Each field is changed
    // by one bit here to check the whole-GUID comparison.
    let fields = [
        Guid {
            data1: IID_IPSTORE.data1 ^ 1,
            ..IID_IPSTORE
        },
        Guid {
            data2: IID_IPSTORE.data2 ^ 1,
            ..IID_IPSTORE
        },
        Guid {
            data3: IID_IPSTORE.data3 ^ 1,
            ..IID_IPSTORE
        },
        Guid {
            data4: [0x8d, 0x39, 0x00, 0xc0, 0x4f, 0xd9, 0x12, 0x6b],
            ..IID_IPSTORE
        },
    ];
    for riid in &fields {
        let mut p = provider(1);
        assert_eq!(p.query_interface(riid), E_NOINTERFACE);
        assert_eq!(p.ref_count, 1);
    }

    // Not oracle-derived: the all-zero GUID is neither IID.
    let mut p = provider(1);
    assert_eq!(
        p.query_interface(&Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }),
        E_NOINTERFACE
    );
    assert_eq!(p.ref_count, 1);
}

#[test]
fn ref_count_round_trip_reaches_zero_at_the_free_point() {
    // Not oracle-derived: the free point is when `release` returns 0, so 0 -> 1 -> 0 is the
    // boundary that the owner must act on.
    let mut p = provider(0);
    assert_eq!(p.add_ref(), 1);
    assert_eq!(p.release(), 0);
    assert_eq!(p.ref_count, 0);
}

#[test]
fn counts_below_zero_are_returned_as_unsigned_bit_patterns() {
    // Not oracle-derived: the C functions return ULONG from a LONG count. The AddRef from -1
    // returns 0 (not a free, since only Release frees), and Release from -2 returns 0xfffffffd.
    let mut p = provider(-1);
    assert_eq!(p.add_ref(), 0);
    assert_eq!(p.ref_count, 0);

    let mut p = provider(-2);
    assert_eq!(p.release(), 0xffff_fffd);
    assert_eq!(p.ref_count, -3);
}

#[test]
fn failed_allocation_leaves_an_empty_out_slot_empty() {
    // Not oracle-derived: the harness checked the sentinel case (row 04). This checks the
    // `None` start, which is the C `*ppProvider == NULL` state.
    let mut out = None;
    assert_eq!(
        pstore_create_instance(&mut out, &mut || false),
        E_OUTOFMEMORY
    );
    assert_eq!(out, None);
}

#[test]
fn successful_allocation_overwrites_the_out_slot() {
    // Not oracle-derived: the C body assigns `*ppProvider` unconditionally on success, so an
    // existing value is replaced.
    let mut out = Some(provider(42));
    assert_eq!(pstore_create_instance(&mut out, &mut || true), S_OK);
    assert_eq!(out, Some(provider(1)));
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the harness only passed the fixed names of pstorec.spec. The code and
    // flags are the same for any names.
    let other = wine_spec_unimplemented_stub("other.dll", "Foo");
    assert_eq!(other.module, "other.dll");
    assert_eq!(other.function, "Foo");
    assert_eq!(other.code, EXCEPTION_WINE_STUB);
    assert_eq!(other.flags, EXCEPTION_NONCONTINUABLE);
    assert_eq!(other.number_parameters, 2);

    let empty = wine_spec_unimplemented_stub("", "");
    assert_eq!(empty.module, "");
    assert_eq!(empty.function, "");
}

#[test]
fn export_table_matches_spec_order() {
    // Not oracle-derived: the seven lines of dlls/pstorec/pstorec.spec, in order.
    assert_eq!(
        EXPORTS,
        [
            ("DllCanUnloadNow", ExportKind::Private),
            ("DllGetClassObject", ExportKind::Private),
            ("DllRegisterServer", ExportKind::Private),
            ("DllUnregisterServer", ExportKind::Private),
            ("GetPStoreProvider", ExportKind::Stub),
            ("PStoreCreateInstance", ExportKind::Stdcall),
            ("PStoreEnumProviders", ExportKind::Stub),
        ]
    );
    assert_eq!(
        export_kind("PStoreCreateInstance"),
        Some(ExportKind::Stdcall)
    );
    assert_eq!(export_kind("GetPStoreProvider"), Some(ExportKind::Stub));
    assert_eq!(export_kind("DllRegisterServer"), Some(ExportKind::Private));
    assert_eq!(export_kind("pstorecreateinstance"), None);
    assert_eq!(export_kind(""), None);
}
