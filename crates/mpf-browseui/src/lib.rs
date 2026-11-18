//! Port of the portable logic in Wine's `dlls/browseui/progressdlg.c` and `dlls/browseui/aclmulti.c`.
//!
//! The progress dialog keeps update flags, three text lines, a cancel message, progress counters
//! and a start time. Choosing what to paint, scaling 64-bit progress into the progress bar's 32-bit
//! range, and estimating the time remaining are plain arithmetic over those values. The window, the
//! dialog thread, the resource strings and `FormatMessageW` belong to the caller.
//!
//! `aclmulti.c` chains several `IEnumString` and `IACList` sources. Its `Next`, `Reset` and `Expand`
//! walk the sources in order. [`AclMulti`] keeps that walk, with the sources behind the
//! [`StringEnum`] and [`AclExpand`] traits.
//!
//! Ported:
//! - `update_dialog`, the paint choice: [`Dialog::paint`], with [`scale_progress`] for the bar.
//! - `update_time_remaining`: [`time_remaining`] for the estimate, and [`Dialog::set_remaining_line`] for the line it sets.
//! - `ProgressDialog_SetProgress64`: [`Dialog::set_progress`].
//! - `ProgressDialog_SetLine` and `ProgressDialog_SetCancelMsg`: [`Dialog::set_line`] and [`Dialog::set_cancel_msg`].
//! - `ACLMulti_Next`, `ACLMulti_Reset` and `ACLMulti_Expand`: [`AclMulti::next_string`], [`AclMulti::reset`] and [`AclMulti::expand`].
//!
//! Skipped:
//! - `ProgressDialog_StartProgressDialog`, `dialog_thread` and `dialog_proc`: threads, events, windows and `EnableWindow`.
//! - `load_string` and `load_time_strings`: resources. `set_progress_marquee`: window styles.
//! - `ProgressDialog_SetTitle`, `SetAnimation` and `Timer`: window work or `FIXME` stubs.
//! - `ProgressDialog_SetProgress` (32-bit): forwards to the 64-bit call. The caller widens the values.
//! - `ProgressDialog_HasUserCancelled`, the `OleWindow` methods, `QueryInterface`, `AddRef`, `Release` and the destructors: COM and heap.
//! - `ACLMulti_Append` and `ACLMulti_Remove`: `QueryInterface` on the objects, `realloc` and `memmove`.
//! - `ACLMulti_Skip` and `ACLMulti_Clone`: fixed return values, with no state to walk.
//! - `aclsource.c`: every method is a `FIXME` stub or stores an option.
//! - `compcatcachedaemon.c`: `IRunnableTask` runs on its own thread.
//! - `browseui_main.c`: `DllMain`, the class factory, `DllGetClassObject` and `DllInstall`.
//!
//! Divergences from C:
//! - Wide strings are `String`. The code copies and compares text and never measures wide characters.
//! - `set_buffer(NULL)` stores an empty string, as in C.
//! - `timeMsg[i - 1]` with `i == 0` is undefined behaviour in C (`timeMsg[-1]`), so it is not reproduced. [`TimeRemaining::unit`] gives the unit index, and a caller reads unit `- 1` only when `unit > 0`.
//! - `UPDATE_LINE3` is `UPDATE_LINE2 << 2` (0x20) in Wine, but `SetLine(3)` and `SetCancelMsg` set `UPDATE_LINE1 << 2` (0x10). The port keeps both values, so line 3 and the cancel message are painted only when another path sets 0x20. Wine's `ProgressDialog_SetLine` is not fixed here.

/// `S_OK`.
pub const S_OK: u32 = 0;
/// `S_FALSE`.
pub const S_FALSE: u32 = 1;
/// `E_FAIL`.
pub const E_FAIL: u32 = 0x8000_4005;
/// `E_NOTIMPL`.
pub const E_NOTIMPL: u32 = 0x8000_4001;
/// `E_OUTOFMEMORY`.
pub const E_OUTOFMEMORY: u32 = 0x8007_000E;

/// `UPDATE_PROGRESS`.
pub const UPDATE_PROGRESS: u32 = 0x1;
/// `UPDATE_TITLE`.
pub const UPDATE_TITLE: u32 = 0x2;
/// `UPDATE_LINE1`.
pub const UPDATE_LINE1: u32 = 0x4;
/// `UPDATE_LINE2`.
pub const UPDATE_LINE2: u32 = UPDATE_LINE1 << 1;
/// `UPDATE_LINE3`, defined in Wine as `UPDATE_LINE2 << 2` (0x20). See the divergences above.
pub const UPDATE_LINE3: u32 = UPDATE_LINE2 << 2;

/// `CANCEL_MSG_LINE`.
const CANCEL_MSG_LINE: u32 = 2;
/// The flag that `update_dialog` checks for each of the three lines.
const LINE_FLAGS: [u32; 3] = [UPDATE_LINE1, UPDATE_LINE2, UPDATE_LINE3];

/// The range and position that `update_dialog` sends to the progress bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressBar {
    /// `PBM_SETRANGE32` total.
    pub range: u32,
    /// `PBM_SETPOS` position.
    pub pos: u32,
}

/// The 64-bit to 32-bit scaling of `update_dialog`: both values are halved, as long as the total
/// does not fit in 32 bits.
pub fn scale_progress(completed: u64, total: u64) -> ProgressBar {
    let (mut completed, mut total) = (completed, total);
    while total >> 32 != 0 {
        total >>= 1;
        completed >>= 1;
    }
    ProgressBar {
        range: total as u32,
        pos: completed as u32,
    }
}

/// The arguments of the time-remaining message, as `update_time_remaining` builds them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeRemaining {
    /// The count in the largest unit reached (seconds, minutes or hours).
    pub remaining: u32,
    /// Index of that unit in the `timeMsg` strings: 0 seconds, 1 minutes, 2 hours.
    pub unit: usize,
    /// The count left over in the unit below, or 0 when no division ran.
    pub remainder: u32,
    /// True when Wine formats with `remainingMsg[1]` instead of `remainingMsg[0]`.
    pub long_form: bool,
}

/// The estimate of `update_time_remaining`: the seconds left at the current rate, split into hours,
/// minutes and seconds. `now` stands for `GetTickCount64()`. Unsigned overflow wraps, as in the C
/// arithmetic.
///
/// Returns `None` when Wine does nothing: the dialog has no start time, or no progress yet.
pub fn time_remaining(
    start_time: u64,
    now: u64,
    completed: u64,
    total: u64,
) -> Option<TimeRemaining> {
    if start_time == 0 || completed == 0 || total == 0 {
        return None;
    }
    let elapsed = now.wrapping_sub(start_time);
    let seconds = (elapsed.wrapping_mul(total) / completed).wrapping_sub(elapsed) / 1000;
    let mut remaining = seconds as u32;
    let mut remainder = 0;
    let mut unit = 0;
    while remaining >= 60 && unit < 2 {
        remainder = remaining % 60;
        remaining /= 60;
        unit += 1;
    }
    Some(TimeRemaining {
        remaining,
        unit,
        remainder,
        long_form: unit > 0 && remaining < 2 && remainder != 0,
    })
}

/// The text slot of `SetLine`'s 1-based line number. Wine decrements the number, and any value
/// outside slots 0 to 2 falls back to slot 0.
pub fn line_slot(line_num: u32) -> usize {
    let index = line_num.wrapping_sub(1);
    if index < 3 {
        index as usize
    } else {
        0
    }
}

/// The state that `ProgressDialog` keeps between calls.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Dialog {
    /// The `dwUpdate` flags: what the next paint refreshes.
    pub update: u32,
    /// The window title.
    pub title: String,
    /// The three text lines. Line 3 also carries the time remaining.
    pub lines: [String; 3],
    /// The text that line 3 shows after the user cancels.
    pub cancel_msg: String,
    /// Set once the user cancels.
    pub is_cancelled: bool,
    /// Completed work, as last set by `set_progress`.
    pub completed: u64,
    /// Total work, as last set by `set_progress`.
    pub total: u64,
    /// `PROGDLG_AUTOTIME` was passed to `StartProgressDialog`.
    pub autotime: bool,
    /// The `GetTickCount64` value when the dialog started. 0 before then.
    pub start_time: u64,
}

/// What `update_dialog` paints: each part whose flag is set, with the text it shows.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paint<'a> {
    /// The title, when `UPDATE_TITLE` is set.
    pub title: Option<&'a str>,
    /// Text of lines 1 to 3, each when its flag is set.
    pub lines: [Option<&'a str>; 3],
    /// The progress bar, when `UPDATE_PROGRESS` is set.
    pub progress: Option<ProgressBar>,
}

impl Dialog {
    /// `ProgressDialog_SetLine` on the stored state. A `None` text is stored as an empty string.
    /// Returns whether the caller should send `WM_DLG_UPDATE`: that is the case when a window
    /// exists and the dialog is not cancelled.
    pub fn set_line(&mut self, line_num: u32, text: Option<&str>) -> bool {
        let slot = line_slot(line_num);
        self.lines[slot] = text.unwrap_or_default().to_string();
        self.update |= UPDATE_LINE1 << slot;
        !self.is_cancelled
    }

    /// `ProgressDialog_SetCancelMsg` on the stored state. Returns whether the caller should send
    /// `WM_DLG_UPDATE`: that is the case when a window exists and the dialog is cancelled.
    pub fn set_cancel_msg(&mut self, text: Option<&str>) -> bool {
        self.cancel_msg = text.unwrap_or_default().to_string();
        self.update |= UPDATE_LINE1 << CANCEL_MSG_LINE;
        self.is_cancelled
    }

    /// `ProgressDialog_SetProgress64`. Returns the estimate when the dialog was started with
    /// `PROGDLG_AUTOTIME`. The caller formats the estimate and passes the text to
    /// [`Dialog::set_remaining_line`].
    pub fn set_progress(&mut self, completed: u64, total: u64, now: u64) -> Option<TimeRemaining> {
        self.total = total;
        self.completed = completed;
        self.update |= UPDATE_PROGRESS;
        if self.autotime {
            time_remaining(self.start_time, now, completed, total)
        } else {
            None
        }
    }

    /// The line-3 part of `update_time_remaining`: stores the formatted estimate and flags line 3.
    pub fn set_remaining_line(&mut self, text: &str) {
        self.lines[2] = text.to_string();
        self.update |= UPDATE_LINE3;
    }

    fn shown_line(&self, slot: usize) -> &str {
        match (slot, self.is_cancelled) {
            (2, true) => &self.cancel_msg,
            (_, true) => "",
            _ => &self.lines[slot],
        }
    }

    /// `update_dialog`: the parts to paint, with their text. A cancelled dialog shows an empty
    /// line 1 and 2, and the cancel message on line 3.
    pub fn paint(&self) -> Paint<'_> {
        let line =
            |slot: usize| ((self.update & LINE_FLAGS[slot]) != 0).then(|| self.shown_line(slot));
        Paint {
            title: ((self.update & UPDATE_TITLE) != 0).then_some(self.title.as_str()),
            lines: [line(0), line(1), line(2)],
            progress: ((self.update & UPDATE_PROGRESS) != 0)
                .then(|| scale_progress(self.completed, self.total)),
        }
    }
}

/// The result of one `IEnumString::Next(1, ...)` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fetch<T> {
    /// S_OK with one element.
    Item(T),
    /// S_FALSE: the enumerator is exhausted.
    Done,
    /// Any other HRESULT, which `ACLMulti_Next` returns unchanged. Never S_FALSE.
    Failed(u32),
}

/// An `IEnumString` source.
pub trait StringEnum {
    /// The element type.
    type Item;
    /// `Next(1, ...)`.
    fn fetch(&mut self) -> Fetch<Self::Item>;
    /// `Reset()`.
    fn reset(&mut self);
}

/// An `IACList` source.
pub trait AclExpand {
    /// `Expand(pattern)`, which returns an HRESULT.
    fn expand(&mut self, pattern: &str) -> u32;
}

/// One entry of `ACLMulti::objs`. Either interface may be absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member<E, A> {
    /// `pEnum`.
    pub enumerator: Option<E>,
    /// `pACL`.
    pub acl: Option<A>,
}

/// The walk over the members of `ACLMulti`.
#[derive(Debug)]
pub struct AclMulti<E, A> {
    members: Vec<Member<E, A>>,
    curr: usize,
}

impl<E: StringEnum, A: AclExpand> AclMulti<E, A> {
    /// Creates the list with `currObj` at 0.
    pub fn new(members: Vec<Member<E, A>>) -> Self {
        Self { members, curr: 0 }
    }

    /// `currObj`: the index of the member that the next call reads from.
    pub fn current(&self) -> usize {
        self.curr
    }

    /// The members, in order.
    pub fn members(&self) -> &[Member<E, A>] {
        &self.members
    }

    /// `ACLMulti_Next` for one element. `Ok(Some)` is S_OK with the element, `Ok(None)` is S_FALSE
    /// after every member is exhausted, and `Err(hr)` is a failure from a member. A member that
    /// reports an error stays current.
    pub fn next_string(&mut self) -> Result<Option<E::Item>, u32> {
        while self.curr < self.members.len() {
            if let Some(enumerator) = self.members[self.curr].enumerator.as_mut() {
                match enumerator.fetch() {
                    Fetch::Item(item) => return Ok(Some(item)),
                    Fetch::Failed(hr) => return Err(hr),
                    Fetch::Done => {}
                }
            }
            self.curr += 1;
        }
        Ok(None)
    }

    /// `ACLMulti_Reset`: restarts the walk and resets every enumerator.
    pub fn reset(&mut self) {
        self.curr = 0;
        for member in &mut self.members {
            if let Some(enumerator) = member.enumerator.as_mut() {
                enumerator.reset();
            }
        }
    }

    /// `ACLMulti_Expand`: expands every member that has an `IACList`, and returns the result of the
    /// last one (S_OK when none has one).
    pub fn expand(&mut self, pattern: &str) -> u32 {
        let mut res = S_OK;
        for member in &mut self.members {
            if let Some(acl) = member.acl.as_mut() {
                res = acl.expand(pattern);
            }
        }
        res
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
