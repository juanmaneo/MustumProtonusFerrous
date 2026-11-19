//! Checks the port against rows recorded from the verbatim Wine C code (`dlls/d3dxof/mszip.c`
//! and `parse_header` in `dlls/d3dxof/parsing.c`, built with ASan and UBSan, no reports).
//! Each row is `(commands, replies)`: the `|`-separated commands run in order, each on its own
//! input, and the `|`-separated replies must match exactly.

use crate::header::parse_header;
use crate::mszip::decompress;

#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("mszip 5 434b010500faff68656c6c6f|mszip 3 434b010500faff68656c6c6f", "rc=0 len=5 fnv=a430d84680aabd0b|rc=0 len=3 fnv=334a30192fe3892e"),
    ("mszip 3 434b4b4c4a0600|mszip 12 434b4b4c4a862300", "rc=0 len=3 fnv=e71fa2190541574b|rc=0 len=12 fnv=d63946adcbe9c9c5"),
    ("mszip 600 434b7591510ac3201044afe2d55614531aa9d0dc9f863c5a1f91fe0ccb30cece8ea5ee47a416bd477a3fda89cf182352ae271dfbd8bec4abd716a95c72101e3cb6fab329b284c7274f0904969e6d6684774ae58346c26ecc78ea19c480193d8c832cab6c0c5e74aeb722f2dc90e7caf56a07d22da073baf7b5b67c7b8fce97fb1e34f81afd572e5cf19cfa5fc920961f", "rc=0 len=600 fnv=28f96fb21513cb47"),
    ("mszip 910 434b6260646266616563e7e0e4e2e6e1e5e317101412161115139790949296916518951d951d95a5b12c000000ffff2b49cd5348aa2c492d560400", "rc=0 len=910 fnv=0e2d768a45309756"),
    ("mszip 32600 434b4b4c4a4e494d4b1fa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9516a941aa546a9518a06d4e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cde0b307a2fc0e8bd00a3f7028cf87b0180f7020000", "rc=0 len=32600 fnv=8b158eaf127511b5"),
    ("mszip 32768 434bedcf874208000040c1526820a1411a84a29422251a3293b434901295910889541ac86c50149576329a76146959953253f68eb2a2ad7cc8bb3f3801c13e42c27dfbf5171115131f307090c460c92143874949cbc80e1f2137525e415169d468e53163c7a9a88e9fa0a63e5143739296f6e4293a5375f5a6e94f9f616068643cd364d6ec3973e7cd375d60b6d07c9185a595b5cd625b3b7b87254b97392e77725ee1b27295ab9bfbea356bd779acf7dcb0d16bd3e62dde5b7db66df7dde1e71fb03330283864d7ee3da17bf7ed3f70f050587844e4e12351d1478fc5c41e3f11179f703231293925352d3de354e6e93367cf6565e7e4e6e59fbf70f1d2e52b570bae5d2f2cba71b3f85649695979c5ed3b77efddafacaa7e5053fbf0d1e3274f9fd53daf6f78f1f2d5eb376fdfbdfff0f1d3e72f8d5fbf35357ffff1f3d7ef963f7f5bdbda3b3abbbafff508f0e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffefcf9f3e7cf9f3f7ffeffffbd", "rc=0 len=32768 fnv=ff235a2901aba325"),
    ("mszip 0 434b0300|mszip 32769 434b010500faff68656c6c6f", "rc=0 len=0 fnv=cbf29ce484222325|rc=1"),
    ("mszip 5 5858010500faff68656c6c6f|mszip 5 434b07000000", "rc=2|rc=2"),
    ("mszip 5 434b010500000068656c6c6f", "rc=2"),
    ("mszip 1 434b05e1010400000080200000000000000000000000000800000000000000000000000000000000000000e805|mszip 1 434b05e001040000000010000000000000000000000000080000000000000000000000000000000000000024", "rc=0 len=1 fnv=af63dc4c8601ec8c|rc=0 len=1 fnv=af63dc4c8601ec8c"),
    ("mszip 1 434b05e001040000008020000000000000000000000000180000000000000000000000000000000000000038", "rc=2"),
    ("mszip 1 434b05e00104000000001000000000000000000000000018000000000000000000000000000000000000000c", "rc=2"),
    ("mszip 0 434b05e00104000000001000000000000000000000000000000000000000000000000000000000000000000c", "rc=0 len=0 fnv=cbf29ce484222325"),
    ("mszip 1 434b05e001922449922449fcffff", "rc=2"),
    ("mszip 1 434b05e0010800000080200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "rc=2"),
    ("header 786f66203033303262696e203030333268656c6c6f|header 786f662030333033747874203030363478", "hr=00000000 txt=0 len=5 fnv=a430d84680aabd0b|hr=00000000 txt=1 len=1 fnv=af63f54c86021707"),
    ("header 584f46203033303262696e2030303332|header 786f66203033303162696e2030303332|header 786f6620303330326461742030303332|header 786f66203033303262696e2030303136", "hr=8876035d|hr=8876035e|hr=8876035d|hr=8876035f"),
    ("header 786f6620|header 786f662030333032627a697030303332", "hr=8876035d|hr=8876035d"),
    ("header 786f662030333032627a6970303033321500000005000c00434b010500faff68656c6c6f", "hr=00000000 txt=0 len=5 fnv=a430d84680aabd0b"),
    ("header 786f662030333032747a6970303033321b00000005000c00434b010500faff68656c6c6f06000d00434b010600f9ff776f726c6421", "hr=00000000 txt=1 len=11 fnv=eb6483b2944f7ee0"),
    ("header 786f662030333032627a6970303033321500000005000a00010500faff68656c6c6f", "hr=88760356"),
    ("header 786f662030333032627a6970303033321500000005000c", "hr=8876035d"),
    ("header 786f662030333032627a69703030333210000000", "hr=00000000 txt=0 len=0 fnv=cbf29ce484222325"),
];

#[test]
fn oracle_rows_match_wine() {
    for (commands, expected) in ROWS {
        assert_eq!(replay(commands), *expected, "commands: {commands}");
    }
}

#[test]
fn proper_prefixes_of_accepted_streams_are_rejected() {
    for (commands, replies) in ROWS {
        for (command, reply) in commands.split('|').zip(replies.split('|')) {
            let Some(rest) = command.strip_prefix("mszip ") else {
                continue;
            };
            if !reply.starts_with("rc=0 ") {
                continue;
            }
            let (outlen, hex) = rest.split_once(' ').expect("mszip command");
            let input = unhex(hex);
            for end in 0..input.len() {
                assert!(
                    decompress(&input[..end], num(outlen)).is_err(),
                    "{end} leading bytes accepted: {command}"
                );
            }
        }
    }
}

#[test]
fn unknown_command_is_bad() {
    assert_eq!(run_command("bogus"), "bad-op");
    assert_eq!(run_command(""), "bad-op");
}

fn replay(commands: &str) -> String {
    commands
        .split('|')
        .map(run_command)
        .collect::<Vec<_>>()
        .join("|")
}

fn run_command(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    let arg = |index: usize| words.get(index).copied().unwrap_or("");
    match words.first().copied().unwrap_or("") {
        "mszip" => mszip_reply(num(arg(1)), &unhex(arg(2))),
        "header" => header_reply(&unhex(arg(1))),
        _ => "bad-op".to_string(),
    }
}

fn mszip_reply(outlen: usize, input: &[u8]) -> String {
    match decompress(input, outlen) {
        Ok(out) => format!("rc=0 len={} fnv={:016x}", out.len(), fnv1a(&out)),
        Err(error) => format!("rc={}", error as u8),
    }
}

fn header_reply(file: &[u8]) -> String {
    match parse_header(file) {
        Ok(xof) => format!(
            "hr=00000000 txt={} len={} fnv={:016x}",
            u8::from(xof.txt),
            xof.body.len(),
            fnv1a(&xof.body)
        ),
        Err(error) => format!("hr={:08x}", error.hresult()),
    }
}

/// FNV-1a, 64-bit, as the C harness hashes the bytes that a call produced.
fn fnv1a(data: &[u8]) -> u64 {
    data.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn num(text: &str) -> usize {
    text.parse().expect("decimal number")
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex byte"))
        .collect()
}
