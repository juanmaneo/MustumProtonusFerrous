/// One item of a MENU resource, as the compiled resource stores it.
#[derive(Debug, PartialEq, Eq)]
pub enum MenuItem {
    /// A MENUITEM with its text and command ID. A MENUITEM SEPARATOR compiles to one of these with
    /// empty text and ID 0, because wrc 11.19 (parser.y) does not set MF_SEPARATOR on it.
    Command { text: &'static str, id: u32 },
    /// A POPUP with its text and the items it holds.
    Popup {
        text: &'static str,
        items: &'static [MenuItem],
    },
}

impl MenuItem {
    /// The items that a POPUP holds. A command holds none.
    pub fn items(&self) -> &'static [MenuItem] {
        match self {
            MenuItem::Popup { items, .. } => items,
            MenuItem::Command { .. } => &[],
        }
    }
}

// Command, menu and resource IDs of shdoclc.h, which includes mshtmcid.h.
pub const IDM_ADDFAVORITES: u32 = 2261;
pub const IDM_BROWSEMODE: u32 = 2126;
pub const IDM_CELLPROPERTIES: u32 = 2211;
pub const IDM_CELLSELECT: u32 = 2206;
pub const IDM_COLUMNSELECT: u32 = 2208;
pub const IDM_COPY: u32 = 15;
pub const IDM_COPYBACKGROUND: u32 = 2265;
pub const IDM_COPYSHORTCUT: u32 = 2262;
pub const IDM_CREATESHORTCUT: u32 = 2266;
pub const IDM_CUT: u32 = 16;
pub const IDM_DEBUGBREAK: u32 = 6007;
pub const IDM_DEBUGVIEW: u32 = 6008;
pub const IDM_DELETE: u32 = 17;
pub const IDM_DUMPDISPLAYTREE: u32 = 6015;
pub const IDM_DUMPFORMATCACHES: u32 = 6016;
pub const IDM_DUMPLAYOUTRECTS: u32 = 6017;
pub const IDM_DUMPLINES: u32 = 6010;
pub const IDM_DUMPTRACKINGINFO: u32 = 6006;
pub const IDM_DUMPTREE: u32 = 6009;
pub const IDM_DYNSRCPLAY: u32 = 2271;
pub const IDM_DYNSRCSTOP: u32 = 2272;
pub const IDM_EDITMODE: u32 = 2127;
pub const IDM_FOLLOWLINKC: u32 = 2136;
pub const IDM_FOLLOWLINKN: u32 = 2137;
pub const IDM_GOBACKWARD: u32 = 2282;
pub const IDM_GOFORWARD: u32 = 2283;
pub const IDM_IMGARTPLAY: u32 = 2274;
pub const IDM_IMGARTREWIND: u32 = 2276;
pub const IDM_IMGARTSTOP: u32 = 2275;
pub const IDM_LANGUAGE: u32 = 2292;
pub const IDM_MEMORYMONITOR: u32 = 6013;
pub const IDM_MP_EMAILPICTURE: u32 = 2288;
pub const IDM_MP_MYPICS: u32 = 2287;
pub const IDM_MP_PRINTPICTURE: u32 = 2289;
pub const IDM_PASTE: u32 = 26;
pub const IDM_PERFORMANCEMETERS: u32 = 6014;
pub const IDM_PRINT: u32 = 27;
pub const IDM_PRINTTARGET: u32 = 2273;
pub const IDM_PROPERTIES: u32 = 28;
pub const IDM_RESOURCEFAILURES: u32 = 6005;
pub const IDM_ROWSELECT: u32 = 2207;
pub const IDM_SAVEBACKGROUND: u32 = 2263;
pub const IDM_SAVEHTML: u32 = 6012;
pub const IDM_SAVEPICTURE: u32 = 2270;
pub const IDM_SAVETARGET: u32 = 2268;
pub const IDM_SCROLL_BOTTOM: u32 = 2382;
pub const IDM_SCROLL_DOWN: u32 = 2386;
pub const IDM_SCROLL_HERE: u32 = 2380;
pub const IDM_SCROLL_LEFT: u32 = 2391;
pub const IDM_SCROLL_LEFTEDGE: u32 = 2387;
pub const IDM_SCROLL_PAGEDOWN: u32 = 2384;
pub const IDM_SCROLL_PAGELEFT: u32 = 2389;
pub const IDM_SCROLL_PAGERIGHT: u32 = 2390;
pub const IDM_SCROLL_PAGEUP: u32 = 2383;
pub const IDM_SCROLL_RIGHT: u32 = 2392;
pub const IDM_SCROLL_RIGHTEDGE: u32 = 2388;
pub const IDM_SCROLL_TOP: u32 = 2381;
pub const IDM_SCROLL_UP: u32 = 2385;
pub const IDM_SELECTALL: u32 = 31;
pub const IDM_SETDESKTOPITEM: u32 = 2278;
pub const IDM_SETWALLPAPER: u32 = 2264;
pub const IDM_SHOWPICTURE: u32 = 2269;
pub const IDM_TABLEPROPERTIES: u32 = 2210;
pub const IDM_TABLESELECT: u32 = 2209;
pub const IDM_TRACETAGS: u32 = 6004;
pub const IDM_UNDO: u32 = 43;
pub const IDM_VIEWSOURCE: u32 = 2139;
pub const IDR_BROWSE_CONTEXT_MENU: u32 = 24641;
pub const _IDM_REFRESH: u32 = 6042;

/// The IDR_BROWSE_CONTEXT_MENU MENU of shdoclc.rc, with its twelve top-level POPUPs in .rc order.
pub const BROWSE_CONTEXT_MENU: &[MenuItem] = &[
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Back",
                id: IDM_GOBACKWARD,
            },
            MenuItem::Command {
                text: "F&orward",
                id: IDM_GOFORWARD,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "&Save Background As...",
                id: IDM_SAVEBACKGROUND,
            },
            MenuItem::Command {
                text: "Set As Back&ground",
                id: IDM_SETWALLPAPER,
            },
            MenuItem::Command {
                text: "&Copy Background",
                id: IDM_COPYBACKGROUND,
            },
            MenuItem::Command {
                text: "Set as &Desktop Item",
                id: IDM_SETDESKTOPITEM,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Select &All",
                id: IDM_SELECTALL,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Create Shor&tcut",
                id: IDM_CREATESHORTCUT,
            },
            MenuItem::Command {
                text: "Add to &Favorites...",
                id: IDM_ADDFAVORITES,
            },
            MenuItem::Command {
                text: "&View Source",
                id: IDM_VIEWSOURCE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "&Encoding",
                id: IDM_LANGUAGE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Pr&int",
                id: IDM_PRINT,
            },
            MenuItem::Command {
                text: "&Refresh",
                id: _IDM_REFRESH,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "&Properties",
                id: IDM_PROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Open Link",
                id: IDM_FOLLOWLINKC,
            },
            MenuItem::Command {
                text: "Open Link in &New Window",
                id: IDM_FOLLOWLINKN,
            },
            MenuItem::Command {
                text: "Save Target &As...",
                id: IDM_SAVETARGET,
            },
            MenuItem::Command {
                text: "&Print Target",
                id: IDM_PRINTTARGET,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "S&how Picture",
                id: IDM_SHOWPICTURE,
            },
            MenuItem::Command {
                text: "&Save Picture As...",
                id: IDM_SAVEPICTURE,
            },
            MenuItem::Command {
                text: "&E-mail Picture...",
                id: IDM_MP_EMAILPICTURE,
            },
            MenuItem::Command {
                text: "Pr&int Picture...",
                id: IDM_MP_PRINTPICTURE,
            },
            MenuItem::Command {
                text: "&Go to My Pictures",
                id: IDM_MP_MYPICS,
            },
            MenuItem::Command {
                text: "Set as Back&ground",
                id: IDM_SETWALLPAPER,
            },
            MenuItem::Command {
                text: "Set as &Desktop Item...",
                id: IDM_SETDESKTOPITEM,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Cu&t",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "Copy Shor&tcut",
                id: IDM_COPYSHORTCUT,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Add to &Favorites...",
                id: IDM_ADDFAVORITES,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "P&roperties",
                id: IDM_PROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Undo",
                id: IDM_UNDO,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Cu&t",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command {
                text: "&Delete",
                id: IDM_DELETE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Select &All",
                id: IDM_SELECTALL,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Popup {
                text: "&Select",
                items: &[
                    MenuItem::Command {
                        text: "&Cell",
                        id: IDM_CELLSELECT,
                    },
                    MenuItem::Command {
                        text: "&Row",
                        id: IDM_ROWSELECT,
                    },
                    MenuItem::Command {
                        text: "&Column",
                        id: IDM_COLUMNSELECT,
                    },
                    MenuItem::Command {
                        text: "&Table",
                        id: IDM_TABLESELECT,
                    },
                ],
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "&Cell Properties",
                id: IDM_CELLPROPERTIES,
            },
            MenuItem::Command {
                text: "&Table Properties",
                id: IDM_TABLEPROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "Cu&t",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command {
                text: "Select &All",
                id: IDM_SELECTALL,
            },
            MenuItem::Command {
                text: "&Print",
                id: IDM_PRINT,
            },
            MenuItem::Command { text: "", id: 0 },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Open",
                id: IDM_FOLLOWLINKC,
            },
            MenuItem::Command {
                text: "Open in &New Window",
                id: IDM_FOLLOWLINKN,
            },
            MenuItem::Command {
                text: "Save Target &As...",
                id: IDM_SAVETARGET,
            },
            MenuItem::Command {
                text: "&Print Target",
                id: IDM_PRINTTARGET,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Cut",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "Copy Shor&tcut",
                id: IDM_COPYSHORTCUT,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Add to &Favorites...",
                id: IDM_ADDFAVORITES,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "P&roperties",
                id: IDM_PROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[MenuItem::Command { text: "", id: 0 }],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Open Link",
                id: IDM_FOLLOWLINKC,
            },
            MenuItem::Command {
                text: "Open Link in &New Window",
                id: IDM_FOLLOWLINKN,
            },
            MenuItem::Command {
                text: "Save Target &As...",
                id: IDM_SAVETARGET,
            },
            MenuItem::Command {
                text: "&Print Target",
                id: IDM_PRINTTARGET,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "S&how Picture",
                id: IDM_SHOWPICTURE,
            },
            MenuItem::Command {
                text: "&Save Video As...",
                id: IDM_SAVEPICTURE,
            },
            MenuItem::Command {
                text: "Set as Back&ground",
                id: IDM_SETWALLPAPER,
            },
            MenuItem::Command {
                text: "Set as &Desktop Item...",
                id: IDM_SETDESKTOPITEM,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Cu&t",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "Copy Shor&tcut",
                id: IDM_COPYSHORTCUT,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Add to &Favorites...",
                id: IDM_ADDFAVORITES,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Play",
                id: IDM_DYNSRCPLAY,
            },
            MenuItem::Command {
                text: "Stop",
                id: IDM_DYNSRCSTOP,
            },
            MenuItem::Command {
                text: "P&roperties",
                id: IDM_PROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "&Open Link",
                id: IDM_FOLLOWLINKC,
            },
            MenuItem::Command {
                text: "Open Link in &New Window",
                id: IDM_FOLLOWLINKN,
            },
            MenuItem::Command {
                text: "Save Target &As...",
                id: IDM_SAVETARGET,
            },
            MenuItem::Command {
                text: "&Print Target",
                id: IDM_PRINTTARGET,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "S&how Picture",
                id: IDM_SHOWPICTURE,
            },
            MenuItem::Command {
                text: "&Save Picture As...",
                id: IDM_SAVEPICTURE,
            },
            MenuItem::Command {
                text: "Set as Back&ground",
                id: IDM_SETWALLPAPER,
            },
            MenuItem::Command {
                text: "Set as &Desktop Item...",
                id: IDM_SETDESKTOPITEM,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Cu&t",
                id: IDM_CUT,
            },
            MenuItem::Command {
                text: "&Copy",
                id: IDM_COPY,
            },
            MenuItem::Command {
                text: "Copy Shor&tcut",
                id: IDM_COPYSHORTCUT,
            },
            MenuItem::Command {
                text: "&Paste",
                id: IDM_PASTE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Add to &Favorites...",
                id: IDM_ADDFAVORITES,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Play",
                id: IDM_IMGARTPLAY,
            },
            MenuItem::Command {
                text: "Stop",
                id: IDM_IMGARTSTOP,
            },
            MenuItem::Command {
                text: "Rewind",
                id: IDM_IMGARTREWIND,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "P&roperties",
                id: IDM_PROPERTIES,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "Trace Tags",
                id: IDM_TRACETAGS,
            },
            MenuItem::Command {
                text: "Resource Failures",
                id: IDM_RESOURCEFAILURES,
            },
            MenuItem::Command {
                text: "Dump Tracking Info",
                id: IDM_DUMPTRACKINGINFO,
            },
            MenuItem::Command {
                text: "Debug Break",
                id: IDM_DEBUGBREAK,
            },
            MenuItem::Command {
                text: "Debug View",
                id: IDM_DEBUGVIEW,
            },
            MenuItem::Command {
                text: "Dump Tree",
                id: IDM_DUMPTREE,
            },
            MenuItem::Command {
                text: "Dump Lines",
                id: IDM_DUMPLINES,
            },
            MenuItem::Command {
                text: "Dump DisplayTree",
                id: IDM_DUMPDISPLAYTREE,
            },
            MenuItem::Command {
                text: "Dump FormatCaches",
                id: IDM_DUMPFORMATCACHES,
            },
            MenuItem::Command {
                text: "Dump LayoutRects",
                id: IDM_DUMPLAYOUTRECTS,
            },
            MenuItem::Command {
                text: "Memory Monitor",
                id: IDM_MEMORYMONITOR,
            },
            MenuItem::Command {
                text: "Performance Meters",
                id: IDM_PERFORMANCEMETERS,
            },
            MenuItem::Command {
                text: "Save HTML",
                id: IDM_SAVEHTML,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "&Browse View",
                id: IDM_BROWSEMODE,
            },
            MenuItem::Command {
                text: "&Edit View",
                id: IDM_EDITMODE,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "Scroll Here",
                id: IDM_SCROLL_HERE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Top",
                id: IDM_SCROLL_TOP,
            },
            MenuItem::Command {
                text: "Bottom",
                id: IDM_SCROLL_BOTTOM,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Page Up",
                id: IDM_SCROLL_PAGEUP,
            },
            MenuItem::Command {
                text: "Page Down",
                id: IDM_SCROLL_PAGEDOWN,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Scroll Up",
                id: IDM_SCROLL_UP,
            },
            MenuItem::Command {
                text: "Scroll Down",
                id: IDM_SCROLL_DOWN,
            },
        ],
    },
    MenuItem::Popup {
        text: "",
        items: &[
            MenuItem::Command {
                text: "Scroll Here",
                id: IDM_SCROLL_HERE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Left Edge",
                id: IDM_SCROLL_LEFTEDGE,
            },
            MenuItem::Command {
                text: "Right Edge",
                id: IDM_SCROLL_RIGHTEDGE,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Page Left",
                id: IDM_SCROLL_PAGELEFT,
            },
            MenuItem::Command {
                text: "Page Right",
                id: IDM_SCROLL_PAGERIGHT,
            },
            MenuItem::Command { text: "", id: 0 },
            MenuItem::Command {
                text: "Scroll Left",
                id: IDM_SCROLL_LEFT,
            },
            MenuItem::Command {
                text: "Scroll Right",
                id: IDM_SCROLL_RIGHT,
            },
        ],
    },
];

/// The MENU resource with the given ID, or None when shdoclc.rc defines no MENU with that ID.
pub fn menu_resource(id: u32) -> Option<&'static [MenuItem]> {
    (id == IDR_BROWSE_CONTEXT_MENU).then_some(BROWSE_CONTEXT_MENU)
}
