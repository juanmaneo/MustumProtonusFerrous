//! Port of the portable logic in Wine's `dlls/explorerframe/nstc.c`,
//! `taskbarlist.c` and `explorerframe_main.c`.
//!
//! Ported:
//! - `NSTC2_fnSetControlStyle` / `NSTC2_fnSetControlStyle2` mask validation,
//!   unsupported-bit detection and state update: [`set_control_style`],
//!   [`set_control_style2`], [`apply_host_style`]
//! - `treeview_style_from_nstcs`: [`treeview_style_from_nstcs`]
//! - `NSTC2_fnGetControlStyle` / `NSTC2_fnGetControlStyle2`: [`get_control_style`]
//! - `NSTC2_fnGetItemState` flag mapping and the `E_INVALIDARG` path: [`get_item_state`]
//! - Taskbar method return values (`taskbar_list_*` stubs): [`TaskbarMethod`]
//! - `taskbar_list_QueryInterface` IID acceptance and refcount effect: [`TaskbarIid`], [`query_interface`]
//! - `DllCanUnloadNow`: [`can_unload_now`]
//! - `DllGetClassObject` class selection: [`ClassId`], [`ClassFactory`], [`get_class_object`]
//! - `TaskbarList_Constructor` aggregation check and `EFRAME_LockModule`: [`taskbar_list_constructor`]
//!
//! Skipped:
//! - Every `IUnknown` / `IClassFactory` vtable entry, `TaskbarList_Constructor`'s
//!   allocation and `Release`, and `DllMain`: COM object lifetime and OS state.
//! - The `SendMessageW` side of `NSTC2_fnSetItemState`, `GetItemState`'s tree
//!   lookup and all `nstc.c` tree-view callbacks, item insertion and expansion:
//!   they drive a Win32 tree-view control.
//! - `SetWindowLongPtrW` / `GetWindowLongPtrW` on the host and tree-view windows.
//!   [`set_control_style`] and [`apply_host_style`] return the new values and leave
//!   the write to the caller.
//! - The `GetDesktopWindow` / `WM_WINE_ADD_TAB` / `WM_WINE_DELETE_TAB` messages
//!   sent by `AddTab` and `DeleteTab`: the methods' `S_OK` results are modelled,
//!   the window-manager messages are not.

mod factory;
mod hresult;
mod item_state;
mod style;
mod taskbar;

pub use factory::{
    can_unload_now, get_class_object, taskbar_list_constructor, ClassFactory, ClassId,
};
pub use hresult::{
    succeeded, Hresult, CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_FAIL, E_INVALIDARG,
    E_NOINTERFACE, E_NOTIMPL, S_FALSE, S_OK,
};
pub use item_state::get_item_state;
pub use style::{
    apply_host_style, get_control_style, set_control_style, set_control_style2,
    treeview_style_from_nstcs, ControlStyleUpdate,
};
pub use taskbar::{query_interface, TaskbarIid, TaskbarMethod};

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
