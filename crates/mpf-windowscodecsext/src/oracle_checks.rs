//! Oracle-derived test vectors for the ported windowscodecsext logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies of `DllGetClassObject`,
//! `WICCreateColorTransform_Proxy` and `IWICColorTransform_Initialize_Proxy_W`
//! (`dlls/windowscodecsext/main.c`) with `gcc -fsanitize=address,undefined -g` under
//! `target/canon/windowscodecsext/`, and running the harness. All 22 printed rows ran clean
//! under ASan and UBSan, so none is excluded.

use super::*;

/// The transformer the fake factory hands out. The harness writes a non-NULL object pointer;
/// the port writes this marker, so `out_written` means "the out-parameter was set".
const OBJECT: u32 = 0x7777;

/// The fake COM runtime. `create_color_transformer` writes `out` only when it succeeds, as
/// the harness does.
struct Host {
    init: HRESULT,
    create: HRESULT,
    ct: HRESULT,
    log: Vec<&'static str>,
}

impl ColorTransformHost<u32> for Host {
    fn co_initialize_ex(&mut self) -> HRESULT {
        self.log.push("CoInitializeEx");
        self.init
    }

    fn co_create_imaging_factory(&mut self) -> HRESULT {
        self.log.push("CoCreateInstance");
        self.create
    }

    fn create_color_transformer(&mut self, out: &mut u32) -> HRESULT {
        self.log.push("CreateColorTransformer");
        if succeeded(self.ct) {
            *out = OBJECT;
        }
        self.ct
    }

    fn release_imaging_factory(&mut self) {
        self.log.push("Release");
    }

    fn co_uninitialize(&mut self) {
        self.log.push("CoUninitialize");
    }
}

/// Runs `WICCreateColorTransform_Proxy` on the fake host. Returns the HRESULT, whether the
/// out-parameter was written, and the comma-separated call log the harness prints.
fn create(out_set: bool, init: HRESULT, create: HRESULT, ct: HRESULT) -> (HRESULT, bool, String) {
    let mut host = Host {
        init,
        create,
        ct,
        log: Vec::new(),
    };
    let mut out = 0;
    let ret = wic_create_color_transform_proxy(&mut host, out_set.then_some(&mut out));
    (ret, out == OBJECT, host.log.join(","))
}

/// The fake vtable. It records the forwarded arguments in the harness's `Initialize(...)` form.
struct Vtbl {
    ret: HRESULT,
    log: Vec<String>,
}

impl ColorTransformVtable<usize> for Vtbl {
    fn initialize(
        &mut self,
        source: usize,
        ctx_source: usize,
        ctx_dest: usize,
        pixel_fmt_dest: usize,
    ) -> HRESULT {
        self.log.push(format!(
            "Initialize(src={source:x},cs={ctx_source:x},cd={ctx_dest:x},fmt={pixel_fmt_dest:x})"
        ));
        self.ret
    }
}

#[test]
fn oracle_dll_get_class_object_rows() {
    // rows 01-04: the arguments are only traced, and the return value is always
    // CLASS_E_CLASSNOTAVAILABLE.
    for _ in 0..4 {
        assert_eq!(dll_get_class_object() as u32, 0x8004_0111);
    }
}

#[test]
fn oracle_wic_create_color_transform_proxy_rows() {
    // (out set, init, create, ct) -> (ret, out written, call log)
    let rows: [(bool, u32, u32, u32, u32, bool, &str); 14] = [
        // row 05
        (false, 0, 0, 0, 0x8007_0057, false, ""),
        // row 06
        (false, 1, 0, 0, 0x8007_0057, false, ""),
        // row 07
        (
            true,
            0,
            0,
            0,
            0,
            true,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release,CoUninitialize",
        ),
        // row 08
        (
            true,
            1,
            0,
            0,
            0,
            true,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release,CoUninitialize",
        ),
        // row 09
        (
            true,
            0x8001_0106,
            0,
            0,
            0,
            true,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release",
        ),
        // row 10
        (
            true,
            0,
            0x8000_4002,
            0,
            0x8000_4002,
            false,
            "CoInitializeEx,CoCreateInstance,CoUninitialize",
        ),
        // row 11
        (
            true,
            0x8001_0106,
            0x8000_4002,
            0,
            0x8000_4002,
            false,
            "CoInitializeEx,CoCreateInstance",
        ),
        // row 12
        (
            true,
            0,
            0,
            0x8000_4001,
            0x8000_4001,
            false,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release,CoUninitialize",
        ),
        // row 13
        (
            true,
            0x8001_0106,
            0,
            0x8007_000E,
            0x8007_000E,
            false,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release",
        ),
        // row 14
        (
            true,
            0,
            1,
            0,
            0,
            true,
            "CoInitializeEx,CoCreateInstance,CreateColorTransformer,Release,CoUninitialize",
        ),
        // row 15
        (
            true,
            0x8001_0106,
            0x8000_0000,
            0,
            0x8000_0000,
            false,
            "CoInitializeEx,CoCreateInstance",
        ),
        // row 16
        (false, 0x8001_0106, 0, 0, 0x8007_0057, false, ""),
        // row 17
        (
            true,
            0x8000_4005,
            0x8000_4005,
            0,
            0x8000_4005,
            false,
            "CoInitializeEx,CoCreateInstance",
        ),
        // row 18
        (
            true,
            1,
            0x8000_4005,
            0,
            0x8000_4005,
            false,
            "CoInitializeEx,CoCreateInstance,CoUninitialize",
        ),
    ];
    for (out_set, init, create_hr, ct, ret, out_written, log) in rows {
        let (got, written, calls) = create(out_set, init as i32, create_hr as i32, ct as i32);
        assert_eq!(got as u32, ret);
        assert_eq!(written, out_written);
        assert_eq!(calls, log);
    }
}

#[test]
fn oracle_initialize_proxy_w_rows() {
    // (src, cs, cd, fmt, vtable ret) -> (ret, log)
    let rows: [(usize, usize, usize, usize, u32, &str); 4] = [
        // row 19
        (
            0x1000,
            0x2000,
            0x3000,
            0x4000,
            0,
            "Initialize(src=1000,cs=2000,cd=3000,fmt=4000)",
        ),
        // row 20
        (0, 0, 0, 0, 0x8000_4001, "Initialize(src=0,cs=0,cd=0,fmt=0)"),
        // row 21
        (
            0x1000,
            0,
            0x3000,
            0x4000,
            0x8007_0057,
            "Initialize(src=1000,cs=0,cd=3000,fmt=4000)",
        ),
        // row 22
        (
            0x10,
            0x20,
            0x30,
            0x40,
            1,
            "Initialize(src=10,cs=20,cd=30,fmt=40)",
        ),
    ];
    for (src, cs, cd, fmt, ret, log) in rows {
        let mut vtbl = Vtbl {
            ret: ret as i32,
            log: Vec::new(),
        };
        let got = initialize_proxy_w(&mut vtbl, src, cs, cd, fmt);
        assert_eq!(got as u32, ret);
        assert_eq!(vtbl.log.join(","), log);
    }
}
