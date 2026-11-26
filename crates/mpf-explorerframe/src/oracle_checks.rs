//! Oracle-derived vectors for the ported explorerframe logic.
//!
//! Captured by compiling the verbatim Wine C bodies from
//! `dlls/explorerframe/nstc.c`, `taskbarlist.c` and `explorerframe_main.c`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/explorerframe/`
//! (harness.c with shim.h, output in oracle.txt). Three tables:
//! - NSTC style and item state: 25 rows (A01-A25)
//! - taskbar method results: 19 rows (one per `ITaskbarList4` method)
//! - module, class object, constructor and QueryInterface: 15 rows (C01-C15)
//!
//! No row was excluded: ASan and UBSan reported nothing.

use super::*;
use crate::item_state::{NSTCIS_DISABLED, NSTCIS_SELECTED};
use crate::style::*;

/// Mirrors the harness's `NSTC2Impl` fields that the control-style rows read.
#[derive(Default)]
struct Ctl {
    style: u32,
    tv: Option<u32>,
    main: u32,
    ex: u32,
}

impl Ctl {
    fn set(&mut self, mask: u32, style: u32) -> Result<ControlStyleUpdate, Hresult> {
        let update = set_control_style(self.style, mask, style, self.tv)?;
        self.style = update.style;
        if update.tv_style.is_some() {
            self.tv = update.tv_style;
        }
        (self.main, self.ex) = apply_host_style(self.main, self.ex, mask, style);
        Ok(update)
    }

    fn snapshot(&self) -> (u32, Option<u32>, u32, u32) {
        (self.style, self.tv, self.main, self.ex)
    }
}

#[test]
fn oracle_treeview_style_rows() {
    // A01-A05: (new_style, changed_bits) as printed by treeview_style_from_nstcs.
    assert_eq!(
        treeview_style_from_nstcs(None, 0, 0),
        (0x5601_ca08, 0x0000_0000)
    );
    assert_eq!(
        treeview_style_from_nstcs(None, 0x3, 0x3),
        (0x5601_ca0b, 0x0000_0003)
    );
    assert_eq!(
        treeview_style_from_nstcs(Some(0xffff_ffff), 0, 0x0000_ffff),
        (0xffff_ebc8, 0x0000_1437)
    );
    assert_eq!(
        treeview_style_from_nstcs(Some(0), 0x0000_0620, 0x0001_0620),
        (0x0000_0008, 0x0000_0008)
    );
    assert_eq!(
        treeview_style_from_nstcs(Some(0x8), 0x0080_0004, 0x0080_0004),
        (0x0000_0508, 0x0000_0500)
    );
}

#[test]
fn oracle_control_style_rows() {
    let mut c = Ctl {
        style: 0,
        tv: Some(0),
        main: WS_BORDER | 0x1000_0000,
        ex: 0,
    };
    // A06, A07, A08: masks with a bit outside every style table -> E_FAIL, no change.
    assert_eq!(c.set(0xffff_ffff, 0), Err(E_FAIL));
    assert_eq!(c.snapshot(), (0, Some(0), 0x1080_0000, 0));
    assert_eq!(c.set(NSTCS_RICHTOOLTIP, NSTCS_RICHTOOLTIP), Err(E_FAIL));
    assert_eq!(
        c.set(NSTCS_SPRINGEXPAND | NSTCS_HASLINES, NSTCS_HASLINES),
        Err(E_FAIL)
    );
    assert_eq!(c.snapshot(), (0, Some(0), 0x1080_0000, 0));

    // A09, A10: NSTCS_BORDER maps to WS_BORDER in the host window.
    assert_eq!(
        c.set(NSTCS_BORDER, NSTCS_BORDER).unwrap().style,
        0x0000_8000
    );
    assert_eq!(c.snapshot(), (0x8000, Some(0), 0x1080_0000, 0));
    c.set(NSTCS_BORDER, 0).unwrap();
    assert_eq!(c.snapshot(), (0, Some(0), 0x1000_0000, 0));

    // A11: NSTCS_TABSTOP maps to WS_EX_CONTROLPARENT.
    c.set(NSTCS_TABSTOP, NSTCS_TABSTOP).unwrap();
    assert_eq!(
        c.snapshot(),
        (0x0002_0000, Some(0), 0x1000_0000, 0x0001_0000)
    );

    // A12: tree-view bits are applied, TABSTOP is cleared in the cached style.
    c.set(NSTCS_TABSTOP | NSTCS_HASLINES, NSTCS_HASLINES)
        .unwrap();
    assert_eq!(
        c.snapshot(),
        (0x0000_0002, Some(0x0000_0002), 0x1000_0000, 0)
    );

    // A13: NSTCS_NOREPLACEOPEN is accepted but unsupported (FIXME path).
    let u = c.set(NSTCS_NOREPLACEOPEN, NSTCS_NOREPLACEOPEN).unwrap();
    assert_eq!(u.unsupported, NSTCS_NOREPLACEOPEN);
    assert_eq!(
        c.snapshot(),
        (0x0000_0802, Some(0x0000_0002), 0x1000_0000, 0)
    );

    // A14: FULLROWSELECT (tree view) and SHOWREFRESHBUTTON (unsupported).
    let u = c
        .set(
            NSTCS_FULLROWSELECT | NSTCS_SHOWREFRESHBUTTON,
            NSTCS_FULLROWSELECT | NSTCS_SHOWREFRESHBUTTON,
        )
        .unwrap();
    assert_eq!(u.unsupported, NSTCS_SHOWREFRESHBUTTON);
    assert_eq!(
        c.snapshot(),
        (0x8000_080a, Some(0x0000_1002), 0x1000_0000, 0)
    );

    // A15: GetControlStyle over the low 16 bits.
    assert_eq!(get_control_style(c.style, 0x0000_ffff), 0x0000_080a);
}

#[test]
fn oracle_control_style2_rows() {
    // A16-A20: style2 starts at 0.
    assert_eq!(set_control_style2(0, 0x3, 0x1), (0x1, 0x1));
    assert_eq!(get_control_style(0x1, 0x7), 0x1);
    assert_eq!(set_control_style2(0x1, 0xff, 0xff), (0xff, 0x3f));
    assert_eq!(set_control_style2(0xff, 0x0f, 0x00), (0xf0, 0x0));
    assert_eq!(get_control_style(0xf0, 0x0f), 0x0);
}

#[test]
fn oracle_item_state_rows() {
    // A21, A24: no tree item -> E_INVALIDARG, flags untouched by the port.
    assert_eq!(get_item_state(false, 0, 0, 0x1f), Err(E_INVALIDARG));
    assert_eq!(
        get_item_state(false, 0, 0, NSTCIS_SELECTED),
        Err(E_INVALIDARG)
    );
    // A22: TVIS_SELECTED | TVIS_BOLD | TVIS_EXPANDED -> SELECTED | EXPANDED | BOLD.
    assert_eq!(get_item_state(true, 0x32, 0, 0x1f), Ok(0x7));
    // A23: TVIS_SELECTED with TVIS_EX_DISABLED, masked to NSTCIS_DISABLED.
    assert_eq!(get_item_state(true, 0x2, 0x2, NSTCIS_DISABLED), Ok(0x8));
    // A25: TVIS_EXPANDED masked to 0x12 (bit 0x10 has no NSTCIS name here).
    assert_eq!(get_item_state(true, 0x20, 0, 0x12), Ok(0x2));
}

#[test]
fn oracle_taskbar_stub_rows() {
    // Table B: one row per ITaskbarList4 method, in the harness's order.
    let rows = [
        (TaskbarMethod::HrInit, S_OK),
        (TaskbarMethod::AddTab, S_OK),
        (TaskbarMethod::DeleteTab, S_OK),
        (TaskbarMethod::ActivateTab, E_NOTIMPL),
        (TaskbarMethod::SetActiveAlt, E_NOTIMPL),
        (TaskbarMethod::MarkFullscreenWindow, E_NOTIMPL),
        (TaskbarMethod::SetProgressValue, S_OK),
        (TaskbarMethod::SetProgressState, S_OK),
        (TaskbarMethod::RegisterTab, S_OK),
        (TaskbarMethod::UnregisterTab, S_OK),
        (TaskbarMethod::SetTabOrder, S_OK),
        (TaskbarMethod::SetTabActive, S_OK),
        (TaskbarMethod::ThumbBarAddButtons, S_OK),
        (TaskbarMethod::ThumbBarUpdateButtons, S_OK),
        (TaskbarMethod::ThumbBarSetImageList, S_OK),
        (TaskbarMethod::SetOverlayIcon, S_OK),
        (TaskbarMethod::SetThumbnailTooltip, S_OK),
        (TaskbarMethod::SetThumbnailClip, E_NOTIMPL),
        (TaskbarMethod::SetTabProperties, E_NOTIMPL),
    ];
    assert_eq!(rows.len(), 19);
    for (method, hr) in rows {
        assert_eq!(method.result(), hr, "{method:?}");
    }
}

#[test]
fn oracle_module_rows() {
    // C01-C03: DllCanUnloadNow.
    assert_eq!(can_unload_now(0), S_OK);
    assert_eq!(can_unload_now(1), S_FALSE);
    assert_eq!(can_unload_now(2), S_FALSE);

    // C04-C07: DllGetClassObject. C06 (all-zero CLSID) and C07 (one byte off the
    // NSTC CLSID) both map to ClassId::Other, since GUID comparison is not modelled.
    assert_eq!(
        get_class_object(ClassId::NamespaceTreeControl),
        Ok(ClassFactory::NamespaceTreeControl)
    );
    assert_eq!(
        get_class_object(ClassId::TaskbarList),
        Ok(ClassFactory::TaskbarList)
    );
    assert_eq!(
        get_class_object(ClassId::Other),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );
    assert_eq!(
        get_class_object(ClassId::Other),
        Err(CLASS_E_CLASSNOTAVAILABLE)
    );

    // C08, C09: TaskbarList_Constructor, aggregated and plain.
    assert_eq!(
        taskbar_list_constructor(true, 0),
        Err(CLASS_E_NOAGGREGATION)
    );
    assert_eq!(taskbar_list_constructor(false, 0), Ok(1));

    // C10-C15: QueryInterface on a refcount-1 object.
    let rows = [
        (TaskbarIid::ITaskbarList, (S_OK, 2)),
        (TaskbarIid::ITaskbarList2, (S_OK, 2)),
        (TaskbarIid::ITaskbarList3, (S_OK, 2)),
        (TaskbarIid::ITaskbarList4, (S_OK, 2)),
        (TaskbarIid::IUnknown, (S_OK, 2)),
        (TaskbarIid::Other, (E_NOINTERFACE, 1)),
    ];
    for (iid, expected) in rows {
        assert_eq!(query_interface(iid, 1), expected, "{iid:?}");
    }
}
