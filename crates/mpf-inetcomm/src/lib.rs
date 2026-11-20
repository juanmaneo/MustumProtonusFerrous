//! Port of the deterministic logic in Wine's `dlls/inetcomm`: the mhtml URL split in `protocol.c`,
//! the SMTP status line in `smtptransport.c`, and the MIME header helpers and body decoders in
//! `mimeole.c`.
//!
//! Ported:
//! - `parse_mhtml_url`: [`mhtml::parse_mhtml_url`].
//! - The status line part of `SMTPTransport_ParseResponse`: [`smtp::parse_status`].
//! - The blank-line search of `copy_headers_to_buf`: [`header::header_block_len`].
//! - `read_prop` and `read_value`, which give the name, the unfolded value and the parameters of a
//!   header: [`header::read_field`]. Parameters are split only for Content-Type and
//!   Content-Disposition, the two properties that Wine flags with `MPF_HASPARAMS`.
//! - `unfold_header`, `unquote_string`, `split_params` and `add_param`: [`header::unfold`],
//!   [`header::unquote`] and [`header::split_params`].
//! - `init_content_type` and `init_content_encoding`: [`header::content_type`] and
//!   [`header::content_encoding`].
//! - `decode_base64` and `decode_qp`: [`decode_base64`] and [`codec::decode_quoted_printable`].
//!
//! Skipped:
//! - The POP3 response state machine in `pop3transport.c`. It needs a `sscanf("%lu")` emulation and
//!   the per-command result structs, so it is left for a later pass.
//! - The default property table of `mimeole.c`. Header names are returned as they appear.
//! - The COM objects, the sockets, the registry, MLang and the transport window. These need the OS,
//!   the network or shared state.
//!
//! Reference quirks that are kept:
//! - `read_prop` searches for the colon across lines. A line with no colon joins the name of the
//!   next header, and the name keeps the CRLF between them.
//! - In quoted-printable, `=` followed by two units that are not hex writes only `=`, and drops both
//!   units.
//! - Base64 skips every unit outside its alphabet, including `=`.
//! - SMTP status codes are read as a Windows 32-bit `long`, so they clamp at 0x7fffffff.
//!
//! Divergences from C, where the C code has undefined behavior:
//! - `read_value` with no CRLF after the value dereferences NULL in C. The port takes the rest of the
//!   block as the value.
//! - `unfold_header` on text with no CRLF is undefined in C. The port keeps the text.

pub mod codec;
pub mod header;
pub mod mhtml;
pub mod smtp;

/// `decode_base64`: decodes four units into three bytes. Units outside the base64 alphabet are
/// skipped, `=` included. A partial group at the end is dropped.
pub fn decode_base64(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut group = [0u8; 4];
    let mut n = 0;
    for &c in input {
        let Some(value) = base64_value(c) else {
            continue;
        };
        group[n] = value;
        n += 1;
        match n {
            2 => out.push((group[0] << 2) | (group[1] >> 4)),
            3 => out.push((group[1] << 4) | (group[2] >> 2)),
            4 => {
                out.push((group[2] << 6) | group[3]);
                n = 0;
            }
            _ => {}
        }
    }
    out
}

fn base64_value(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

#[cfg(test)]
mod edge_checks;
#[cfg(test)]
mod oracle_checks;
