//! Oracle-derived test vectors for the ported ctl3dv2.dll16 logic.
//!
//! Vectors were captured by compiling the verbatim Wine C body of `dlls/ctl3dv2.dll16/ctl3d.c`
//! and the verbatim `__wine_spec_unimplemented_stub` body from `dlls/ntdll/exception.c` with
//! `gcc -fsanitize=address,undefined -g` under `target/canon/ctl3dv2_dll16/`, and running the
//! harness once. The 25 printed rows run in order on one flag, as the C process did, so the
//! test below replays them on one [`Ctl3d`]. ASan and UBSan reported nothing, so no vector was
//! excluded. The harness output is kept in `target/canon/ctl3dv2_dll16/oracle.txt`.

use super::*;

#[test]
fn oracle_rows() {
    // The flag starts FALSE, as `static BOOL16 CTL3D16_is_auto_subclass = FALSE;`.
    let mut c = Ctl3d::default();

    // row 01 Ctl3dGetVer() -> ret=0x021f
    assert_eq!(get_ver(), 0x021f);
    // row 02 Ctl3dSubclassDlg(0x1234,0x0007) -> ret=0
    assert!(!subclass_dlg(0x1234, 0x0007));
    // row 03 Ctl3dSubclassCtl(0x1234) -> ret=0
    assert!(!subclass_ctl(0x1234));
    // row 04 Ctl3dCtlColor(0x0001,0x12345678) -> ret=0
    assert_eq!(ctl_color(0x0001, 0x1234_5678), 0);
    // row 05 Ctl3dEnabled() -> ret=0
    assert!(!enabled());
    // row 06 Ctl3dColorChange() -> ret=1
    assert!(color_change());
    // row 07 BtnWndProc3d(0x1234,0x00f5,0x0001,0x7fffffff) -> ret=0
    assert_eq!(btn_wnd_proc_3d(0x1234, 0x00f5, 0x0001, 0x7fff_ffff), 0);
    // row 08 EditWndProc3d(0x1234,0x0014,0x0000,-1) -> ret=0
    assert_eq!(edit_wnd_proc_3d(0x1234, 0x0014, 0x0000, -1), 0);
    // row 09 ListWndProc3d(0x1234,0x0020,0x0001,0x0001) -> ret=0
    assert_eq!(list_wnd_proc_3d(0x1234, 0x0020, 0x0001, 0x0001), 0);
    // row 10 ComboWndProc3d(0x1234,0x0110,0x0002,0x0003) -> ret=0
    assert_eq!(combo_wnd_proc_3d(0x1234, 0x0110, 0x0002, 0x0003), 0);
    // row 11 StaticWndProc3d(0x1234,0x0138,0xffff,-2147483648) -> ret=0
    assert_eq!(static_wnd_proc_3d(0x1234, 0x0138, 0xffff, i32::MIN), 0);
    // row 12 Ctl3dRegister(0x1234) -> ret=1
    assert!(register(0x1234));
    // row 13 Ctl3dUnregister(0x1234) -> ret=1 flag=0->0
    assert!(c.unregister(0x1234));
    assert!(!c.is_auto_subclass());
    // row 14 Ctl3dAutoSubclass(0x1234) -> ret=1 flag=0->1
    assert!(c.auto_subclass(0x1234));
    assert!(c.is_auto_subclass());
    // row 15 Ctl3dDlgProc(0x1234,0x0110,0x0001,0x2) -> ret=0
    assert_eq!(dlg_proc(0x1234, 0x0110, 0x0001, 0x2), 0);
    // row 16 Ctl3dCtlColorEx(0x0111,0x0000,0x0) -> ret=0
    assert_eq!(ctl_color_ex(0x0111, 0x0000, 0), 0);
    // row 17 Ctl3dSetStyle() -> raise code=0x80000100 flags=0x1 nargs=2 p0=ctl3dv2.dll16 p1=Ctl3dSetStyle
    assert_eq!(
        set_style(),
        StubException {
            code: 0x8000_0100,
            flags: 0x01,
            number_parameters: 2,
            module: "ctl3dv2.dll16",
            function: "Ctl3dSetStyle",
        }
    );
    // row 18 Ctl3dDlgFramePaint(0x1234,0x0085,0,0) -> ret=0
    assert_eq!(dlg_frame_paint(0x1234, 0x0085, 0x0000, 0), 0);
    // row 19 Ctl3dSubclassDlgEx(0x1234,0xffffffff) -> ret=0
    assert!(!subclass_dlg_ex(0x1234, u32::MAX));
    // row 20 Ctl3dWinIniChange() -> flag=1->1
    assert!(c.is_auto_subclass());
    win_ini_change();
    assert!(c.is_auto_subclass());
    // row 21 Ctl3dIsAutoSubclass() -> ret=1
    assert!(c.is_auto_subclass());
    // row 22 Ctl3dUnAutoSubclass() -> ret=0 flag=1->0
    assert!(!c.un_auto_subclass());
    assert!(!c.is_auto_subclass());
    // row 23 Ctl3dSubclassCtlEx(0x1234,-1) -> ret=0
    assert!(!subclass_ctl_ex(0x1234, -1));
    // row 24 Ctl3dUnsubclassCtl(0x1234) -> ret=0
    assert!(!unsubclass_ctl(0x1234));
    // row 25 Ctl3dAutoSubclassEx(0x1234,2) -> ret=1 flag=0->1
    assert!(c.auto_subclass_ex(0x1234, 2));
    assert!(c.is_auto_subclass());
}
