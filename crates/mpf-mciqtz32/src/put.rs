//! The rectangle of `MCIQTZ_mciPut`: the window position it sets and the notify it asks for.

use crate::abi::{MCI_DGV_PUT_DESTINATION, MCI_DGV_RECT, MCI_NOTIFY, MCI_TEST};

/// `MCI_DGV_RECT_PARMS.rc`, with left, top, right and bottom edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// The window position that `IVideoWindow::SetWindowPosition` takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

/// What `MCIQTZ_mciPut` does for a request. `reply` is its return value. `position` is set when
/// the caller must move the video window, and `notify` when it must send the notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PutPlan {
    pub reply: u32,
    pub position: Option<Position>,
    pub notify: bool,
}

/// Plans `MCI_PUT`. Only the rectangle form is supported, and `MCI_TEST` changes nothing. The
/// window position is set only for `MCI_DGV_PUT_DESTINATION`. The width and height wrap, where
/// Wine's `LONG` subtraction would overflow.
pub fn plan_put(flags: u32, rc: Rect) -> PutPlan {
    if flags & MCI_DGV_RECT == 0 {
        return PutPlan {
            reply: 1,
            position: None,
            notify: false,
        };
    }
    if flags & MCI_TEST != 0 {
        return PutPlan {
            reply: 0,
            position: None,
            notify: false,
        };
    }
    let position = if flags & MCI_DGV_PUT_DESTINATION != 0 {
        Some(Position {
            left: rc.left,
            top: rc.top,
            width: rc.right.wrapping_sub(rc.left),
            height: rc.bottom.wrapping_sub(rc.top),
        })
    } else {
        None
    };
    PutPlan {
        reply: 0,
        position,
        notify: flags & MCI_NOTIFY != 0,
    }
}
