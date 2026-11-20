//! Oracle rows and edge tests for the LZX port.
//!
//! The rows are the output of Wine's verbatim `dlls/itss/lzx.c`, built with ASan
//! and UBSan by `target/canon/itss/gen_rows.py`. Each row is a script of `init`,
//! `reset` and `dec` commands in the language of `harness.c`. [`replay`] runs the
//! script through the port and prints the replies the way the harness does, so
//! every row is compared as exact text. Rows whose C run is sanitizer-flagged are
//! excluded, and the generator keeps only rows whose output matches the plaintext
//! it encoded.

use super::*;

/// Rows from Wine's oracle: (script, expected replies).
#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    // uncompressed_basic
    ("init 32768|dec 8 003080005b0200000f3700000e800000666c3e3460787962", "ok|ret=0 out=666c3e3460787962"),
    // uncompressed_repeat
    ("init 32768|dec 21 0030a0000300000005000000070000007c282348756362553e4c00206801888888888a88aaaabeaabffffbeff8fe222222222222aa2aaaaa6f80bea5fc96fbbdfffee9bf222222222222aa2aaaaafefbbffffbef7823", "ok|ret=0 out=7c282348756362553e4c553e4c554c553e553e553e"),
    // uncompressed_odd
    ("init 32768|dec 9 003050003f5f000023b8000072990000734f7d495f0000608000c68b000049ae0000b13e00006f783728", "ok|ret=0 out=734f7d495f6f783728"),
    // uncompressed_split
    ("init 32768|dec 4 00308000f80b000073630000c3d0000059766c55216a3520|dec 4 216a3520", "ok|ret=0 out=59766c55|ret=0 out=216a3520"),
    // outlen_zero
    ("init 32768|dec 0 -", "ok|ret=0 out="),
    // verbatim_literals
    ("init 32768|dec 24 0010840144444444454455555f551d37003c40068031cf00204f8c01b1740c80b3f4c7d03d5f002c1d67861c01327f9ff2df11511111111155155555ff7ddf7ffdf7440444444444554555557f5ff7dffffd2767208bdfb2dc5ef214edd4a77f0000", "ok|ret=0 out=424d413054356f653149704a793d7c5930216c297849762d"),
    // slot0_off1
    ("init 32768|dec 6 0010640044444444454455555f55b57f7fcff7df14fd44444444454455555055e700efbffefb98ff888888888888aaaaabaafbeffffeecbf00a0", "ok|ret=0 out=414141414141"),
    // long_match
    ("init 32768|dec 41 0010940244444444454455555f55d17fbfe7fbefa4fd44444444454455555e553d9f7ffff7df84fc44444444454455555f559e2fadff7fcff0df0050", "ok|ret=0 out=5858585858585858585858585858585858585858585858585858585858585858585858585858585858"),
    // offset35
    ("init 32768|dec 46 0010e40244444444454455555f551837c70000430c1806001830740c8e313a8ec01860000130868e1803e0e80130868ed0313ac6c058df63fcf7449444444444554555557f5fc7cdefbfe0fb880888888888aa8aaaaaffbeefbffefb8acb2e2ae2849c75d73c1169b2ec59470b035b7a62ed12901f6a00d4", "ok|ret=0 out=4a2f255b3e5b2d7938716c6b4e3366492d367c6c4458343e2d2250637c58727a615338212d5a3d2a5b2d7938716c"),
    // aligned_extra12
    ("init 32768|dec 19 002036016ddb44b44444444455455555735f0140e49cd1334fd79021cf009e857aa6f719fffd11511111111155115555ce576bcfff3eefbf28fa888888888a88aaaabeaabffffbefcbfee1eaa86b3d4100fc", "ok|ret=0 out=3871356c3e36634c46343048343048344c4634"),
    // aligned_extra3
    ("init 32768|dec 24 002086016ddb44b44444444455455555335f1920c7003d63e31c8ca53d67742c19b060ef3100869e597afb0cfffe889688888888aa88aaaaefabf3f6fbeffffe888088888888aa88aaaaefabfefbbfffacecbe86e358c620ebdcf739806d", "ok|ret=0 out=447820416b33264b54432371426c397a3a3a6752416b3326"),
    // aligned_extra_gt3
    ("init 32768|dec 48 002006036ddb44b444444444554555554f5f8017c7001d4bc71b01470b80b0000bbb00c060580030c01800060030001805c03186c67830008c016358f7df84fc44444444454455555f55ca7fc7b0efbfe0fb880088888888aa88aaaaefabfefbbfff89ecc275f367d5808d235e607fcff437496cb478e7baec40624e3f6afd2f0080", "ok|ret=0 out=2d5c767d57727b6627625b554b3a6e41534f7a6c4970345f57414b79714e4c2829463b4c6f4373697d57727b6657727b"),
    // aligned_slot3
    ("init 32768|dec 16 002006016ddb44b44444444455455555775f163e3580d7db3d4bf52c00f4f719fffd11311111111155115555c4574fd73c3dfefbbfff22ed222222222a22aaaafbaafffeefbf2bfbe80dce7500fa", "ok|ret=0 out=5b694b62794875317531317575313131"),
    // reset_then_decode
    ("init 32768|dec 18 0010240144444444454455555f553d5f007dbc06e74c0d3db967f59cefb3fefb226222222222aa22aaaab1af7faff7df64fd44444444454455555f55df7ffdf765ffe91bcd8a8cf1|reset|dec 18 0010240144444444454455555f553d5f007dbc06e74c0d3db967f59cefb3fefb226222222222aa22aaaab1af7faff7df64fd44444444454455555f55df7ffdf765ffe91bcd8a8cf1", "ok|ret=0 out=5b3b566e542b555c6d79375c6d79375c6d79|ret=0|ret=0 out=5b3b566e542b555c6d79375c6d79375c6d79"),
    // invalid_type0
    ("init 32768|dec 8 00008000|dec 4 -", "ok|ret=2|ret=2"),
    // incomplete_pretree
    ("init 32768|dec 8 0010850055555555555555550055", "ok|ret=2"),
    // incomplete_main
    ("init 32768|dec 8 0010840044444444454455555f55b57f7fcff7df14fd44444444454455555f55df7ffdf740ff", "ok|ret=2"),
    // stale_table
    ("init 32768|dec 8 0010840044444444454455555f55b57f7fcff7df14fd44444444454455555f55df7ffdf740ff|dec 8 -", "ok|ret=2|ret=0 out=4141414141414141"),
    // multi_block
    ("init 32768|dec 55 0010440144444444454455555f557b33b5f0fbeffffe88b488888888aa88aaaae6abb3ef7faff7df34fd44444444454455555e553d8f7fffe7c7efbff2f5304e000000507992000000b60000241f00003765696e40000040cd03dbb6886888888888aa8aaaaaffbed0bf67dfff7ee6bf22a222222222aa2aaaaa7bfaefedf7dffffd112d11111111551155559d577b5efffeefbfc3f900ba", "ok|ret=0 out=38212021202120212021202120212021202120213765696e406a7820213765696e406a786a786a786a786a786a786a786a786a786a786a"),
    // continue_frames
    ("init 32768|dec 9 0010940044444444454455555f555d6b758dde7506d86dbcfe7b9eff888888888888aaaaabaaebe6f7dffffd017c11111111111155555755f7dffffdd97f2a730078|dec 5 0020a800888888888a88aaaabeaa3d6f7ffff7dfc8fb888888888a88aaaaa0aaf71cfffddf7f51f311111111151155557d557ffff7df98fd", "ok|ret=0 out=3b45762e5b582e5b58|ret=0 out=2e5b582e21"),
    // block_spans_calls
    ("init 32768|dec 22 0010040244444444454455555f5521638e9e3340cfd03d63f41cd0f353cf063e0340ff3ee3bf22a222222222aa2aaaaad9f973d7ff3eefbfc8f9888888888a88aaaabeaabffffbefccfee32245593e4d57c400c0|dec 10 e013", "ok|ret=0 out=4f34377a3c652c4f5c575c497e4f4f5c5c575c497e4f|ret=0 out=497e4f497e2e7e2e7e2e"),
    // zero_padding
    ("init 32768|dec 30 0010e40144444444454455555f555e4700f8c26b00d0df6bfcf744e444444444554555550d50f5766be0afb7df7ffcf744b444444444554555557b5efffed4bfbfe798e7c0f800a80000000000000000", "ok|ret=0 out=5c5c5c5c5c775c5c5c5c5c775c5c5c5c5c5c5c5c5c5c5c5c252525252573"),
    // random_verbatim
    ("init 32768|dec 59 0010b40344444444454455555f55c47f06d02d7c5978fdf725ff111111111111555554555d03f50ce175ab6747afff5eeebf22a222222222aa2aaaaab9f7e7d8efbffefb64ff57eff0a40056", "ok|ret=0 out=657c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c7c4e4b7c7c"),
    // random_aligned
    ("init 32768|dec 37 002056026ddb44b444444444554555557f5fe7c0efbffefb225a22222222aa22aaaa4fafff9eefbffefb224222222222aa22aaaa3baff7bcfffddf7f96f1", "ok|ret=0 out=47474747474747474747474747474747474747474747474747474747474747474747474747"),
    // random_mixed
    ("init 32768|dec 53 0010a40044444444454455555f55cc7fbfe7fbeff4fd44444444454455555e553d9f7ffff7df84fc44444444454455555e55fe7b3dff7fff4cd70000001838570000d8650000808e000059493375477a00404d03dbb6886888888888aa8aaaaab6be2dbcf576d7d55e57bfff82ef222222222222aaaaafaaaeb180b6ef35fefb90ff888888888888aaaaaaaacf21df7ffbf7f7dc3ffc605c08ac0000225a22222222aa22aaaabfaf6be1afaf7e07bfff22ed222222222a22aaaaefaadfcfefbffefb22f222222222aa22aaaabfaffbeffffe67b200d0", "ok|ret=0 out=5353535353535353535359493375477a535353537a535353537a655b2a41517a655b2a41517a655b2a416b49655b55655b55655b55"),
];

fn unhex(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Runs one harness script through the port and prints the replies as `harness.c` does.
fn replay(script: &str) -> String {
    let mut state: Option<Lzx> = None;
    let mut replies = Vec::new();
    for cmd in script.split('|') {
        let t: Vec<&str> = cmd.split_whitespace().collect();
        let reply = match t[0] {
            "init" => {
                state = Lzx::new(t[1].parse().unwrap());
                if state.is_some() { "ok" } else { "null" }.to_string()
            }
            "reset" => {
                state.as_mut().unwrap().reset();
                "ret=0".to_string()
            }
            "dec" => {
                let input = unhex(t[2]);
                let outlen = t[1].parse().unwrap();
                match state.as_mut().unwrap().decompress(&input, outlen) {
                    Ok(out) => format!("ret=0 out={}", hex(&out)),
                    Err(e) => format!("ret={}", e.code()),
                }
            }
            _ => panic!("unknown harness command: {}", t[0]),
        };
        replies.push(reply);
    }
    replies.join("|")
}

#[test]
fn oracle_rows_match_wine() {
    for (script, expected) in VECTORS {
        assert_eq!(replay(script), *expected, "{script}");
    }
}

#[test]
fn replay_covers_the_remaining_harness_paths() {
    assert_eq!(replay("init 4096"), "null");
    assert_eq!(replay("init 32768|dec 0 -"), "ok|ret=0 out=");
    assert_eq!(replay("init 32768|reset"), "ok|ret=0");
}

#[test]
#[should_panic(expected = "unknown harness command")]
fn replay_rejects_unknown_commands() {
    replay("bogus");
}
