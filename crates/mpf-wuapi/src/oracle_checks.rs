//! Tests against Wine's oracle vectors, then Rust-only edge cases.
//!
//! The vectors are the replies of Wine's verbatim `main.c` and `QueryInterface`
//! bodies, built with ASan and UBSan by `target/canon/wuapi/build.sh`.
//! [`replay`] runs each command through the port and prints the result the way
//! the C harness does, so every vector is compared as the exact text.

use super::*;

#[rustfmt::skip]
const VECTORS: &[(&str, &str)] = &[
    ("class 4cb43d7f-7eee-4906-869860da1c38f2fe 00000000-0000-0000-c000000000000046", "0x00000000 session"),
    ("class 4cb43d7f-7eee-4906-869860da1c38f2fe 00000001-0000-0000-c000000000000046", "0x00000000 session"),
    ("class bfe18e9c-6d87-4450-b37ce02f0b373803 00000001-0000-0000-c000000000000046", "0x00000000 automatic"),
    ("class c01b9ba0-bea7-41ba-b604d0a36f469133 00000001-0000-0000-c000000000000046", "0x00000000 sysinfo"),
    ("class d2e0fe7f-d23e-48e1-93c06fa8cc346474 00000001-0000-0000-c000000000000046", "0x00000000 installer"),
    ("class 4cb43d7f-7eee-4906-869860da1c38f2fe 816858a4-260d-4260-933a2585f1abc76b", "0x80004002 -"),
    ("class 4cb43d7f-7eee-4906-869860da1c38f2fe 00020400-0000-0000-c000000000000046", "0x80004002 -"),
    ("class 00000000-0000-0000-0000000000000000 00000001-0000-0000-c000000000000046", "0x80040111 -"),
    ("create session 1 816858a4-260d-4260-933a2585f1abc76b", "0x80040110"),
    ("create session 0 816858a4-260d-4260-933a2585f1abc76b", "0x00000000"),
    ("create session 0 00020400-0000-0000-c000000000000046", "0x00000000"),
    ("create session 0 8f45abf1-f9ae-4b95-a933f0f66e5056ea", "0x80004002"),
    ("create automatic 0 673425bf-c082-4c7c-bdfd569464b8e0ce", "0x00000000"),
    ("create automatic 0 816858a4-260d-4260-933a2585f1abc76b", "0x80004002"),
    ("create sysinfo 0 ade87bf7-7b56-4275-8fabb9b0e591844b", "0x00000000"),
    ("create installer 0 7b929c68-ccdc-4226-96b18724600b54c2", "0x00000000"),
    ("create installer 0 00000001-0000-0000-c000000000000046", "0x80004002"),
    ("create installer 1 00000000-0000-0000-c000000000000046", "0x80040110"),
    ("object session 816858a4-260d-4260-933a2585f1abc76b", "0x00000000"),
    ("object searcher 8f45abf1-f9ae-4b95-a933f0f66e5056ea", "0x00000000"),
    ("object downloader 68f1c6f9-7ecc-4666-a464247fe12496c3", "0x00000000"),
    ("object searcher 68f1c6f9-7ecc-4666-a464247fe12496c3", "0x80004002"),
    ("object automatic 673425bf-c082-4c7c-bdfd569464b8e0ce", "0x00000000"),
    ("object sysinfo 00020400-0000-0000-c000000000000046", "0x00000000"),
    ("object installer 7b929c68-ccdc-4226-96b18724600b54c2", "0x00000000"),
];

fn parse_guid(token: &str) -> Guid {
    let parts: Vec<&str> = token.split('-').collect();
    let mut data4 = [0u8; 8];
    for (i, byte) in data4.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&parts[3][2 * i..2 * i + 2], 16).unwrap();
    }
    Guid {
        data1: u32::from_str_radix(parts[0], 16).unwrap(),
        data2: u16::from_str_radix(parts[1], 16).unwrap(),
        data3: u16::from_str_radix(parts[2], 16).unwrap(),
        data4,
    }
}

/// The `HRESULT` as the C harness prints it.
fn hresult_text(hres: i32) -> String {
    format!("0x{:08X}", hres as u32)
}

fn class_name(class: ComClass) -> &'static str {
    match class {
        ComClass::UpdateSession => "session",
        ComClass::AutomaticUpdates => "automatic",
        ComClass::SystemInformation => "sysinfo",
        ComClass::UpdateInstaller => "installer",
    }
}

fn class_from_name(name: &str) -> ComClass {
    match name {
        "session" => ComClass::UpdateSession,
        "automatic" => ComClass::AutomaticUpdates,
        "sysinfo" => ComClass::SystemInformation,
        "installer" => ComClass::UpdateInstaller,
        other => panic!("unknown factory {other}"),
    }
}

fn object_from_name(name: &str) -> ComObject {
    match name {
        "session" => ComObject::UpdateSession,
        "searcher" => ComObject::UpdateSearcher,
        "downloader" => ComObject::UpdateDownloader,
        "installer" => ComObject::UpdateInstaller,
        "automatic" => ComObject::AutomaticUpdates,
        "sysinfo" => ComObject::SystemInformation,
        other => panic!("unknown object {other}"),
    }
}

/// Runs one oracle command through the port and formats it like the C harness.
fn replay(command: &str) -> String {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words[0] {
        "class" => {
            let clsid = parse_guid(words[1]);
            let hres = dll_get_class_object(&clsid, &parse_guid(words[2]));
            let name = match class_for_clsid(&clsid) {
                Some(class) if hres == S_OK => class_name(class),
                _ => "-",
            };
            format!("{} {name}", hresult_text(hres))
        }
        "create" => {
            let class = class_from_name(words[1]);
            let iid = parse_guid(words[3]);
            hresult_text(create_instance(class, words[2] == "1", &iid))
        }
        "object" => {
            let object = object_from_name(words[1]);
            hresult_text(object_query_interface(object, &parse_guid(words[2])))
        }
        other => panic!("unknown oracle command {other}"),
    }
}

#[test]
fn oracle_vectors_match_wine() {
    for (command, expected) in VECTORS {
        assert_eq!(replay(command), *expected, "{command}");
    }
}

#[test]
#[should_panic(expected = "unknown oracle command bogus")]
fn unknown_oracle_command_panics() {
    replay("bogus 1 2");
}

#[test]
#[should_panic(expected = "unknown factory bogus")]
fn unknown_factory_panics() {
    replay("create bogus 0 00000000-0000-0000-c000000000000046");
}

#[test]
#[should_panic(expected = "unknown object bogus")]
fn unknown_object_panics() {
    replay("object bogus 00000000-0000-0000-c000000000000046");
}
