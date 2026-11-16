//! user32 subset, mirroring `dlls/user32/` in reference Wine — headless:
//! windows are in-memory records (no X11/Wayland driver yet; that lands
//! with the graphics phase's native dxvk companion).

use mpf_ntdll::object::ObjectManager;
use std::collections::HashMap;
use std::sync::Mutex;

/// Window handle (a value from the object table; 0 = NULL).
pub type Hwnd = u64;

pub const IDOK: i32 = 1;
pub const MB_OK: u32 = 0;
pub const MB_ICONERROR: u32 = 0x10;
pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;

/// SM_* indices we answer (see winnt/user32 GetSystemMetrics).
pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;
pub const SM_CMOUSEBUTTONS: i32 = 43;

/// A window class atom (like ATOM, 0xC000+ on Windows).
pub type Atom = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone)]
struct Window {
    class: String,
    title: String,
    style: u32,
    rect: Rect,
}

struct User32State {
    om: ObjectManager,
    atoms: HashMap<String, Atom>,
    next_atom: u16,
    windows: HashMap<Hwnd, Window>,
    desktop: Hwnd,
}

static STATE: Mutex<Option<User32State>> = Mutex::new(None);

fn with_state<R>(f: impl FnOnce(&mut User32State) -> R) -> R {
    let mut guard = STATE.lock().unwrap();
    let state = guard.get_or_insert_with(|| {
        let mut om = ObjectManager::new();
        let desktop = om.allocate_handle(2);
        User32State {
            om,
            atoms: HashMap::new(),
            next_atom: 0xC000,
            windows: HashMap::from([(
                desktop,
                Window {
                    class: "#Desktop".into(),
                    title: String::new(),
                    style: 0,
                    rect: Rect { left: 0, top: 0, right: 1920, bottom: 1080 },
                },
            )]),
            desktop,
        }
    });
    f(state)
}

fn wide_to_string(s: &[u16]) -> String {
    let end = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    s[..end]
        .iter()
        .filter_map(|&c| char::from_u32(u32::from(c)))
        .collect()
}

/// RegisterClassW: registers a class name, returns its atom (0 on failure).
pub fn register_class_w(name: &[u16]) -> Atom {
    let name = wide_to_string(name);
    if name.is_empty() {
        return 0;
    }
    with_state(|st| {
        if let Some(atom) = st.atoms.get(&name) {
            return *atom;
        }
        let atom = st.next_atom;
        st.next_atom = st.next_atom.wrapping_add(1);
        st.atoms.insert(name, atom);
        atom
    })
}

/// CreateWindowExW: creates an in-memory window; returns its Hwnd (0 on
/// failure, e.g. empty class and unknown atom).
pub fn create_window_ex_w(
    class: &str,
    title: &[u16],
    style: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> Hwnd {
    let title = wide_to_string(title);
    if class.is_empty() {
        return 0;
    }
    with_state(|st| {
        if !st.atoms.contains_key(class) && !class.starts_with('#') {
            return 0;
        }
        let hwnd = st.om.allocate_handle(3);
        st.windows.insert(
            hwnd,
            Window {
                class: class.to_string(),
                title,
                style,
                rect: Rect { left: x, top: y, right: x + w, bottom: y + h },
            },
        );
        hwnd
    })
}

/// GetDesktopWindow.
pub fn get_desktop_window() -> Hwnd {
    with_state(|st| st.desktop)
}

/// DefWindowProcW: headless default proc returns 0.
pub fn def_window_proc_w(_hwnd: Hwnd, _msg: u32, _wparam: usize, _lparam: isize) -> isize {
    0
}

/// MessageBoxW: headless — logs and returns IDOK.
pub fn message_box_w(owner: Hwnd, text: &[u16], caption: &[u16], msg_type: u32) -> i32 {
    let text = wide_to_string(text);
    let caption = wide_to_string(caption);
    eprintln!("[mpf-user32] MessageBox (owner={owner:#x}, type={msg_type:#x}) {caption}: {text}");
    IDOK
}

/// GetSystemMetrics for the indices we know; 0 otherwise.
pub fn get_system_metrics(index: i32) -> i32 {
    match index {
        SM_CXSCREEN => 1920,
        SM_CYSCREEN => 1080,
        SM_CMOUSEBUTTONS => 2,
        _ => 0,
    }
}

/// GetClientRect.
pub fn get_client_rect(hwnd: Hwnd) -> Option<Rect> {
    with_state(|st| st.windows.get(&hwnd).map(|w| w.rect))
}

/// Queries a window's class, title and style.
pub fn window_info(hwnd: Hwnd) -> Option<(String, String, u32)> {
    with_state(|st| {
        st.windows
            .get(&hwnd)
            .map(|w| (w.class.clone(), w.title.clone(), w.style))
    })
}

/// DestroyWindow.
pub fn destroy_window(hwnd: Hwnd) -> bool {
    with_state(|st| {
        if hwnd == st.desktop {
            return false;
        }
        match st.windows.remove(&hwnd) {
            Some(_) => {
                st.om.close_handle(hwnd);
                true
            }
            None => false,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    #[test]
    fn class_atoms_are_stable_and_unique() {
        let a1 = register_class_w(&w("MyClass"));
        let a2 = register_class_w(&w("MyClass"));
        let b = register_class_w(&w("Other"));
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
        assert!(a1 >= 0xC000);
        assert_eq!(register_class_w(&w("")), 0);
    }

    #[test]
    fn window_lifecycle() {
        register_class_w(&w("TestClass"));
        let hwnd = create_window_ex_w("TestClass", &w("hello"), WS_OVERLAPPEDWINDOW, 10, 20, 300, 200);
        assert_ne!(hwnd, 0);
        let rect = get_client_rect(hwnd).unwrap();
        assert_eq!(rect, Rect { left: 10, top: 20, right: 310, bottom: 220 });
        assert!(destroy_window(hwnd));
        assert_eq!(get_client_rect(hwnd), None);
        assert!(!destroy_window(hwnd));
    }

    #[test]
    fn window_info_reports_class_title_style() {
        register_class_w(&w("InfoClass"));
        let hwnd = create_window_ex_w("InfoClass", &w("title"), WS_OVERLAPPEDWINDOW, 0, 0, 1, 1);
        let (class, title, style) = window_info(hwnd).unwrap();
        assert_eq!(class, "InfoClass");
        assert_eq!(title, "title");
        assert_eq!(style, WS_OVERLAPPEDWINDOW);
        assert_eq!(window_info(9999), None);
        assert!(destroy_window(hwnd));
    }

    #[test]
    fn window_creation_failures() {
        assert_eq!(create_window_ex_w("", &w("x"), 0, 0, 0, 1, 1), 0);
        assert_eq!(create_window_ex_w("NoSuchClass", &w("x"), 0, 0, 0, 1, 1), 0);
    }

    #[test]
    fn desktop_window_and_metrics_and_proc() {
        let desktop = get_desktop_window();
        assert_ne!(desktop, 0);
        assert!(!destroy_window(desktop));
        assert_eq!(get_system_metrics(SM_CXSCREEN), 1920);
        assert_eq!(get_system_metrics(SM_CYSCREEN), 1080);
        assert_eq!(get_system_metrics(SM_CMOUSEBUTTONS), 2);
        assert_eq!(get_system_metrics(9999), 0);
        assert_eq!(def_window_proc_w(desktop, 0x10, 0, 0), 0);
        assert_eq!(message_box_w(desktop, &w("hi"), &w("cap"), MB_OK), IDOK);
        assert_eq!(message_box_w(0, &w("x"), &w("y"), MB_ICONERROR), IDOK);
    }
}
