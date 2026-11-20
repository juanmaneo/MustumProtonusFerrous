use crate::wide_str;

const QUOTE: u16 = b'"' as u16;
const SPACE: u16 = b' ' as u16;

/// `split_command`: splits a command line at its first space outside double quotes. Returns the
/// program part, and the parameters after that space if there is one. The quotes stay in the
/// program part, as they do in C.
pub fn split_command(cmd: &[u16]) -> (&[u16], Option<&[u16]>) {
    let cmd = wide_str(cmd);
    let mut in_quotes = false;
    for (i, &c) in cmd.iter().enumerate() {
        if c == QUOTE {
            in_quotes = !in_quotes;
        } else if c == SPACE && !in_quotes {
            return (&cmd[..i], Some(&cmd[i + 1..]));
        }
    }
    (cmd, None)
}
