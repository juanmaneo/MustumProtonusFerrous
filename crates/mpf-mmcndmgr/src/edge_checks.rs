//! Edge-case tests for the ported mmcndmgr logic.
//!
//! These cover the GUID comparison boundary, the NULL out-pointer checks, the class-id dispatch
//! order, the fixed reference counts, and the GUID, HRESULT and stub constants. Not every
//! expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/mmc.idl, include/unknwn.idl, include/winerror.h,
    // include/winternl.h and include/winnt.h.
    assert_eq!(
        IID_IUNKNOWN,
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(
        IID_ICLASSFACTORY,
        Guid {
            data1: 1,
            data2: 0,
            data3: 0,
            data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46],
        }
    );
    assert_eq!(
        IID_IMMCVERSIONINFO,
        Guid {
            data1: 0xa8d2_c5fe,
            data2: 0xcdcb,
            data3: 0x4b9d,
            data4: [0xbd, 0xe5, 0xa2, 0x73, 0x43, 0xff, 0x54, 0xbc],
        }
    );
    assert_eq!(
        CLSID_MMCVERSIONINFO,
        Guid {
            data1: 0xd6fe_db1d,
            data2: 0xcf21,
            data3: 0x4bd9,
            data4: [0xaf, 0x3b, 0xc5, 0x46, 0x8e, 0x9c, 0x66, 0x84],
        }
    );
    assert_eq!(S_OK, 0);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn guid_match_needs_every_field() {
    // Not oracle-derived: IsEqualGUID compares the whole GUID, so a one-bit difference in any
    // field refuses the match. The oracle only passed exact GUIDs and unrelated ones.
    let mut near = IID_IMMCVERSIONINFO;
    near.data1 ^= 1;
    assert_eq!(mmcversion_query_interface(&near), Err(E_NOINTERFACE));
    let mut near = IID_IMMCVERSIONINFO;
    near.data2 ^= 1;
    assert_eq!(mmcversion_query_interface(&near), Err(E_NOINTERFACE));
    let mut near = IID_IMMCVERSIONINFO;
    near.data3 ^= 1;
    assert_eq!(mmcversion_query_interface(&near), Err(E_NOINTERFACE));
    let mut near = IID_IMMCVERSIONINFO;
    near.data4[0] ^= 1;
    assert_eq!(mmcversion_query_interface(&near), Err(E_NOINTERFACE));
    let mut near = IID_IMMCVERSIONINFO;
    near.data4[7] ^= 1;
    assert_eq!(mmcversion_query_interface(&near), Err(E_NOINTERFACE));
    assert_eq!(
        mmcversion_query_interface(&IID_IMMCVERSIONINFO),
        Ok(Object::MmcVersionInfo)
    );
}

#[test]
fn near_miss_class_id_is_not_served() {
    // Not oracle-derived: the class id is compared in full, so a one-byte difference from
    // CLSID_MMCVersionInfo gives CLASS_E_CLASSNOTAVAILABLE and the IID is never looked at.
    let mut near = CLSID_MMCVERSIONINFO;
    near.data4[7] ^= 1;
    assert_eq!(
        dll_get_class_object(&near, &IID_ICLASSFACTORY),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        dll_get_class_object(&near, &IID_IMMCVERSIONINFO),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn unknown_class_id_is_checked_before_the_iid() {
    // Not oracle-derived: the C checks the class id first. An unknown class id gives
    // CLASS_E_CLASSNOTAVAILABLE even for an IID the factory would refuse with E_NOINTERFACE.
    assert_eq!(
        dll_get_class_object(&IID_IMMCVERSIONINFO, &IID_IMMCVERSIONINFO),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
}

#[test]
fn get_mmc_version_with_no_out_pointers_is_s_ok() {
    // Not oracle-derived: the oracle never passed NULL for both pointers. The C writes nothing
    // and returns S_OK.
    assert_eq!(mmcversion_get_mmc_version(None, None), S_OK);
}

#[test]
fn reference_counts_are_fixed() {
    // Not oracle-derived: the oracle printed each count once. The C keeps no count, so repeated
    // calls give the same values.
    for _ in 0..3 {
        assert_eq!(mmcversion_add_ref(), 2);
        assert_eq!(mmcversion_release(), 1);
        assert_eq!(class_factory_add_ref(), 2);
        assert_eq!(class_factory_release(), 1);
    }
}

#[test]
fn lock_server_ignores_the_lock_flag() {
    // Not oracle-derived: the oracle passed only TRUE. The C does not read fLock, so the port
    // returns S_OK with no argument.
    assert_eq!(class_factory_lock_server(), S_OK);
    assert_eq!(class_factory_lock_server(), S_OK);
}

#[test]
fn stub_names_pass_through_unchanged() {
    // Not oracle-derived: the stub passes its arguments through, and the code and flags do not
    // depend on the names.
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
fn stub_exports_match_spec_lines() {
    // Not oracle-derived: the three `@ stub` lines of dlls/mmcndmgr/mmcndmgr.spec, in order:
    // `@ stub CreateExecutivePlatform`, `@ stub -arch=win32 _GetAllocCounters@0` and
    // `@ stub -arch=win64 GetAllocCounters`.
    assert_eq!(
        create_executive_platform().function,
        "CreateExecutivePlatform"
    );
    assert_eq!(get_alloc_counters_win32().function, "_GetAllocCounters@0");
    assert_eq!(get_alloc_counters_win64().function, "GetAllocCounters");
    for stub in [
        create_executive_platform(),
        get_alloc_counters_win32(),
        get_alloc_counters_win64(),
    ] {
        assert_eq!(stub.module, "mmcndmgr.dll");
    }
}
