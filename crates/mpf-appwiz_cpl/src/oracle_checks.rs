//! Replays the commands of the Wine oracle (`target/canon/appwiz.cpl`) against this crate.
//! The rows are the verbatim output of Wine's `addons.c` and `appwiz.c` under ASan and UBSan,
//! run through Win32 shims. Each row is `(commands, replies)`: the `|`-separated commands run in
//! order, and the `|`-separated replies must match exactly.

use crate::{
    append_url_params, build_install_path, cache_file_path, digest_matches, get_url,
    install_file_result, install_target, start_params, Addon,
};

/// Rows recorded from the verbatim Wine C code: `(commands, replies)`.
#[rustfmt::skip]
const ROWS: &[(&str, &str)] = &[
    ("start install_gecko", "ret=1 addon=gecko"),
    ("start install_mono", "ret=1 addon=mono"),
    ("start install_foo|start -|start @|start install_gecko_x", "ret=0 addon=-|ret=0 addon=-|ret=0 addon=-|ret=0 addon=-"),
    ("start install_gecko|start install_mono", "ret=1 addon=gecko|ret=1 addon=mono"),
    ("query http://source.winehq.org/winegecko.php x86_64 2.47.4 10.0", "http://source.winehq.org/winegecko.php?arch=x86_64&v=2.47.4&winev=10.0"),
    ("query http://x.org/a x86 11.3.0 -", "http://x.org/a?arch=x86&v=11.3.0&winev="),
    ("geturl gecko - 1 -", "http://source.winehq.org/winegecko.php?arch=x86_64&v=2.47.4&winev="),
    ("geturl mono - 1 10.0", "http://source.winehq.org/winemono.php?arch=x86&v=11.3.0&winev=10.0"),
    ("geturl gecko http://h.org/x.php 1 -", "http://h.org/x.php?arch=x86_64&v=2.47.4&winev="),
    ("geturl gecko https://h.org/x.php 1 10.0", "https://h.org/x.php?arch=x86_64&v=2.47.4&winev=10.0"),
    ("geturl gecko ftp://h.org/x 1 -", "ftp://h.org/x"),
    ("geturl mono htt 1 -", "htt"),
    ("geturl mono http 1 -", "http?arch=x86&v=11.3.0&winev="),
    ("geturl gecko http://h.org/x 4 -", "http://source.winehq.org/winegecko.php?arch=x86_64&v=2.47.4&winev="),
    ("geturl gecko @ 1 -", ""),
    ("path \\??\\C:\\wine\\share @ wine-gecko.msi", "res=0 msi=C:\\wine\\share\\\\wine-gecko.msi"),
    ("path \\??\\C:\\wine\\ @ f.msi|path @ @ a|path /x/ /sub/ f.msi", "res=0 msi=C:\\wine\\\\f.msi|res=2|res=0 msi=/x//sub/\\f.msi"),
    ("path \\\\?\\C:\\wine @ f.msi|path \\??\\ab @ f", "res=0 msi=C:\\wine\\\\f.msi|res=0 msi=\\\\?\\ab\\\\f"),
    ("cache @ \\\\home\\u f.msi", "\\\\home\\u\\.cache\\wine\\f.msi"),
    ("cache - \\??\\Z:\\home\\u f.msi|cache /x/c - f|cache - - f", "\\\\?\\Z:\\home\\u\\.cache\\wine\\f.msi|\\\\?\\unix/x/c\\wine\\f|NULL"),
    ("cache @ @ f|cache - \\\\h f", "\\\\cache\\wine\\f|\\\\h\\.cache\\wine\\f"),
    ("sha gecko e590b7d988a32d6aa4cf1d8aa3aa3d33766fdd4cf4c89c2dcc2095ecb28d066f", "ok=1"),
    ("sha gecko 00000000000000000000000000000000000000000000000000000000000000ff|sha mono df2dfc1665c2511882e7cabd56eafd0c0a3d94e5a7e86f969277f6c189d418d3", "ok=0|ok=1"),
    ("name gecko|name mono", "wine-gecko-2.47.4-x86_64.msi gecko|wine-mono-11.3.0-x86.msi mono"),
];

#[test]
fn oracle_rows_match_wine() {
    for (commands, replies) in ROWS {
        assert_eq!(replay(commands), *replies, "row: {commands}");
    }
}

#[test]
fn unknown_command_is_bad() {
    assert_eq!(run_command("nope x"), "unknown");
}

fn replay(commands: &str) -> String {
    commands
        .split('|')
        .map(run_command)
        .collect::<Vec<_>>()
        .join("|")
}

fn run_command(command: &str) -> String {
    let words: Vec<&str> = command.split(' ').collect();
    match words.as_slice() {
        ["start", param] => {
            let addon = start_params(nullable(param).as_deref());
            format!(
                "ret={} addon={}",
                u8::from(addon.is_some()),
                addon_name(addon)
            )
        }
        ["query", base, arch, version, winev] => {
            let url = append_url_params(&wide(base), arch, version, winev_arg(winev));
            String::from_utf16_lossy(&url.unwrap_or_default())
        }
        ["geturl", addon, reg, ty, winev] => {
            let ty: u32 = ty.parse().unwrap();
            let data = nullable(reg).map(|reg| [reg, vec![0]].concat());
            let reg = data.as_deref().map(|data| (ty, data));
            let url = get_url(addon_of(addon), reg, winev_arg(winev));
            String::from_utf16_lossy(&url.unwrap_or_default())
        }
        ["path", dir, subdir, file] => {
            let path = build_install_path(&arg(dir), &arg(subdir), &arg(file));
            match install_target(&path) {
                None => "res=2".to_string(),
                Some(target) => format!(
                    "res={} msi={}",
                    install_file_result(0, || 0) as u32,
                    String::from_utf16_lossy(target)
                ),
            }
        }
        ["cache", xdg, home, file] => {
            match cache_file_path(
                nullable(xdg).as_deref(),
                nullable(home).as_deref(),
                &arg(file),
            ) {
                Some(path) => String::from_utf16_lossy(&path),
                None => "NULL".to_string(),
            }
        }
        ["sha", addon, hex] => {
            let mut digest = [0u8; 32];
            for (byte, pair) in digest.iter_mut().zip(hex.as_bytes().chunks(2)) {
                *byte = u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
            }
            format!(
                "ok={}",
                u8::from(digest_matches(&digest, addon_of(addon).sha()))
            )
        }
        ["name", addon] => {
            let addon = addon_of(addon);
            format!("{} {}", addon.file_name(), addon.subdir())
        }
        _ => "unknown".to_string(),
    }
}

fn addon_of(word: &str) -> Addon {
    if word == "mono" {
        Addon::Mono
    } else {
        Addon::Gecko
    }
}

fn addon_name(addon: Option<Addon>) -> &'static str {
    match addon {
        Some(Addon::Gecko) => "gecko",
        Some(Addon::Mono) => "mono",
        None => "-",
    }
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// A command argument: `@` is the empty string.
fn arg(word: &str) -> Vec<u16> {
    if word == "@" {
        Vec::new()
    } else {
        wide(word)
    }
}

/// A nullable argument: `-` is NULL.
fn nullable(word: &str) -> Option<Vec<u16>> {
    if word == "-" {
        None
    } else {
        Some(arg(word))
    }
}

fn winev_arg(word: &str) -> Option<&str> {
    if word == "-" {
        None
    } else {
        Some(word)
    }
}
