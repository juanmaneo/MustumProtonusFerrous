use crate::rundll::{c_str, parse_rundll, RunDll};

/// What `PrintUIEntryW` does with the parsed command line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dispatch {
    /// A command was parsed. Its handler is not implemented, so `PrintUIEntryW` reports
    /// `command not implemented`.
    NotImplemented { command: u16, subcommand: u16 },
    /// The command line was empty or did not parse to a command.
    Failed,
}

/// `PrintUIEntryW` from `dlls/printui/printui.c`. `command_line` is `pCommand`, and `split` stands
/// in for `CommandLineToArgvW`, which is only called for a non-empty command line.
pub fn print_ui_entry_w(
    command_line: Option<&[u16]>,
    split: impl FnOnce(&[u16]) -> Vec<Vec<u16>>,
) -> Dispatch {
    let cx = match command_line.map(c_str).filter(|line| !line.is_empty()) {
        Some(line) => parse_rundll(&split(line)),
        None => RunDll::default(),
    };
    if cx.ok && cx.command != 0 {
        Dispatch::NotImplemented {
            command: cx.command,
            subcommand: cx.subcommand,
        }
    } else {
        Dispatch::Failed
    }
}
