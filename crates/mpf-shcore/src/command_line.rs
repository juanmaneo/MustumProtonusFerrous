//! `CommandLineToArgvW` in Wine's `dlls/shcore/main.c`.

use crate::str_len;

const QUOTE: u16 = b'"' as u16;
const BACKSLASH: u16 = b'\\' as u16;

fn is_blank(c: u16) -> bool {
    c == b' ' as u16 || c == b'\t' as u16
}

/// Splits a command line into arguments, with `argv[0]` first, as `CommandLineToArgvW` does.
///
/// The line ends at its first NUL. An empty line gives `module_path` as the only argument. The
/// executable name ends at a closing quote when it starts with one, and at a blank otherwise.
/// Backslashes before a quote count in pairs, and a quote after an odd number of them is literal.
/// Consecutive quotes inside an argument follow the three-quote rule of the C code.
pub fn command_line_to_argv(cmdline: &[u16], module_path: &[u16]) -> Vec<Vec<u16>> {
    let line = &cmdline[..str_len(cmdline)];
    if line.is_empty() {
        return vec![module_path.to_vec()];
    }

    let mut args = Vec::new();
    let mut exe = Vec::new();
    let mut s = 0;
    if line[0] == QUOTE {
        s = 1;
        while s < line.len() {
            let c = line[s];
            s += 1;
            if c == QUOTE {
                break;
            }
            exe.push(c);
        }
    } else {
        while s < line.len() && !is_blank(line[s]) {
            exe.push(line[s]);
            s += 1;
        }
        if s < line.len() {
            s += 1;
        }
    }
    args.push(exe);

    while s < line.len() && is_blank(line[s]) {
        s += 1;
    }
    if s >= line.len() {
        return args;
    }

    let mut cur = Vec::new();
    let mut qcount = 0usize;
    let mut bcount = 0usize;
    while s < line.len() {
        let c = line[s];
        if is_blank(c) && qcount == 0 {
            args.push(std::mem::take(&mut cur));
            bcount = 0;
            while s < line.len() && is_blank(line[s]) {
                s += 1;
            }
            if s >= line.len() {
                return args;
            }
        } else if c == BACKSLASH {
            cur.push(c);
            bcount += 1;
            s += 1;
        } else if c == QUOTE {
            if bcount.is_multiple_of(2) {
                cur.truncate(cur.len() - bcount / 2);
                qcount += 1;
            } else {
                cur.truncate(cur.len() - bcount / 2 - 1);
                cur.push(QUOTE);
            }
            s += 1;
            bcount = 0;
            while s < line.len() && line[s] == QUOTE {
                qcount += 1;
                if qcount == 3 {
                    cur.push(QUOTE);
                    qcount = 0;
                }
                s += 1;
            }
            if qcount == 2 {
                qcount = 0;
            }
        } else {
            cur.push(c);
            bcount = 0;
            s += 1;
        }
    }
    args.push(cur);
    args
}
