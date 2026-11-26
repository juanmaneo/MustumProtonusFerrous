//! Port of the portable logic in Wine's `dlls/infosoft/wordbreaker.c`.
//!
//! Ported:
//! - `wb_BreakText`, the word-splitting state machine over a UTF-16 text source: [`break_text`]
//! - `call_sink`, which emits one word and advances the cursor (used by [`break_text`])
//! - The separator test `!ch || iswpunct(ch) || iswspace(ch)` for the C locale:
//!   [`is_c_locale_separator`]
//!
//! Skipped:
//! - COM plumbing: `wb_QueryInterface`, `wb_AddRef`, `wb_Release`, `wb_Constructor` and the vtable.
//! - `wb_Init`, which only sets `*pfLicense = FALSE`.
//! - `wb_ComposePhrase` and `wb_GetLicenseToUse`: `FIXME` stubs.
//! - `IWordSink::PutWord` and `IPhraseSink`: the port passes words to a closure, and the COM
//!   wrapper owns the sinks. `wb_BreakText` ignores the `PutWord` result, so the port has none.
//! - `infosoft_main.c`: class factory, `DllGetClassObject`, registration.

/// Text source cursor: the Rust side of Wine's `TEXT_SOURCE` fields `awcBuffer`, `iCur` and
/// `iEnd`. `buffer[cur..end]` is the unread text, and `end` must not exceed `buffer.len()`.
pub struct TextSource {
    pub buffer: Vec<u16>,
    pub cur: usize,
    pub end: usize,
}

/// `wb_BreakText`. Calls `put_word` with each word and its position in `buffer`, in order.
///
/// `is_separator` is Wine's `!ch || iswpunct(ch) || iswspace(ch)` test, so it must treat NUL
/// as a separator to match Wine. `fill_text_buffer` runs after each pass over the buffer. It
/// returns `true` only where Wine's loop continues on `S_OK`, and `false` ends the loop for any
/// other result. A word still open at `end` is emitted as a fragment, and the next buffer
/// continues in word state, so a word that crosses a buffer boundary is split, as in Wine.
pub fn break_text(
    ts: &mut TextSource,
    is_separator: impl Fn(u16) -> bool,
    mut put_word: impl FnMut(&[u16], usize),
    mut fill_text_buffer: impl FnMut(&mut TextSource) -> bool,
) {
    let mut in_word = false;
    loop {
        let mut len = 0;
        while ts.cur + len < ts.end {
            let ch = ts.buffer[ts.cur + len];
            if !in_word {
                if is_separator(ch) {
                    ts.cur += 1;
                } else {
                    in_word = true;
                }
            } else if is_separator(ch) {
                call_sink(ts, len, &mut put_word);
                len = 0;
                in_word = false;
            } else {
                len += 1;
            }
        }
        call_sink(ts, len, &mut put_word);
        if !fill_text_buffer(ts) {
            return;
        }
    }
}

/// `call_sink`. Emits `buffer[cur..cur + len]` with source position `cur`, then advances `cur`.
/// Zero-length words are not emitted.
fn call_sink(ts: &mut TextSource, len: usize, put_word: &mut impl FnMut(&[u16], usize)) {
    if len == 0 {
        return;
    }
    let start = ts.cur;
    put_word(&ts.buffer[start..start + len], start);
    ts.cur += len;
}

/// The C-locale form of Wine's separator test: NUL, and the ASCII punctuation and whitespace
/// that `iswpunct` and `iswspace` accept in the C locale (`\t \n \v \f \r` and space).
/// Code units above 0x7F are not separators, because the C locale classifies only ASCII.
pub fn is_c_locale_separator(ch: u16) -> bool {
    match u8::try_from(ch) {
        Ok(b) => b == 0 || b.is_ascii_punctuation() || matches!(b, 0x09..=0x0D | 0x20),
        Err(_) => false,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
