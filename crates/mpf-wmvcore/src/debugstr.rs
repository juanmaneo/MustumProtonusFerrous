//! `debugstr_time` from Wine's `wmvcore_private.h`.

/// Formats a time in 100 ns units as seconds, with up to seven fractional
/// digits. At least one fractional digit is kept.
pub fn debugstr_time(time: i64) -> String {
    let mut abstime = time.unsigned_abs();
    let mut buffer = Vec::new();
    while abstime != 0 || buffer.len() <= 8 {
        buffer.push(b'0' + (abstime % 10) as u8);
        abstime /= 10;
        if buffer.len() == 7 {
            buffer.push(b'.');
        }
    }
    if time < 0 {
        buffer.push(b'-');
    }
    buffer.reverse();

    let mut len = buffer.len();
    while buffer[len - 1] == b'0' && buffer[len - 2] != b'.' {
        len -= 1;
    }
    buffer[..len].iter().map(|&byte| char::from(byte)).collect()
}
