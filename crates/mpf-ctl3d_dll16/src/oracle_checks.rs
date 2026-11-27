//! Oracle-derived test vectors for the ported ctl3d.dll16 logic.
//!
//! Vectors were captured by compiling the verbatim `dlls/ctl3d.dll16/ctl3d.c` and the
//! `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/ctl3d_dll16/`, and running the
//! harness. It prints 25 rows, one per export in spec order. ASan and UBSan reported nothing,
//! so no row is excluded.

use super::*;

#[test]
fn oracle_rows_in_spec_order() {
    // One instance serves every row, as the C static serves the whole process. The flag
    // transitions come from the `flag=` column of each row.
    let mut ctl = Ctl3d::default();

    // row 01 ord=1 Ctl3dGetVer() ret=543 flag=0
    assert_eq!(ctl3d_get_ver(), 543);
    assert!(!ctl.is_auto_subclass());

    // row 02 ord=2 Ctl3dSubclassDlg(hwnd=0x1234,types=0x00ff) ret=0 flag=0
    assert!(!ctl3d_subclass_dlg(0x1234, 0x00ff));
    assert!(!ctl.is_auto_subclass());

    // row 03 ord=3 Ctl3dSubclassCtl(hwnd=0x1234) ret=0 flag=0
    assert!(!ctl3d_subclass_ctl(0x1234));
    assert!(!ctl.is_auto_subclass());

    // row 04 ord=4 Ctl3dCtlColor(hdc=0x0042,hwnd=-7) ret=0 flag=0
    assert_eq!(ctl3d_ctl_color(0x0042, -7), 0);
    assert!(!ctl.is_auto_subclass());

    // row 05 ord=5 Ctl3dEnabled() ret=0 flag=0
    assert!(!ctl3d_enabled());
    assert!(!ctl.is_auto_subclass());

    // row 06 ord=6 Ctl3dColorChange() ret=1 flag=0
    assert!(ctl3d_color_change());
    assert!(!ctl.is_auto_subclass());

    // row 07 ord=7 BtnWndProc3d(hwnd=1,msg=0x0201,wparam=1,lparam=0x50003) ret=0 flag=0
    assert_eq!(btn_wnd_proc3d(1, 0x0201, 1, 0x0005_0003), 0);
    assert!(!ctl.is_auto_subclass());

    // row 08 ord=8 EditWndProc3d(hwnd=2,msg=0x0100,wparam=9,lparam=-1) ret=0 flag=0
    assert_eq!(edit_wnd_proc3d(2, 0x0100, 9, -1), 0);
    assert!(!ctl.is_auto_subclass());

    // row 09 ord=9 ListWndProc3d(hwnd=3,msg=0x0114,wparam=0,lparam=0) ret=0 flag=0
    assert_eq!(list_wnd_proc3d(3, 0x0114, 0, 0), 0);
    assert!(!ctl.is_auto_subclass());

    // row 10 ord=10 ComboWndProc3d(hwnd=4,msg=0x0135,wparam=0x8000,lparam=0x7fffffff) ret=0 flag=0
    assert_eq!(combo_wnd_proc3d(4, 0x0135, 0x8000, 0x7fff_ffff), 0);
    assert!(!ctl.is_auto_subclass());

    // row 11 ord=11 StaticWndProc3d(hwnd=5,msg=0x0014,wparam=0xffff,lparam=-2) ret=0 flag=0
    assert_eq!(static_wnd_proc3d(5, 0x0014, 0xffff, -2), 0);
    assert!(!ctl.is_auto_subclass());

    // row 12 ord=12 Ctl3dRegister(hinst=0x0777) ret=0 flag=0
    assert!(!ctl3d_register(0x0777));
    assert!(!ctl.is_auto_subclass());

    // row 13 ord=13 Ctl3dUnregister(hinst=0x0777) ret=1 flag=0
    assert!(ctl.unregister(0x0777));
    assert!(!ctl.is_auto_subclass());

    // row 14 ord=16 Ctl3dAutoSubclass(hinst=0x0777) ret=1 flag=1
    assert!(ctl.auto_subclass(0x0777));
    assert!(ctl.is_auto_subclass());

    // row 15 ord=17 Ctl3dDlgProc(hwnd=6,msg=0x0110,wparam=0,lparam=0) ret=0 flag=1
    assert_eq!(ctl3d_dlg_proc(6, 0x0110, 0, 0), 0);
    assert!(ctl.is_auto_subclass());

    // row 16 ord=18 Ctl3dCtlColorEx(msg=0x0138,wparam=1,lparam=0x1234) ret=0 flag=1
    assert_eq!(ctl3d_ctl_color_ex(0x0138, 1, 0x1234), 0);
    assert!(ctl.is_auto_subclass());

    // row 17 ord=19 Ctl3dSetStyle() stub raises=1 code=0x80000100 flags=0x1 nparams=2
    //               module=ctl3d.dll16 function=Ctl3dSetStyle
    assert_eq!(
        ctl3d_set_style(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ctl3d.dll16",
            function: "Ctl3dSetStyle",
        }
    );
    assert!(ctl.is_auto_subclass());

    // row 18 ord=20 Ctl3dDlgFramePaint(hwnd=7,msg=0x0014,wparam=0,lparam=0) ret=0 flag=1
    assert_eq!(ctl3d_dlg_frame_paint(7, 0x0014, 0, 0), 0);
    assert!(ctl.is_auto_subclass());

    // row 19 ord=21 Ctl3dSubclassDlgEx(hwnd=8,types=0xdeadbeef) ret=0 flag=1
    assert!(!ctl3d_subclass_dlg_ex(8, 0xdead_beef));
    assert!(ctl.is_auto_subclass());

    // row 20 ord=22 Ctl3dWinIniChange() ret=void flag=1
    ctl3d_win_ini_change();
    assert!(ctl.is_auto_subclass());

    // row 21 ord=23 Ctl3dIsAutoSubclass() ret=1 flag=1
    assert!(ctl.is_auto_subclass());

    // row 22 ord=24 Ctl3dUnAutoSubclass() ret=0 flag=0
    assert!(!ctl.un_auto_subclass());
    assert!(!ctl.is_auto_subclass());

    // row 23 ord=25 Ctl3dSubclassCtlEx(hwnd=9,type=-1) ret=0 flag=0
    assert!(!ctl3d_subclass_ctl_ex(9, -1));
    assert!(!ctl.is_auto_subclass());

    // row 24 ord=26 Ctl3dUnsubclassCtl(hwnd=10) ret=0 flag=0
    assert!(!ctl3d_unsubclass_ctl(10));
    assert!(!ctl.is_auto_subclass());

    // row 25 ord=27 Ctl3dAutoSubclassEx(hinst=0x0888,type=0x80000000) ret=1 flag=1
    assert!(ctl.auto_subclass_ex(0x0888, 0x8000_0000));
    assert!(ctl.is_auto_subclass());
}
