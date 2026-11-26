//! Oracle-derived test vectors for the ported ctl3d32 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/ctl3d32/ctl3d32.c` with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/ctl3d32/` and running the harness. All 22 printed rows are kept:
//! ASan and UBSan reported nothing, so no vector was excluded.

use super::*;

/// Builds the flag state the harness sets with `CTL3D_is_auto_subclass = init`.
fn with_flag(init: bool) -> Ctl3d {
    Ctl3d {
        auto_subclass: init,
    }
}

/// Runs one harness row: sets the flag to `init`, makes the call, and checks
/// the return value and the flag left behind.
fn check(init: bool, call: impl FnOnce(&mut Ctl3d) -> bool, ret: bool, flag_after: bool) {
    let mut c = with_flag(init);
    assert_eq!(call(&mut c), ret);
    assert_eq!(c, with_flag(flag_after));
}

#[test]
fn oracle_single_call_rows() {
    // auto_subclass_init0: Ctl3dAutoSubclass(0x1234) init=0 -> ret=0x1 flag=1
    check(false, |c| c.auto_subclass(0x1234), true, true);
    // auto_subclass_init1: Ctl3dAutoSubclass(0) init=1 -> ret=0x1 flag=1
    check(true, |c| c.auto_subclass(0), true, true);
    // auto_subclass_ex_init0_type0: Ctl3dAutoSubclassEx(0x1234, 0) init=0 -> ret=0x1 flag=1
    check(false, |c| c.auto_subclass_ex(0x1234, 0), true, true);
    // auto_subclass_ex_init1_typemax: Ctl3dAutoSubclassEx(0, 0xffffffff) init=1 -> ret=0x1 flag=1
    check(true, |c| c.auto_subclass_ex(0, u32::MAX), true, true);
    // un_auto_subclass_init1: Ctl3dUnAutoSubclass() init=1 -> ret=0x0 flag=0
    check(true, Ctl3d::un_auto_subclass, false, false);
    // un_auto_subclass_init0: Ctl3dUnAutoSubclass() init=0 -> ret=0x0 flag=0
    check(false, Ctl3d::un_auto_subclass, false, false);
    // unregister_init1: Ctl3dUnregister(0x1234) init=1 -> ret=0x1 flag=0
    check(true, |c| c.unregister(0x1234), true, false);
    // unregister_init0: Ctl3dUnregister(0) init=0 -> ret=0x1 flag=0
    check(false, |c| c.unregister(0), true, false);
    // is_auto_subclass_init0: Ctl3dIsAutoSubclass() init=0 -> ret=0x0 flag=0
    check(false, |c| c.is_auto_subclass(), false, false);
    // is_auto_subclass_init1: Ctl3dIsAutoSubclass() init=1 -> ret=0x1 flag=1
    check(true, |c| c.is_auto_subclass(), true, true);
    // get_ver_init0 and get_ver_init1: Ctl3dGetVer() -> ret=0x21f. The function
    // takes no flag, so both harness rows give the same value.
    assert_eq!(get_ver(), 0x021f);
}

#[test]
fn oracle_sequence_rows() {
    // seq1_auto_subclass .. seq1_is_auto_subclass_after_un, starting from init=0
    let mut c = with_flag(false);
    // seq1_auto_subclass: Ctl3dAutoSubclass(0x1234) -> ret=0x1 flag=1
    assert!(c.auto_subclass(0x1234));
    assert_eq!(c, with_flag(true));
    // seq1_is_auto_subclass: Ctl3dIsAutoSubclass() -> ret=0x1 flag=1
    assert!(c.is_auto_subclass());
    // seq1_un_auto_subclass: Ctl3dUnAutoSubclass() -> ret=0x0 flag=0
    assert!(!c.un_auto_subclass());
    assert_eq!(c, with_flag(false));
    // seq1_is_auto_subclass_after_un: Ctl3dIsAutoSubclass() -> ret=0x0 flag=0
    assert!(!c.is_auto_subclass());

    // seq2_auto_subclass_ex .. seq2_is_auto_subclass_after_unregister, starting from init=0
    let mut c = with_flag(false);
    // seq2_auto_subclass_ex: Ctl3dAutoSubclassEx(0x1234, 2) -> ret=0x1 flag=1
    assert!(c.auto_subclass_ex(0x1234, 2));
    assert_eq!(c, with_flag(true));
    // seq2_unregister: Ctl3dUnregister(0x1234) -> ret=0x1 flag=0
    assert!(c.unregister(0x1234));
    assert_eq!(c, with_flag(false));
    // seq2_is_auto_subclass_after_unregister: Ctl3dIsAutoSubclass() -> ret=0x0 flag=0
    assert!(!c.is_auto_subclass());

    // seq3_unregister .. seq3_is_auto_subclass, starting from init=1
    let mut c = with_flag(true);
    // seq3_unregister: Ctl3dUnregister(0) -> ret=0x1 flag=0
    assert!(c.unregister(0));
    assert_eq!(c, with_flag(false));
    // seq3_auto_subclass_ex: Ctl3dAutoSubclassEx(0, 1) -> ret=0x1 flag=1
    assert!(c.auto_subclass_ex(0, 1));
    assert_eq!(c, with_flag(true));
    // seq3_is_auto_subclass: Ctl3dIsAutoSubclass() -> ret=0x1 flag=1
    assert!(c.is_auto_subclass());
}
