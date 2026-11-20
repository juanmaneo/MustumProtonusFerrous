//! The status line part of `SMTPTransport_ParseResponse` in `smtptransport.c`: the reply code, the
//! continuation flag and the `IXP_E_SMTP_*` result. The command logging callback is not ported.

/// `IXP_E_SMTP_RESPONSE_ERROR`. It is also the result for a reply code the table does not know.
pub const RESPONSE_ERROR: u32 = hr_e(0xCC60);

/// The parsed status line of a reply.
#[derive(Debug, PartialEq, Eq)]
pub struct Status {
    /// The reply code. Leading digits are read as a Windows 32-bit `long`, so the value clamps at
    /// `LONG_MAX`.
    pub code: u32,
    /// `false` when the code is followed by `-`, which marks a continuation line.
    pub done: bool,
    /// The `IXP_E_SMTP_*` result for `code`.
    pub hr: u32,
}

/// Parses one reply line. Returns `None` when the line does not start with a digit, which is
/// `IXP_E_SMTP_RESPONSE_ERROR` in the C code.
pub fn parse_status(line: &[u8]) -> Option<Status> {
    if !line.first().is_some_and(u8::is_ascii_digit) {
        return None;
    }
    let digits = line.iter().take_while(|c| c.is_ascii_digit()).count();
    let code = line[..digits].iter().fold(0u64, |acc, &d| {
        (acc * 10 + u64::from(d - b'0')).min(LONG_MAX)
    });
    let code = code as u32;
    Some(Status {
        code,
        done: line.get(digits) != Some(&b'-'),
        hr: result_for(code),
    })
}

const LONG_MAX: u64 = 0x7fff_ffff;

const fn hr_e(n: u32) -> u32 {
    (1 << 31) | (12 << 16) | n
}

const fn hr_s(n: u32) -> u32 {
    (12 << 16) | n
}

fn result_for(code: u32) -> u32 {
    match code {
        211 => hr_s(0xCC70),
        214 => hr_s(0xCC71),
        220 => hr_s(0xCC72),
        221 => hr_s(0xCC73),
        245 => hr_s(0xCC79),
        250 => hr_s(0xCC74),
        251 => hr_s(0xCC75),
        334 => hr_s(0xCC78),
        354 => hr_s(0xCC76),
        421 => hr_e(0xCC67),
        450 => hr_e(0xCC68),
        451 => hr_e(0xCC6A),
        452 => hr_e(0xCC6C),
        454 => hr_e(0xCC7F),
        500 => hr_e(0xCC62),
        501 => hr_e(0xCC63),
        502 => hr_e(0xCC64),
        503 => hr_e(0xCC65),
        504 => hr_e(0xCC66),
        530 => hr_e(0xCC7C),
        550 => hr_e(0xCC69),
        551 => hr_e(0xCC6B),
        552 => hr_e(0xCC6D),
        553 => hr_e(0xCC6E),
        554 => hr_e(0xCC6F),
        _ => RESPONSE_ERROR,
    }
}
