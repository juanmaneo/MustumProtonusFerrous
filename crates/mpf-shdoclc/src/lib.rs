//! Port of the portable data in Wine's dlls/shdoclc/shdoclc.rc and dlls/shdoclc/shdoclc.h.
//!
//! shdoclc is a data-only DLL. dlls/shdoclc/Makefile.in passes -Wb,--data-only, and the directory
//! has no .spec file and no C source. The DLL therefore has no export table, no @ stub export and
//! no C function, so the stub model does not apply. The port covers the resources and constants
//! that the DLL carries. The verdict is partial.
//!
//! Ported:
//! - The LANGUAGE statement: LANG_ID, makelangid, LANG_ENGLISH and SUBLANG_DEFAULT.
//! - The two STRINGTABLE blocks, with the English text: string_table_entry and the IDS_ constants.
//! - The IDR_BROWSE_CONTEXT_MENU MENU: BROWSE_CONTEXT_MENU, MenuItem, menu_resource, and the IDM_,
//!   IDR_ and _IDM_ constants of shdoclc.h. A MENUITEM SEPARATOR is kept as the empty ID 0 item
//!   that wrc 11.19 compiles it to, so the port matches the shipped resource.
//! - The ERROR.HTM HTML resource: html_resource and ERROR_HTM.
//!
//! Skipped:
//! - The #include lines and the #pragma makedep po line. They drive the build, and the IDs in
//!   shdoclc.h are ported as constants.
//! - The comments, including the FIXME note on the IDS_PRINT_HEADER_TEMPLATE line.
//! - The #msgctxt#do not translate# copy of the footer string. wrc writes it at Version 0, and
//!   winebuild keeps the Version 1 copy, so only that text is ported.
//! - The copyright header, and the @makedep comment for ERROR.HTM, which only feeds the build.

mod html;
mod language;
mod menu;
mod strings;

pub use html::{html_resource, ERROR_HTM};
pub use language::{makelangid, LANG_ENGLISH, LANG_ID, SUBLANG_DEFAULT};
pub use menu::{
    menu_resource, MenuItem, _IDM_REFRESH, BROWSE_CONTEXT_MENU, IDM_ADDFAVORITES, IDM_BROWSEMODE,
    IDM_CELLPROPERTIES, IDM_CELLSELECT, IDM_COLUMNSELECT, IDM_COPY, IDM_COPYBACKGROUND,
    IDM_COPYSHORTCUT, IDM_CREATESHORTCUT, IDM_CUT, IDM_DEBUGBREAK, IDM_DEBUGVIEW, IDM_DELETE,
    IDM_DUMPDISPLAYTREE, IDM_DUMPFORMATCACHES, IDM_DUMPLAYOUTRECTS, IDM_DUMPLINES,
    IDM_DUMPTRACKINGINFO, IDM_DUMPTREE, IDM_DYNSRCPLAY, IDM_DYNSRCSTOP, IDM_EDITMODE,
    IDM_FOLLOWLINKC, IDM_FOLLOWLINKN, IDM_GOBACKWARD, IDM_GOFORWARD, IDM_IMGARTPLAY,
    IDM_IMGARTREWIND, IDM_IMGARTSTOP, IDM_LANGUAGE, IDM_MEMORYMONITOR, IDM_MP_EMAILPICTURE,
    IDM_MP_MYPICS, IDM_MP_PRINTPICTURE, IDM_PASTE, IDM_PERFORMANCEMETERS, IDM_PRINT,
    IDM_PRINTTARGET, IDM_PROPERTIES, IDM_RESOURCEFAILURES, IDM_ROWSELECT, IDM_SAVEBACKGROUND,
    IDM_SAVEHTML, IDM_SAVEPICTURE, IDM_SAVETARGET, IDM_SCROLL_BOTTOM, IDM_SCROLL_DOWN,
    IDM_SCROLL_HERE, IDM_SCROLL_LEFT, IDM_SCROLL_LEFTEDGE, IDM_SCROLL_PAGEDOWN,
    IDM_SCROLL_PAGELEFT, IDM_SCROLL_PAGERIGHT, IDM_SCROLL_PAGEUP, IDM_SCROLL_RIGHT,
    IDM_SCROLL_RIGHTEDGE, IDM_SCROLL_TOP, IDM_SCROLL_UP, IDM_SELECTALL, IDM_SETDESKTOPITEM,
    IDM_SETWALLPAPER, IDM_SHOWPICTURE, IDM_TABLEPROPERTIES, IDM_TABLESELECT, IDM_TRACETAGS,
    IDM_UNDO, IDM_VIEWSOURCE, IDR_BROWSE_CONTEXT_MENU,
};
pub use strings::{
    string_table_entry, IDS_MESSAGE_BOX_TITLE, IDS_PRINT_FOOTER_TEMPLATE, IDS_PRINT_HEADER_TEMPLATE,
};

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
