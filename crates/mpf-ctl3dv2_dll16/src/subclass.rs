/// `Ctl3dSubclassDlg`: returns FALSE and subclasses nothing.
pub fn subclass_dlg(_hwnd: u16, _types: u16) -> bool {
    false
}

/// `Ctl3dSubclassDlgEx`: returns FALSE and subclasses nothing.
pub fn subclass_dlg_ex(_hwnd: u16, _types: u32) -> bool {
    false
}

/// `Ctl3dSubclassCtl`: returns FALSE and subclasses nothing.
pub fn subclass_ctl(_hwnd: u16) -> bool {
    false
}

/// `Ctl3dSubclassCtlEx`: returns FALSE and subclasses nothing.
pub fn subclass_ctl_ex(_hwnd: u16, _ty: i16) -> bool {
    false
}

/// `Ctl3dUnsubclassCtl`: returns FALSE and unsubclasses nothing.
pub fn unsubclass_ctl(_hwnd: u16) -> bool {
    false
}
