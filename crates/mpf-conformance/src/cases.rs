//! Seeded conformance cases, one per ported behavior. Names follow wine's
//! `dlls/<dll>/tests` conventions where applicable.

use crate::Case;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NONCE: AtomicU64 = AtomicU64::new(0);

fn temp_dir(tag: &str) -> std::path::PathBuf {
    let n = NONCE.fetch_add(1, Ordering::Relaxed);
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("mpf-conf-{tag}-{t}-{n}"))
}

fn w(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn shlwapi_path_helpers() {
    let path = w("dir\\file.txt");
    assert_eq!(mpf_shlwapi::path_find_file_name(&path), 4);
    assert_eq!(mpf_shlwapi::path_find_extension(&path), 8);
    assert!(mpf_shlwapi::path_is_relative(&w("file://folder/test.txt")));
    assert!(!mpf_shlwapi::path_is_relative(&w("C:\\test.txt")));

    let unc = w("\\\\server\\share");
    assert!(mpf_shlwapi::path_is_unc(&unc));
    assert!(mpf_shlwapi::path_is_unc_server_share(&unc));
    assert!(mpf_shlwapi::path_is_root(&unc));
    assert_eq!(mpf_shlwapi::path_skip_root(&w("C:\\folder")), Some(3));

    let mut added = w("one\\two");
    assert_eq!(mpf_shlwapi::path_add_backslash(&mut added), Some(8));
    assert_eq!(added, w("one\\two\\"));
    let mut removed = w("C:\\folder\\file.txt");
    assert!(mpf_shlwapi::path_remove_file_spec(&mut removed));
    assert_eq!(removed, w("C:\\folder"));
    let mut stripped = w("short//path\\file.txt");
    mpf_shlwapi::path_strip_path(&mut stripped);
    assert_eq!(stripped, w("file.txt"));

    assert_eq!(mpf_shlwapi::path_get_drive_number(&w("C:\\folder")), 2);
    assert!(mpf_shlwapi::path_is_file_spec(&w("file.txt")));
    assert!(mpf_shlwapi::path_is_lfn_file_spec(&w("long file name.txt")));
    let mut no_extension = w("file.txt");
    mpf_shlwapi::path_remove_extension(&mut no_extension);
    assert_eq!(no_extension, w("file"));
    let mut quoted = w("a path\\file.txt");
    mpf_shlwapi::path_quote_spaces(&mut quoted);
    mpf_shlwapi::path_unquote_spaces(&mut quoted);
    assert_eq!(quoted, w("a path\\file.txt"));

    let is_z_drive = |drive: i32| drive == 25;
    assert!(mpf_shlwapi::path_is_network_path(Some(&unc), is_z_drive));
    assert!(mpf_shlwapi::path_is_network_path(
        Some(&w("Z:\\data")),
        is_z_drive
    ));
    let mut decorated = w("file[1].txt");
    mpf_shlwapi::path_undecorate(&mut decorated);
    assert_eq!(decorated, w("file.txt"));
    let mut slashed = w("C:/dir/sub");
    mpf_shlwapi::fix_slashes_and_colon(&mut slashed);
    assert_eq!(slashed, w("C:\\dir\\sub"));
}

fn shlwapi_string_helpers() {
    let s = w("Testing String");
    assert_eq!(mpf_shlwapi::str_chr(&s, b'S' as u16), Some(8));
    assert_eq!(mpf_shlwapi::str_chr_n(&s, b'r' as u16, 10), None);
    assert_eq!(mpf_shlwapi::str_chr_n(&s, b't' as u16, 10), Some(3));

    let deadbeef = w("DeAdBeEf");
    assert_eq!(mpf_shlwapi::str_str(&deadbeef, &w("AdBe")), Some(2));
    assert_eq!(mpf_shlwapi::str_str(&deadbeef, &w("adbe")), None);
    assert_eq!(mpf_shlwapi::str_str_n(&deadbeef, &w("BeEf"), 4), None);
    assert_eq!(mpf_shlwapi::str_str_n(&deadbeef, &w("BeEf"), 5), Some(4));

    let mut buf = [0u16; 8];
    assert!(mpf_shlwapi::str_cpy(&mut buf, &w("Test")));
    assert!(mpf_shlwapi::str_cat(&mut buf, &w("!")));
    assert_eq!(buf[..6], w("Test!")[..]);
    mpf_shlwapi::str_ncat(&mut buf, &w("XYZ"), 2);
    assert_eq!(buf[..7], w("Test!X")[..]);

    let mut accel = w("&File");
    assert_eq!(mpf_shlwapi::sh_strip_mneumonic(&mut accel), b'F' as u16);
    assert_eq!(accel, w("File"));

    assert_eq!(mpf_shlwapi::str_to_int(&w("-55")), -55);
    assert_eq!(
        mpf_shlwapi::str_to_int64_ex(&w("-0x44fd"), mpf_shlwapi::STIF_SUPPORT_HEX),
        Some(0x44fd)
    );
    assert_eq!(mpf_shlwapi::str_to_int_ex(&w("+ 88987"), 0), None);

    let mut cpy = [0xffffu16; 8];
    assert_eq!(mpf_shlwapi::str_cpy_nx(&mut cpy, Some(&w("hello")), 8), 5);
    assert_eq!(cpy[..6], w("hello")[..]);

    let mut chain = [0x11u16; 33];
    assert_eq!(
        mpf_shlwapi::str_cat_chain(&mut chain, 32, Some(0), Some(&w("DeAd"))),
        4
    );
    assert_eq!(
        mpf_shlwapi::str_cat_chain(&mut chain, 32, None, Some(&w("BeEf"))),
        8
    );
    assert_eq!(chain[..9], w("DeAdBeEf")[..]);

    assert_eq!(mpf_shlwapi::str_spn(Some(&w("abc1")), Some(&w("cba"))), 3);
    assert_eq!(mpf_shlwapi::str_cspn(Some(&w("abc1")), Some(&w("1"))), 3);
    assert_eq!(
        mpf_shlwapi::str_pbrk(Some(&w("hello")), Some(&w("lo"))),
        Some(2)
    );
    assert_eq!(mpf_shlwapi::str_dup(Some(&w("dup"))), w("dup"));
    let mut cat_buf = [0u16; 8];
    assert!(mpf_shlwapi::str_cpy(&mut cat_buf, &w("abc")));
    mpf_shlwapi::str_cat_buff(Some(&mut cat_buf), &w("defgh"), 8);
    assert_eq!(cat_buf[..], w("abcdefg")[..]);
}

fn shlwapi_guid_and_map() {
    assert_eq!(mpf_shlwapi::sh_search_map_int(&[1, 2], &[10, 20], 2), 20);
    assert_eq!(mpf_shlwapi::sh_search_map_int(&[1, 2], &[10, 20], 3), -1);

    let text = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
    let guid = mpf_shlwapi::guid_from_string_w(Some(&text)).unwrap();
    let mut buf = [0u16; 39];
    assert_eq!(mpf_shlwapi::sh_string_from_guid(&guid, &mut buf), Some(39));
    assert_eq!(buf[..], text[..]);
    assert_eq!(
        mpf_shlwapi::sh_string_from_guid(&guid, &mut buf[..38]),
        None
    );
}

fn kernelbase_strip_prefix() {
    let mut legacy_backslash = w("C:\\folder");
    assert_eq!(
        mpf_kernelbase::path_add_backslash(&mut legacy_backslash),
        Some(10)
    );
    assert_eq!(legacy_backslash, w("C:\\folder\\"));
    let mut with_backslash = w("C:\\folder");
    assert_eq!(
        mpf_kernelbase::path_cch_add_backslash(&mut with_backslash, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(with_backslash, w("C:\\folder\\"));
    assert_eq!(
        mpf_kernelbase::path_find_file_name(&w("C:\\folder\\file.txt")),
        10
    );
    assert_eq!(
        mpf_kernelbase::path_find_extension(&w("C:\\folder\\file.txt")),
        14
    );
    assert_eq!(
        mpf_kernelbase::path_cch_find_extension(&w("C:\\folder\\file.txt"), 64),
        (mpf_kernelbase::S_OK, Some(14))
    );
    let mut legacy_extension = w("C:\\folder\\file");
    assert!(mpf_kernelbase::path_add_extension(
        &mut legacy_extension,
        &w(".txt")
    ));
    assert_eq!(legacy_extension, w("C:\\folder\\file.txt"));
    mpf_kernelbase::path_remove_extension(&mut legacy_extension);
    assert_eq!(legacy_extension, w("C:\\folder\\file"));
    let mut quoted_path = w("\"C:\\folder\\file.txt\"");
    mpf_kernelbase::path_unquote_spaces(&mut quoted_path);
    assert_eq!(quoted_path, w("C:\\folder\\file.txt"));
    let mut legacy_renamed = w("C:\\folder\\file.old");
    assert!(mpf_kernelbase::path_rename_extension(
        &mut legacy_renamed,
        &w(".new")
    ));
    assert_eq!(legacy_renamed, w("C:\\folder\\file.new"));
    let mut extension = w("C:\\folder\\file.txt");
    assert_eq!(
        mpf_kernelbase::path_cch_remove_extension(&mut extension, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(extension, w("C:\\folder\\file"));
    let mut renamed = w("C:\\folder\\file.old");
    assert_eq!(
        mpf_kernelbase::path_cch_rename_extension(&mut renamed, 64, &w("new")),
        mpf_kernelbase::S_OK
    );
    assert_eq!(renamed, w("C:\\folder\\file.new"));
    let mut with_extension = w("C:\\folder\\file.old");
    assert_eq!(
        mpf_kernelbase::path_cch_add_extension(&mut with_extension, 64, &w(".new")),
        mpf_kernelbase::S_FALSE
    );
    assert_eq!(with_extension, w("C:\\folder\\file.old"));
    assert!(mpf_kernelbase::path_cch_is_root(&w("C:\\")));
    assert!(mpf_kernelbase::path_is_root(&w("C:\\")));
    assert!(!mpf_kernelbase::path_cch_is_root(&w("C:\\folder")));
    assert!(mpf_kernelbase::path_cch_is_root(&w("\\\\server\\share")));
    assert_eq!(
        mpf_kernelbase::path_is_unc_ex(&w("\\\\server\\share")),
        (true, Some(2))
    );
    assert_eq!(
        mpf_kernelbase::path_is_unc_ex(&w("\\\\?\\UNC\\server")),
        (true, Some(8))
    );
    assert_eq!(
        mpf_kernelbase::path_is_unc_ex(&w("C:\\path")),
        (false, None)
    );
    assert!(mpf_kernelbase::path_is_relative(&w("file.txt")));
    assert!(!mpf_kernelbase::path_is_file_spec(&w("C:\\file.txt")));
    assert!(mpf_kernelbase::path_is_file_spec(&w("file.txt")));
    assert_eq!(mpf_kernelbase::path_get_drive_number(&w("C:\\file.txt")), 2);
    assert_eq!(
        mpf_kernelbase::path_get_drive_number(&w("\\\\?\\C:\\file.txt")),
        2
    );
    assert_eq!(
        mpf_kernelbase::path_common_prefix(&w("C:\\one\\a"), &w("c:\\one\\b")),
        6
    );
    assert!(mpf_kernelbase::path_is_prefix(&w("C:\\"), &w("C:\\folder")));
    assert!(mpf_kernelbase::path_is_same_root(
        &w("C:\\one"),
        &w("C:\\two")
    ));
    assert!(mpf_kernelbase::path_is_url(Some(&w(
        "https://example.test/path"
    ))));
    assert!(!mpf_kernelbase::path_is_url(Some(&w("not a url"))));
    assert_eq!(
        mpf_kernelbase::path_canonicalize(Some(&w("C:\\one\\.\\..\\two\\three\\.."))),
        Some(w("C:\\two"))
    );
    assert_eq!(
        mpf_kernelbase::path_canonicalize(Some(&w(""))),
        Some(w("\\"))
    );
    assert_eq!(mpf_kernelbase::path_canonicalize(None), None);
    assert_eq!(
        mpf_kernelbase::path_combine(Some(&w("C:\\a")), Some(&w("..\\b"))),
        Some(w("C:\\b"))
    );
    let mut appended = w("C:\\a");
    assert!(mpf_kernelbase::path_append(
        Some(&mut appended),
        Some(&w("\\b"))
    ));
    assert_eq!(appended, w("C:\\a\\b"));
    assert!(!mpf_kernelbase::path_is_relative(&w("C:\\file.txt")));
    assert!(!mpf_kernelbase::path_is_relative(&w("\\rooted")));
    assert!(mpf_kernelbase::path_is_unc(&w("\\\\server\\share")));
    assert!(mpf_kernelbase::path_is_unc_server(&w("\\\\server")));
    assert!(!mpf_kernelbase::path_is_unc_server(&w("\\\\server\\share")));
    assert!(mpf_kernelbase::path_is_unc_server_share(&w(
        "\\\\server\\share"
    )));
    assert!(!mpf_kernelbase::path_is_unc_server_share(&w(
        "\\\\server\\share\\file"
    )));
    let mut legacy_path = w("C:\\folder\\file.txt");
    assert!(mpf_kernelbase::path_remove_file_spec(&mut legacy_path));
    assert_eq!(legacy_path, w("C:\\folder"));
    let mut legacy_trailing = w("C:\\folder\\");
    assert_eq!(
        mpf_kernelbase::path_remove_backslash(&mut legacy_trailing),
        9
    );
    assert_eq!(legacy_trailing, w("C:\\folder"));
    let mut trailing = w("C:\\folder\\");
    assert_eq!(
        mpf_kernelbase::path_cch_remove_backslash(&mut trailing, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(trailing, w("C:\\folder"));
    let mut trailing_ex = w("C:\\folder\\");
    assert_eq!(
        mpf_kernelbase::path_cch_remove_backslash_ex(&mut trailing_ex, 64),
        (mpf_kernelbase::S_OK, Some(9), 55)
    );
    assert_eq!(trailing_ex, w("C:\\folder"));
    let mut root = w("C:\\folder\\file");
    assert_eq!(
        mpf_kernelbase::path_cch_strip_to_root(&mut root, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(root, w("C:\\"));
    assert_eq!(
        mpf_kernelbase::path_get_char_type(b'\\' as u16),
        mpf_kernelbase::GCT_SEPARATOR
    );
    assert_eq!(
        mpf_kernelbase::path_get_char_type(b'?' as u16),
        mpf_kernelbase::GCT_WILD
    );
    assert_eq!(
        mpf_kernelbase::path_get_char_type(b'A' as u16),
        mpf_kernelbase::GCT_SHORTCHAR | mpf_kernelbase::GCT_LFNCHAR
    );
    let mut quoted_path = w("C:\\Program Files\\app.exe");
    mpf_kernelbase::path_quote_spaces(&mut quoted_path);
    assert_eq!(quoted_path, w("\"C:\\Program Files\\app.exe\""));
    let mut icon_path = w("\" C:\\Program Files\\app.exe \",5");
    assert_eq!(mpf_kernelbase::path_parse_icon_location(&mut icon_path), 5);
    assert_eq!(icon_path, w("C:\\Program Files\\app.exe"));
    assert!(mpf_kernelbase::path_is_lfn_file_spec(Some(&w(
        "program-file.exe"
    ))));
    assert!(!mpf_kernelbase::path_is_lfn_file_spec(Some(&w("PROG.EXE"))));
    assert!(mpf_kernelbase::path_is_valid_char(
        b'?' as u16,
        mpf_kernelbase::PATH_CHAR_CLASS_LETTER
    ));
    assert!(!mpf_kernelbase::path_is_valid_char(
        b'?' as u16,
        mpf_kernelbase::PATH_CHAR_CLASS_OTHER_VALID
    ));
    assert_eq!(
        mpf_kernelbase::path_find_next_component(&w("C:\\folder\\file.txt")),
        Some(3)
    );
    assert_eq!(
        mpf_kernelbase::path_get_args(&w("\"C:\\Program Files\\app.exe\" --help")),
        27
    );
    assert_eq!(mpf_kernelbase::path_skip_root(&w("C:\\folder")), Some(3));
    assert_eq!(
        mpf_kernelbase::path_skip_root(&w("\\\\server\\share\\file")),
        Some(15)
    );
    let mut stripped_path = w("C:\\folder\\file.txt");
    mpf_kernelbase::path_strip_path(&mut stripped_path);
    assert_eq!(stripped_path, w("file.txt"));
    let mut legacy_root = w("C:\\folder\\file");
    assert!(mpf_kernelbase::path_strip_to_root(&mut legacy_root));
    assert_eq!(legacy_root, w("C:\\"));
    let mut legacy_blanks = w("  C:\\folder\\file  ");
    mpf_kernelbase::path_remove_blanks(&mut legacy_blanks);
    assert_eq!(legacy_blanks, w("C:\\folder\\file"));
    let mut file_spec = w("C:\\folder\\file.txt");
    assert_eq!(
        mpf_kernelbase::path_cch_remove_file_spec(&mut file_spec, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(file_spec, w("C:\\folder"));
    assert_eq!(
        mpf_kernelbase::path_cch_skip_root(&w("C:\\folder")),
        (mpf_kernelbase::S_OK, 3)
    );
    assert_eq!(
        mpf_kernelbase::path_cch_skip_root(&w("\\\\server\\share\\file")),
        (mpf_kernelbase::S_OK, 15)
    );

    let mut drive = w("\\\\?\\C:\\folder");
    assert_eq!(
        mpf_kernelbase::path_cch_strip_prefix(&mut drive, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(drive, w("C:\\folder"));

    let mut unc = w("\\\\?\\UNC\\server\\share");
    assert_eq!(
        mpf_kernelbase::path_cch_strip_prefix(&mut unc, 64),
        mpf_kernelbase::S_OK
    );
    assert_eq!(unc, w("\\\\server\\share"));
}

fn msvcrt_wide_compare() {
    // mirrors dlls/msvcrt/tests/string.c test_wcscmp: sign-only expectations
    assert_eq!(mpf_msvcrt::wcscmp(&w("a"), &w("z")).signum(), -1);
    assert_eq!(mpf_msvcrt::wcscmp(&w("z"), &w("a")).signum(), 1);
    assert_eq!(mpf_msvcrt::wcscmp(&w("f"), &w("f")), 0);
    assert_eq!(mpf_msvcrt::wcslen(&w("hello")), 5);
    assert_eq!(mpf_msvcrt::wcsicmp(&w("HeLLo"), &w("hello")), 0);
}

fn oleaut32_narrowing() {
    // mirrors dlls/oleaut32/tests/vartype.c CONVERTRANGE/OVERFLOWRANGE bounds
    assert_eq!(mpf_oleaut32::var_i1_from_i2(-128), Ok(-128));
    assert_eq!(
        mpf_oleaut32::var_i1_from_i2(128),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(mpf_oleaut32::var_i1_from_ui1(127), Ok(127));
    assert_eq!(
        mpf_oleaut32::var_i1_from_ui1(128),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(
        mpf_oleaut32::var_ui1_from_i1(-1),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
    assert_eq!(mpf_oleaut32::var_ui1_from_i2(255), Ok(255));
    assert_eq!(
        mpf_oleaut32::var_ui1_from_i2(256),
        Err(mpf_oleaut32::DISP_E_OVERFLOW)
    );
}

fn rpcrt4_uuid_string_roundtrip() {
    // mirrors dlls/rpcrt4/tests/rpc.c test_UuidFromString
    let text = "00112233-4455-6677-8899-aabbccddeeff";
    let uuid = mpf_rpcrt4::from_string(Some(text)).unwrap();
    assert_eq!(mpf_rpcrt4::to_string(Some(&uuid)), text);
    assert_eq!(
        mpf_rpcrt4::from_string(Some("g0112233-4455-6677-8899-aabbccddeeff")),
        Err(mpf_rpcrt4::UuidError)
    );
    assert_eq!(
        mpf_rpcrt4::from_string(None),
        Ok(mpf_rpcrt4::Uuid::default())
    );
}

fn rpc_uuid(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> mpf_rpcrt4::Uuid {
    mpf_rpcrt4::Uuid {
        data1,
        data2,
        data3,
        data4,
    }
}

fn rpcrt4_uuid_equal_grid() {
    // mirrors dlls/rpcrt4/tests/rpc.c test_UuidEqual (Uuid_Table / Uuid_Comparison_Grid)
    let table = [
        rpc_uuid(0x0000_0000, 0x0000, 0x0000, [0x00; 8]),
        rpc_uuid(
            0xdead_beef,
            0xdead,
            0xbeef,
            [0x10, 0x21, 0x35, 0x56, 0x89, 0xa0, 0xf4, 0x8a],
        ),
        rpc_uuid(
            0xabad_feed,
            0x49ff,
            0xbead,
            [0x8a, 0xf4, 0xa0, 0x89, 0x56, 0x35, 0x21, 0x10],
        ),
        rpc_uuid(
            0x93da_375c,
            0x1324,
            0x1355,
            [0x87, 0xff, 0x49, 0x44, 0x34, 0x44, 0x22, 0x19],
        ),
        rpc_uuid(
            0xdead_beef,
            0xdead,
            0xbeef,
            [0x10, 0x21, 0x35, 0x56, 0x89, 0xa0, 0xf4, 0x8b],
        ),
        rpc_uuid(
            0x9bad_feed,
            0x49ff,
            0xbead,
            [0x8a, 0xf4, 0xa0, 0x89, 0x56, 0x35, 0x21, 0x10],
        ),
        rpc_uuid(0x0000_0000, 0x0001, 0x0000, [0x00; 8]),
        rpc_uuid(0x0000_0000, 0x0000, 0x0000, [0, 0, 0, 0, 0, 0, 0, 1]),
        rpc_uuid(
            0x1231_2312,
            0x1231,
            0x1231,
            [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xff],
        ),
        rpc_uuid(0x1111_1111, 0x1111, 0x1111, [0x11; 8]),
    ];
    // index 10 is NULL, which `table.get` yields as None
    #[rustfmt::skip]
    let grid: [[bool; 11]; 11] = [
        [true,  false, false, false, false, false, false, false, false, false, true ],
        [false, true,  false, false, false, false, false, false, false, false, false],
        [false, false, true,  false, false, false, false, false, false, false, false],
        [false, false, false, true,  false, false, false, false, false, false, false],
        [false, false, false, false, true,  false, false, false, false, false, false],
        [false, false, false, false, false, true,  false, false, false, false, false],
        [false, false, false, false, false, false, true,  false, false, false, false],
        [false, false, false, false, false, false, false, true,  false, false, false],
        [false, false, false, false, false, false, false, false, true,  false, false],
        [false, false, false, false, false, false, false, false, false, true,  false],
        [true,  false, false, false, false, false, false, false, false, false, true ],
    ];
    for (i1, row) in grid.iter().enumerate() {
        for (i2, expected) in row.iter().enumerate() {
            assert_eq!(
                mpf_rpcrt4::uuid_equal(table.get(i1), table.get(i2)),
                *expected,
                "UuidEqual({i1}, {i2})"
            );
        }
    }
}

fn ole32_thread_com_state() {
    // mirrors dlls/ole32/compobj.c CoGetState/CoSetState semantics
    mpf_ole32::co_set_state(None);
    assert!(mpf_ole32::co_get_state().is_none());
    mpf_ole32::co_set_state(Some(std::rc::Rc::new(String::from("apartment state"))));
    assert_eq!(
        mpf_ole32::co_get_state()
            .unwrap()
            .downcast_ref::<String>()
            .unwrap(),
        "apartment state"
    );
    mpf_ole32::co_set_state(None);
    assert!(mpf_ole32::is_equal_guid(None, Some(&[0; 16])));
}

fn ole32_guid_strings() {
    // mirrors dlls/combase/combase.c CLSIDFromString, IIDFromString and StringFromGUID2
    let text = w("{01020304-0506-0708-090A-0B0C0D0E0F0A}");
    let clsid = mpf_ole32::clsid_from_string(Some(&text)).unwrap();
    assert_eq!(mpf_ole32::clsid_from_string(None), Some(Default::default()));
    assert_eq!(mpf_ole32::iid_from_string(Some(&text)), Ok(clsid));
    assert_eq!(
        mpf_ole32::iid_from_string(Some(&w("{01020304-0506}"))),
        Err(mpf_ole32::IidStringError::InvalidArg)
    );
    let mut buf = [0u16; 39];
    assert_eq!(mpf_ole32::string_from_guid2(Some(&clsid), &mut buf), 39);
    assert_eq!(buf[..], text[..]);
}

fn shell32_pidl_layout() {
    // mirrors dlls/shell32/pidl.c ILGetSize/ILGetNext/ILFindLastID
    let pidl = [3, 0, 0xaa, 4, 0, 0xbb, 0xcc, 0, 0];
    assert_eq!(mpf_shell32::il_get_size(&pidl), Some(9));
    assert_eq!(
        mpf_shell32::il_combine(Some(&[3, 0, 0xaa, 0, 0]), Some(&[4, 0, 0xbb, 0xcc, 0, 0])),
        Some(pidl.to_vec())
    );
    assert_eq!(mpf_shell32::il_clone(&pidl), Some(pidl.to_vec()));
    assert_eq!(
        mpf_shell32::il_clone_first(&pidl),
        Some(vec![3, 0, 0xaa, 0, 0])
    );
    assert_eq!(mpf_shell32::il_get_next(&pidl, 0), Some(3));
    assert_eq!(mpf_shell32::il_find_last_id(&pidl), Some(3));
    assert_eq!(mpf_shell32::il_get_size(&[0, 0]), Some(2));
    let mut removable = pidl;
    assert!(mpf_shell32::il_remove_last_id(&mut removable));
    assert_eq!(mpf_shell32::il_get_size(&removable), Some(5));
}

fn ucrtbase_onexit_table() {
    // mirrors dlls/ucrtbase/tests/misc.c on-exit table expectations
    let mut table = mpf_ucrtbase::OnExitTable::new();
    assert_eq!(mpf_ucrtbase::initialize_onexit_table(None), -1);
    assert_eq!(mpf_ucrtbase::register_onexit_function(None, None), -1);
    assert_eq!(
        mpf_ucrtbase::register_onexit_function(Some(&mut table), None),
        0
    );
    assert_eq!(mpf_ucrtbase::execute_onexit_table(Some(&mut table)), 0);
    assert!(table.is_empty());
}

fn kernel32_lstrlen_semantics() {
    // mirrors dlls/kernel32/tests/string.c lstrlenW expectations
    assert_eq!(
        mpf_kernel32::lstrlen_w(&w("hello")),
        5,
        "lstrlen_w miscounts"
    );
}

fn kernel32_lstrcpyn_truncation() {
    let src = w("abcdef");
    let mut dst = [0u16; 4];
    let n = mpf_kernel32::lstrcpyn_w(&mut dst, &src);
    assert_eq!(n, 6, "lstrcpyn_w returns the source length");
    assert_eq!(
        dst,
        [b'a' as u16, b'b' as u16, b'c' as u16, 0],
        "lstrcpyn_w truncates and terminates"
    );
}

fn ntdll_rtl_compare() {
    use mpf_ntdll::unicode::*;
    let a = WideString::from_utf8("abc");
    let b = WideString::from_utf8("abd");
    assert!(rtl_compare_unicode_strings(&a, &b) < 0, "compare wrong");
    assert!(
        rtl_compare_unicode_strings(&a, &WideString::from_utf8("ABD")) > 0,
        "compare ordering wrong"
    );
    assert!(
        rtl_equal_unicode_string_ignore_case(&a, &WideString::from_utf8("ABC")),
        "ignore-case equal wrong"
    );
    assert!(
        !rtl_equal_unicode_string(&a, &WideString::from_utf8("ABC")),
        "exact equal wrong"
    );
}

fn ntdll_ansi_roundtrip() {
    use mpf_ntdll::unicode::*;
    let wide = WideString::from_utf8("abc");
    let ansi = rtl_unicode_string_to_ansi_string(&wide);
    assert_eq!(ansi, b"abc", "unicode -> ansi wrong");
    assert_eq!(
        rtl_ansi_string_to_unicode_string(&ansi),
        wide,
        "ansi -> unicode wrong"
    );
}

fn ntdll_heap() {
    use mpf_ntdll::heap::*;
    let heap = rtl_process_heap();
    let p = rtl_allocate_heap(heap, 128, true);
    assert!(!p.is_null(), "alloc failed");
    let bytes = unsafe { std::slice::from_raw_parts(p, 128) };
    assert!(bytes.iter().all(|&b| b == 0), "not zeroed");
    assert_eq!(
        rtl_free_heap(heap, p),
        mpf_ntdll::ntstatus::STATUS_SUCCESS,
        "free failed"
    );
}

fn ntdll_critical_section() {
    use mpf_ntdll::critical_section::CriticalSection;
    let cs = CriticalSection::new();
    let _g1 = cs.enter();
    let _g2 = cs.enter();
    // same thread: recursive try must succeed
    assert!(cs.try_enter().is_some(), "recursive try_enter failed");
}

fn ntdll_object_handles() {
    use mpf_ntdll::ntstatus::{is_success, STATUS_INVALID_HANDLE};
    use mpf_ntdll::object::ObjectManager;
    let mut om = ObjectManager::new();
    let h = om.allocate_handle(3);
    assert_eq!(om.query_object(h), Some(3), "handle query wrong");
    assert!(is_success(om.close_handle(h)), "handle close wrong");
    assert_eq!(
        om.close_handle(h),
        STATUS_INVALID_HANDLE,
        "double close wrong"
    );
}

fn ntdll_exception_guard() {
    use mpf_ntdll::exception::{guard, EXCEPTION_RUST_PANIC};
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = guard(|| -> u32 { panic!("boom") });
    std::panic::set_hook(prev);
    assert_eq!(
        r.unwrap_err().code,
        EXCEPTION_RUST_PANIC,
        "panic not converted"
    );
}

fn pe_parse_headers() {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).unwrap();
    assert_eq!(pe.machine(), mpf_pe::MACHINE_AMD64, "machine wrong");
    assert!(pe.is_64bit(), "not PE32+");
    assert_eq!(pe.size_of_image(), 0x2000, "size of image wrong");
}

fn pe_tables() {
    let data = mpf_pe::testing::build_test_pe64();
    let pe = mpf_pe::PeImage::parse(&data).unwrap();
    let exports = pe.exports().unwrap();
    let imports = pe.imports().unwrap();
    let relocs = pe.relocations().unwrap();
    let delay = pe.delay_imports().unwrap();
    assert_eq!(exports.len(), 1, "exports wrong");
    assert_eq!(exports[0].name, "TestFunc", "export name wrong");
    assert_eq!(imports.len(), 1, "imports wrong");
    assert_eq!(imports[0].function, "BaseFunc", "import name wrong");
    assert_eq!(relocs.len(), 1, "relocs wrong");
    assert_eq!(relocs[0].entries.len(), 1, "reloc entries wrong");
    assert_eq!(relocs[0].entries[0].reloc_type, 10, "reloc type wrong");
    assert!(pe.tls_directory().unwrap().is_some(), "tls missing");
    assert_eq!(delay.len(), 1, "delay wrong");
    assert_eq!(delay[0].dll, "DELAY.dll", "delay dll wrong");
}

fn pe_mapped() {
    let file = mpf_pe::testing::build_test_pe64();
    let mut mapped = vec![0u8; 0x2000];
    mapped[..0x200].copy_from_slice(&file[..0x200]);
    mapped[0x1000..0x1300].copy_from_slice(&file[0x200..0x500]);
    let pe = mpf_pe::PeImage::parse_mapped(&mapped).unwrap();
    assert_eq!(pe.exports().unwrap()[0].rva, 0x1080, "mapped exports wrong");
}

fn loader_pipeline() {
    let data = mpf_pe::testing::build_test_pe64();
    let mut m = mpf_loader::map_image(&data).unwrap();
    assert_eq!(m.relocate(0x5000_0000), 1, "reloc count wrong");
    let mut resolver = |dll: &str, func: &str, _ord: Option<u16>| {
        (dll == "KERNEL32.dll" && func == "BaseFunc").then_some(0xDEAD_BEEF)
    };
    assert_eq!(m.resolve_imports(&mut resolver), 1, "import resolve wrong");
    let v = u64::from_le_bytes(m.image()[0x1170..0x1178].try_into().unwrap());
    assert_eq!(v, 0xDEAD_BEEF, "IAT not patched");
}

fn winnt_layouts() {
    use core::mem::offset_of;
    assert_eq!(offset_of!(mpf_winnt::Peb, ldr), 0x18, "PEB offset wrong");
    assert_eq!(
        offset_of!(mpf_winnt::Teb, process_environment_block),
        0x60,
        "TEB offset wrong"
    );
}

fn wineboot_prefix() {
    let root = temp_dir("prefix");
    let prefix = mpf_wineboot::init_prefix(&root).unwrap();
    let system = std::fs::read_to_string(prefix.system_reg()).unwrap();
    assert!(
        system.starts_with(mpf_advapi32::REG_HEADER) && system.contains("11.22"),
        "system.reg wrong"
    );
    mpf_advapi32::init_from_prefix(&root).unwrap();
    std::fs::remove_dir_all(&root).unwrap();
}

fn advapi32_registry() {
    use mpf_advapi32::{RegValue, Registry, RootKey};
    let mut reg = Registry::new();
    let key = reg
        .reg_create_key_ex_w(RootKey::LocalMachine, "Software\\MPF")
        .unwrap();
    assert_eq!(
        reg.reg_set_value_ex_w(key, "version", RegValue::String("11.22".into())),
        0,
        "set failed"
    );
    assert_eq!(
        reg.reg_query_value_ex_w(key, "version").unwrap(),
        RegValue::String("11.22".into()),
        "query wrong"
    );
}

fn kernel32_file() {
    use mpf_kernel32::file::*;
    let dir = temp_dir("file");
    std::fs::create_dir_all(&dir).unwrap();
    let path = w(&dir.join("data.txt").to_string_lossy());
    let h = create_file_w(&path, GENERIC_READ | GENERIC_WRITE, true);
    assert_ne!(h, INVALID_HANDLE_VALUE, "create failed");
    write_file(h, b"mpf").unwrap();
    close_handle(h);
    let h = create_file_w(&path, GENERIC_READ, false);
    let mut buf = [0u8; 8];
    let n = read_file(h, &mut buf).unwrap();
    close_handle(h);
    assert_eq!(&buf[..n], b"mpf", "content wrong");
    std::fs::remove_dir_all(&dir).unwrap();
}

fn kernel32_module() {
    use mpf_kernel32::module::*;
    let data = mpf_pe::testing::build_test_pe64();
    let module = mpf_loader::map_image(&data).unwrap();
    let base = module.base();
    let size = module.size_of_image;
    // The module table points into the mapping, so it must outlive the case.
    std::mem::forget(module);
    register_module("mpf_test.dll", base, size);
    set_current_exe(base);
    assert_eq!(
        get_module_handle_w(None) as usize,
        base,
        "current exe wrong"
    );
    assert_eq!(
        get_proc_address(base as *const core::ffi::c_void, "TestFunc"),
        Some(base + 0x1080),
        "get_proc_address wrong"
    );
}

fn kernel32_thread() {
    use mpf_kernel32::thread::*;
    extern "system" fn noop(_p: *mut core::ffi::c_void) -> u32 {
        0
    }
    let h = create_thread(noop, core::ptr::null_mut());
    assert_eq!(wait_for_thread(h, 0), WAIT_OBJECT_0, "wait wrong");
    assert_ne!(get_current_thread_id(), 0, "thread id 0");
}

fn kernel32_process() {
    use mpf_kernel32::process::*;
    assert_eq!(get_current_process_id(), std::process::id(), "pid wrong");
    assert_eq!(get_current_process() as isize, -1, "pseudo handle wrong");
}

/// All seeded cases.
pub fn all() -> Vec<Case> {
    vec![
        Case {
            name: "shlwapi!PathFindFileName/Extension/IsRelative/UNC/SkipRoot/RemoveFileSpecW",
            run: shlwapi_path_helpers,
        },
        Case {
            name:
                "shlwapi!StrChr/StrStr/StrCpy/StrCat/StrNCat/StrCpyNX/StrCatChain/SHStripMneumonicW",
            run: shlwapi_string_helpers,
        },
        Case {
            name: "shlwapi!SHSearchMapInt/SHStringFromGUIDW/GUIDFromStringW",
            run: shlwapi_guid_and_map,
        },
        Case {
            name: "kernelbase!PathCchStripPrefix",
            run: kernelbase_strip_prefix,
        },
        Case {
            name: "msvcrt!wcscmp/wcslen/_wcsicmp",
            run: msvcrt_wide_compare,
        },
        Case {
            name: "oleaut32!VarI1From*/VarUI1From* range checks",
            run: oleaut32_narrowing,
        },
        Case {
            name: "rpcrt4!UuidFromString/UuidToString roundtrip",
            run: rpcrt4_uuid_string_roundtrip,
        },
        Case {
            name: "rpcrt4!UuidEqual grid",
            run: rpcrt4_uuid_equal_grid,
        },
        Case {
            name: "ole32!CoGetState/CoSetState thread-local state",
            run: ole32_thread_com_state,
        },
        Case {
            name: "ole32!CLSIDFromString/IIDFromString/StringFromGUID2",
            run: ole32_guid_strings,
        },
        Case {
            name: "shell32!ILGetSize/ILGetNext/ILFindLastID PIDL layout",
            run: shell32_pidl_layout,
        },
        Case {
            name: "ucrtbase!_initialize/_register/_execute_onexit_table",
            run: ucrtbase_onexit_table,
        },
        Case {
            name: "kernel32!lstrlenW semantics",
            run: kernel32_lstrlen_semantics,
        },
        Case {
            name: "kernel32!lstrcpynW truncation",
            run: kernel32_lstrcpyn_truncation,
        },
        Case {
            name: "ntdll!RtlCompareUnicodeString/Equal",
            run: ntdll_rtl_compare,
        },
        Case {
            name: "ntdll!ANSI<->Unicode roundtrip",
            run: ntdll_ansi_roundtrip,
        },
        Case {
            name: "ntdll!RtlAllocateHeap/RtlFreeHeap",
            run: ntdll_heap,
        },
        Case {
            name: "ntdll!critical section recursion",
            run: ntdll_critical_section,
        },
        Case {
            name: "ntdll!object handle table",
            run: ntdll_object_handles,
        },
        Case {
            name: "ntdll!exception guard catches panic",
            run: ntdll_exception_guard,
        },
        Case {
            name: "pe!parse headers",
            run: pe_parse_headers,
        },
        Case {
            name: "pe!exports/imports/relocs/TLS/delay",
            run: pe_tables,
        },
        Case {
            name: "pe!parse_mapped export lookup",
            run: pe_mapped,
        },
        Case {
            name: "loader!map/relocate/resolve",
            run: loader_pipeline,
        },
        Case {
            name: "winnt!PEB/TEB x64 offsets",
            run: winnt_layouts,
        },
        Case {
            name: "wineboot!prefix + hives",
            run: wineboot_prefix,
        },
        Case {
            name: "advapi32!Reg* roundtrip",
            run: advapi32_registry,
        },
        Case {
            name: "kernel32!CreateFileW/ReadFile/WriteFile",
            run: kernel32_file,
        },
        Case {
            name: "kernel32!GetModuleHandle/GetProcAddress",
            run: kernel32_module,
        },
        Case {
            name: "kernel32!CreateThread/WaitForSingleObject",
            run: kernel32_thread,
        },
        Case {
            name: "kernel32!GetCurrentProcess(Id)",
            run: kernel32_process,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_cases_pass() {
        let cases = all();
        let (passed, failed) = crate::run(&cases);
        assert_eq!(failed, 0, "conformance failures");
        assert_eq!(passed, cases.len());
    }
}
