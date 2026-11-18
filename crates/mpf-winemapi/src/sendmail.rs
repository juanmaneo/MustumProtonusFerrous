//! The argument lists and the `mailto:` string of `sendmail.c`.

use crate::{Message, MAPI_BCC, MAPI_CC, MAPI_E_FAILURE, MAPI_TO};

/// `isspace` in the C locale: tab, newline, vertical tab, form feed, carriage return and space.
pub fn is_c_space(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | b' ')
}

/// `is_non_empty`: whether the string has a byte that is not white space.
pub fn is_non_empty(text: Option<&[u8]>) -> bool {
    text.is_some_and(|text| text.iter().any(|&byte| !is_c_space(byte)))
}

/// Drops a leading `smtp:` (any ASCII case), as `add_target` and `send_mail_winebrowser` do.
pub fn strip_smtp_prefix(address: &[u8]) -> &[u8] {
    match address.get(..5) {
        Some(prefix) if prefix.eq_ignore_ascii_case(b"smtp:") => &address[5..],
        _ => address,
    }
}

/// `add_target`: appends the arguments for one recipient. `MAPI_ORIG` and unknown classes add nothing.
pub fn add_target<'a>(argv: &mut Vec<&'a [u8]>, class: u32, address: &'a [u8]) {
    let address = strip_smtp_prefix(address);
    match class {
        MAPI_TO => argv.push(address),
        MAPI_CC => argv.extend([&b"--cc"[..], address]),
        MAPI_BCC => argv.extend([&b"--bcc"[..], address]),
        _ => {}
    }
}

/// Keeps the text only when `is_non_empty` accepts it, as `send_mail_xdg` does for subject and body.
fn non_blank(text: Option<&[u8]>) -> Option<&[u8]> {
    text.filter(|&text| is_non_empty(Some(text)))
}

/// `send_mail_xdg`'s argument list: `xdg-email`, the recipients with an address, the files, then the subject and body.
pub fn xdg_argv<'a>(message: &Message<'a>) -> Vec<&'a [u8]> {
    let mut argv = vec![&b"xdg-email"[..]];
    for recipient in message.recipients() {
        if let Some(address) = recipient.address {
            add_target(&mut argv, recipient.class, address);
        }
    }
    for path in message.files.iter().copied().flatten() {
        argv.extend([&b"--attach"[..], path]);
    }
    if let Some(subject) = non_blank(message.subject) {
        argv.extend([&b"--subject"[..], subject]);
    }
    if let Some(body) = non_blank(message.note_text) {
        argv.extend([&b"--body"[..], body]);
    }
    argv
}

/// The addresses of one class, without `smtp:`, joined with commas in list order.
fn joined_addresses(message: &Message<'_>, class: u32) -> Vec<u8> {
    let addresses: Vec<&[u8]> = message
        .recipients()
        .iter()
        .filter(|recipient| recipient.class == class)
        .filter_map(|recipient| recipient.address)
        .map(strip_smtp_prefix)
        .collect();
    addresses.join(&b","[..])
}

/// `send_mail_winebrowser`'s `mailto:` string. `subject` and `body` are the already escaped texts.
pub fn mailto_url(message: &Message<'_>, subject: &[u8], body: &[u8]) -> Vec<u8> {
    let to = joined_addresses(message, MAPI_TO);
    let cc = joined_addresses(message, MAPI_CC);
    let bcc = joined_addresses(message, MAPI_BCC);
    let parts: [&[u8]; 11] = [
        b"mailto:\"",
        &to,
        b"\"?subject=\"",
        subject,
        b"\"&cc=\"",
        &cc,
        b"\"&bcc=\"",
        &bcc,
        b"\"&body=\"",
        body,
        b"\"",
    ];
    parts.concat()
}

/// `MAPISendMail`'s checks and the argument list of its `xdg-email` attempt. `None` is the NULL message.
/// Running `xdg-email`, and the `winebrowser` fallback, are left to the caller.
pub fn send_mail_argv<'a>(message: Option<&Message<'a>>) -> Result<Vec<&'a [u8]>, u32> {
    match message {
        Some(message) if message.recip_count == 0 || message.recips.is_some() => {
            Ok(xdg_argv(message))
        }
        _ => Err(MAPI_E_FAILURE),
    }
}
