use crate::path::ObjectPath;
use crate::{error_code_text, facility_code_text};

#[rustfmt::skip]
const PATH_ROWS: &[(&str, &str)] = &[
    (r#"\\srv\root\cimv2:Win32_X.Name="a""#, r#"set=00000000 info=20c1a server=[srv] class=[Win32_X] ns=2[root][cimv2] keys=1[Name="a"] t0=28[root\cimv2:Win32_X.Name="a"] t1=1[] t2=17[Win32_X.Name="a"] t4=34[\\srv\root\cimv2:Win32_X.Name="a"] t8=17[\\srv\root\cimv2] t16=11[root\cimv2] t32=34[\\srv\root\cimv2:Win32_X.Name="a"]"#),
    (r#"//srv/root:Cls"#, r#"set=00000000 info=20c16 server=[srv] class=[Cls] ns=1[root] keys=0 t0=9[root:Cls] t1=1[] t2=4[Cls] t4=15[\\srv\root:Cls] t8=11[\\srv\root] t16=5[root] t32=15[//srv/root:Cls]"#),
    (r#"\\.\root\cimv2:Win32_Service"#, r#"set=00000000 info=20c15 server=[.] class=[Win32_Service] ns=2[root][cimv2] keys=0 t0=25[root\cimv2:Win32_Service] t1=1[] t2=14[Win32_Service] t4=29[\\.\root\cimv2:Win32_Service] t8=15[\\.\root\cimv2] t16=11[root\cimv2] t32=29[\\.\root\cimv2:Win32_Service]"#),
    (r#"\\srv"#, r#"set=00000000 info=20c16 server=[srv] class=[] ns=0 keys=0 t0=1[] t1=1[] t2=1[] t4=6[\\srv] t8=6[\\srv] t16=1[] t32=6[\\srv]"#),
    (r#"\\"#, r#"set=00000000 info=20c16 server=[] class=[] ns=0 keys=0 t0=1[] t1=1[] t2=1[] t4=4[\\.] t8=4[\\.] t16=1[] t32=3[\\]"#),
    (r#"\root\cimv2:Win32_Process"#, r#"set=00000000 info=c15 server=- class=[Win32_Process] ns=2[root][cimv2] keys=0 t0=25[root\cimv2:Win32_Process] t1=1[] t2=14[Win32_Process] t4=29[\\.\root\cimv2:Win32_Process] t8=15[\\.\root\cimv2] t16=11[root\cimv2] t32=26[\root\cimv2:Win32_Process]"#),
    (r#"root\:Cls"#, r#"set=00000000 info=c15 server=- class=[Cls] ns=2[root][] keys=0 t0=10[root\:Cls] t1=1[] t2=4[Cls] t4=14[\\.\root\:Cls] t8=10[\\.\root\] t16=6[root\] t32=10[root\:Cls]"#),
    (r#"Win32_Process"#, r#"set=00000000 info=c15 server=- class=[Win32_Process] ns=0 keys=0 t0=15[:Win32_Process] t1=1[] t2=14[Win32_Process] t4=18[\\.:Win32_Process] t8=4[\\.] t16=1[] t32=14[Win32_Process]"#),
    (r#"root:Win32_Process"#, r#"set=00000000 info=c15 server=- class=[root:Win32_Process] ns=0 keys=0 t0=20[:root:Win32_Process] t1=1[] t2=19[root:Win32_Process] t4=23[\\.:root:Win32_Process] t8=4[\\.] t16=1[] t32=19[root:Win32_Process]"#),
    (r#":Win32_Process"#, r#"set=00000000 info=c15 server=- class=[Win32_Process] ns=0 keys=0 t0=15[:Win32_Process] t1=1[] t2=14[Win32_Process] t4=18[\\.:Win32_Process] t8=4[\\.] t16=1[] t32=15[:Win32_Process]"#),
    (r#"\"#, r#"set=00000000 info=c15 server=- class=[] ns=1[] keys=0 t0=1[] t1=1[] t2=1[] t4=5[\\.\] t8=5[\\.\] t16=1[] t32=2[\]"#),
    (r#"Win32_Process.A=1,B=2,C=3"#, r#"set=00000000 info=c19 server=- class=[Win32_Process] ns=0 keys=3[A=1][B=2][C=3] t0=27[:Win32_Process.A=1,B=2,C=3] t1=1[] t2=26[Win32_Process.A=1,B=2,C=3] t4=30[\\.:Win32_Process.A=1,B=2,C=3] t8=4[\\.] t16=1[] t32=26[Win32_Process.A=1,B=2,C=3]"#),
    (r#"Win32_Process=5"#, r#"set=00000000 info=c99 server=- class=[Win32_Process] ns=0 keys=1[=5] t0=18[:Win32_Process=5] t1=1[] t2=17[Win32_Process=5] t4=21[\\.:Win32_Process=5] t8=4[\\.] t16=1[] t32=16[Win32_Process=5]"#),
    (r#"Win32_Process.=5,b=6"#, r#"set=00000000 info=c99 server=- class=[Win32_Process] ns=0 keys=2[=5][b=6] t0=22[:Win32_Process=5,b=6] t1=1[] t2=21[Win32_Process=5,b=6] t4=25[\\.:Win32_Process=5,b=6] t8=4[\\.] t16=1[] t32=21[Win32_Process.=5,b=6]"#),
    (r#"Win32_Process=5,b=6"#, r#"set=00000000 info=c99 server=- class=[Win32_Process] ns=0 keys=1[=5] t0=18[:Win32_Process=5] t1=1[] t2=17[Win32_Process=5] t4=21[\\.:Win32_Process=5] t8=4[\\.] t16=1[] t32=20[Win32_Process=5,b=6]"#),
    (r#"Win32_Process.A=1,,B=2"#, r#"set=80041008 info=4001 server=- class=- ns=0 keys=0 t0=1[] t1=1[] t2=1[] t4=4[\\.] t8=4[\\.] t16=1[] t32=1[]"#),
    (r#"Win32_Process="#, r#"set=80041008 info=4001 server=- class=- ns=0 keys=0 t0=1[] t1=1[] t2=1[] t4=4[\\.] t8=4[\\.] t16=1[] t32=1[]"#),
    (r#"Win32_X.K="a\b""#, r#"set=00000000 info=c15 server=- class=[] ns=2[Win32_X.K="a][b"] keys=0 t0=16[Win32_X.K="a\b"] t1=1[] t2=1[] t4=20[\\.\Win32_X.K="a\b"] t8=20[\\.\Win32_X.K="a\b"] t16=16[Win32_X.K="a\b"] t32=16[Win32_X.K="a\b"]"#),
    (r#""#, r#"set=00000000 info=4001 server=- class=- ns=0 keys=0 t0=1[] t1=1[] t2=1[] t4=4[\\.] t8=4[\\.] t16=1[] t32=1[]"#),
];

#[rustfmt::skip]
const STATUS_ROWS: &[(&str, &str)] = &[
    (r#"0"#, r#"err=[Error code: 0x00000000] fac=[Facility code: 0x00000000]"#),
    (r#"80070005"#, r#"err=[Error code: 0x80070005] fac=[Facility code: 0x80070005]"#),
    (r#"8004103A"#, r#"err=[Error code: 0x8004103a] fac=[Facility code: 0x8004103a]"#),
    (r#"FFFFFFFF"#, r#"err=[Error code: 0xffffffff] fac=[Facility code: 0xffffffff]"#),
];

#[test]
fn path_rows_match_oracle() {
    for &(input, expected) in PATH_ROWS {
        assert_eq!(replay(input), expected, "input [{input}]");
    }
}

#[test]
fn status_rows_match_oracle() {
    for &(input, expected) in STATUS_ROWS {
        assert_eq!(replay_status(input), expected, "input [{input}]");
    }
}

/// The line harness_path.c prints for one SetText input.
fn replay(input: &str) -> String {
    let units: Vec<u16> = input.encode_utf16().collect();
    let mut path = ObjectPath::default();
    let hr = match path.set_text(1, Some(&units)) {
        Ok(()) => 0,
        Err(code) => code,
    };
    let info = path.get_info(0).unwrap();
    let mut line = format!(
        "set={:08x} info={:x} server={} class={}",
        hr as u32,
        info,
        opt(path.server.as_deref()),
        opt(path.class.as_deref()),
    );
    line += &format!(" ns={}", path.get_namespace_count());
    for namespace in &path.namespaces {
        line += &format!("[{}]", String::from_utf16_lossy(namespace));
    }
    line += &format!(" keys={}", path.get_key_count());
    for key in &path.keys {
        line += &format!(
            "[{}={}]",
            String::from_utf16_lossy(&key.name),
            String::from_utf16_lossy(&key.value)
        );
    }
    for flag in [0, 1, 2, 4, 8, 16, 32] {
        let mut buf = [0u16; 1024];
        let len = path.get_text(flag, buf.len(), Some(&mut buf[..])).unwrap();
        line += &format!(" t{flag}={len}[{}]", cut(&buf));
    }
    line
}

fn opt(value: Option<&[u16]>) -> String {
    match value {
        Some(units) => format!("[{}]", String::from_utf16_lossy(units)),
        None => "-".to_string(),
    }
}

fn cut(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

/// The line harness_status.c prints for one HRESULT given in hex.
fn replay_status(input: &str) -> String {
    let res = u32::from_str_radix(input, 16).unwrap() as i32;
    format!(
        "err=[{}] fac=[{}]",
        error_code_text(res),
        facility_code_text(res)
    )
}
