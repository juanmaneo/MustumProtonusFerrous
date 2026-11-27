/// `Ctl3dCtlColor`: returns a NULL `HBRUSH`, so the caller gets no brush.
pub fn ctl_color(_hdc: u16, _hwnd: i32) -> usize {
    0
}

/// `Ctl3dCtlColorEx`: returns a NULL `HBRUSH`, so the caller gets no brush.
pub fn ctl_color_ex(_msg: u16, _wparam: u16, _lparam: i32) -> usize {
    0
}

/// `Ctl3dDlgFramePaint`: returns 0 and paints nothing.
pub fn dlg_frame_paint(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `Ctl3dDlgProc`: returns 0 and handles nothing.
pub fn dlg_proc(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `BtnWndProc3d`: returns 0 and handles nothing.
pub fn btn_wnd_proc_3d(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `EditWndProc3d`: returns 0 and handles nothing.
pub fn edit_wnd_proc_3d(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `ListWndProc3d`: returns 0 and handles nothing.
pub fn list_wnd_proc_3d(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `ComboWndProc3d`: returns 0 and handles nothing.
pub fn combo_wnd_proc_3d(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}

/// `StaticWndProc3d`: returns 0 and handles nothing.
pub fn static_wnd_proc_3d(_hwnd: u16, _msg: u16, _wparam: u16, _lparam: i32) -> i32 {
    0
}
