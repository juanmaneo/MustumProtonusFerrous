//! Oracle rows for the registrar script engine.
//!
//! Each row is the output of `target/canon/atl/oracle` for one script: the verbatim Wine
//! `registrar.c` and `wcs.inc` bodies, run against a fake registry that logs every call.
//! All 25 rows ran clean under `-fsanitize=address,undefined`, so none is excluded.

use super::*;

struct Row {
    script: &'static str,
    register: bool,
    hresult: u32,
    log: &'static [&'static str],
}

const ROWS: &[Row] = &[
    // nested_keys
    Row {
        script: r#"HKCR { Foo { Bar { } } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"create HKEY_CLASSES_ROOT\Foo\Bar ok",
            r"close HKEY_CLASSES_ROOT\Foo\Bar",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // val_sz_with_space
    Row {
        script: r#"HKCR { Foo { val Name = s 'hi there' } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo Name sz=18:hi there ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // default_sz
    Row {
        script: r#"HKCR { Foo = s 'Default' }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo @ sz=16:Default ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // sz_doubled_quote
    Row {
        script: r#"HKCR { Foo = s 'It''s' }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo @ sz=10:It's ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // quoted_key_name
    Row {
        script: r#"HKCR { 'Quoted Key' { } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Quoted Key ok",
            r"close HKEY_CLASSES_ROOT\Quoted Key",
        ],
    },
    // binary_hex
    Row {
        script: r#"HKCR { Foo { val Bin = b 0aFf10 } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo Bin bin=3:0aff10 ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // binary_bad_digit
    Row {
        script: r#"HKCR { Foo { val Bin = b abc } }"#,
        register: true,
        hresult: 0x80004005,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"close HKEY_CLASSES_ROOT\Foo",
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"delete_key HKEY_CLASSES_ROOT\Foo",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // dword_plain
    Row {
        script: r#"HKCR { Foo { val N = d 123 } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo N dword=0x0000007b ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // dword_clamp_high
    Row {
        script: r#"HKCR { Foo { val N = d 4294967296 } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo N dword=0xffffffff ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // dword_negative
    Row {
        script: r#"HKCR { Foo { val N = d -5 } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo N dword=0xfffffffb ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // noremove_forceremove
    Row {
        script: r#"HKCR { NoRemove Foo { } ForceRemove Bar { } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"close HKEY_CLASSES_ROOT\Foo",
            r"delete_tree HKEY_CLASSES_ROOT\Bar",
            r"create HKEY_CLASSES_ROOT\Bar ok",
            r"close HKEY_CLASSES_ROOT\Bar",
        ],
    },
    // unreg_delete
    Row {
        script: r#"HKCR { Delete Foo }"#,
        register: false,
        hresult: 0x00000000,
        log: &[],
    },
    // unreg_dword_minus
    Row {
        script: r#"HKCR { Foo { val N = d -7 } }"#,
        register: false,
        hresult: 0x00000000,
        log: &[
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"delete_key HKEY_CLASSES_ROOT\Foo",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // unreg_missing_open
    Row {
        script: r#"HKCR { Missing { Child { } } }"#,
        register: false,
        hresult: 0x00000000,
        log: &[
            r"open HKEY_CLASSES_ROOT\Missing err 2",
            r"open <null>\Child err 6",
            r"delete_key <null>\Child",
            r"delete_key HKEY_CLASSES_ROOT\Missing",
        ],
    },
    // module_macro
    Row {
        script: r#"HKCR { %MODULE% { } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\app.dll ok",
            r"close HKEY_CLASSES_ROOT\app.dll",
        ],
    },
    // value_name_macro
    Row {
        script: r#"HKCR { Foo { val %NAME%%% = s 'x' } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo Widget% sz=4:x ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // unknown_macro
    Row {
        script: r#"HKCR { %Nope% { } }"#,
        register: true,
        hresult: 0x80020009,
        log: &[],
    },
    // val_without_value
    Row {
        script: r#"HKCR { Foo { val Name } }"#,
        register: true,
        hresult: 0x80020009,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"close HKEY_CLASSES_ROOT\Foo",
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // bad_value_type
    Row {
        script: r#"HKCR { Foo { val N = q 1 } }"#,
        register: true,
        hresult: 0x80020009,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"close HKEY_CLASSES_ROOT\Foo",
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"delete_key HKEY_CLASSES_ROOT\Foo",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // create_denied
    Row {
        script: r#"HKCR { Good { } Denied { } }"#,
        register: true,
        hresult: 0x80070005,
        log: &[
            r"create HKEY_CLASSES_ROOT\Good ok",
            r"close HKEY_CLASSES_ROOT\Good",
            r"create HKEY_CLASSES_ROOT\Denied err 5",
            r"open HKEY_CLASSES_ROOT\Good ok",
            r"delete_key HKEY_CLASSES_ROOT\Good",
            r"close HKEY_CLASSES_ROOT\Good",
            r"open HKEY_CLASSES_ROOT\Denied ok",
            r"delete_key HKEY_CLASSES_ROOT\Denied",
            r"close HKEY_CLASSES_ROOT\Denied",
        ],
    },
    // set_denied
    Row {
        script: r#"HKCR { Foo { val Denied = s 'x' } }"#,
        register: true,
        hresult: 0x80070005,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo ok",
            r"set HKEY_CLASSES_ROOT\Foo Denied sz=4:x err 5",
            r"close HKEY_CLASSES_ROOT\Foo",
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"delete_key HKEY_CLASSES_ROOT\Foo",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // delete_is_not_value
    Row {
        script: r#"HKCR { Delete Foo = s 'x' }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"delete_tree HKEY_CLASSES_ROOT\Foo",
            r"create HKEY_CLASSES_ROOT\= ok",
            r"close HKEY_CLASSES_ROOT\=",
            r"create HKEY_CLASSES_ROOT\s ok",
            r"close HKEY_CLASSES_ROOT\s",
            r"create HKEY_CLASSES_ROOT\x ok",
            r"close HKEY_CLASSES_ROOT\x",
        ],
    },
    // unreg_forceremove
    Row {
        script: r#"HKCR { ForceRemove Foo { } }"#,
        register: false,
        hresult: 0x00000000,
        log: &[
            r"open HKEY_CLASSES_ROOT\Foo ok",
            r"delete_key HKEY_CLASSES_ROOT\Foo",
            r"close HKEY_CLASSES_ROOT\Foo",
        ],
    },
    // hkcu_case_fold
    Row {
        script: r#"hkcu { NoRemove foo { } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CURRENT_USER\foo ok",
            r"close HKEY_CURRENT_USER\foo",
        ],
    },
    // glued_brace
    Row {
        script: r#"HKCR { Foo{ } }"#,
        register: true,
        hresult: 0x00000000,
        log: &[
            r"create HKEY_CLASSES_ROOT\Foo{ ok",
            r"close HKEY_CLASSES_ROOT\Foo{",
        ],
    },
];

/// The predefined root names in `RootKey` order, as the oracle prints them.
const ROOT_NAMES: [&str; 7] = [
    "HKEY_CLASSES_ROOT",
    "HKEY_CURRENT_USER",
    "HKEY_LOCAL_MACHINE",
    "HKEY_USERS",
    "HKEY_PERFORMANCE_DATA",
    "HKEY_DYN_DATA",
    "HKEY_CURRENT_CONFIG",
];

/// The oracle's fake registry. A handle is an index into `paths`, and the first seven
/// entries are the predefined roots. `log` receives each call in the oracle's format.
pub(crate) struct Mock {
    paths: Vec<String>,
    pub(crate) log: Vec<String>,
}

impl Mock {
    pub(crate) fn new() -> Self {
        Mock {
            paths: ROOT_NAMES.iter().map(|name| name.to_string()).collect(),
            log: Vec::new(),
        }
    }

    fn path(&self, key: Option<usize>) -> String {
        match key {
            Some(index) => self.paths[index].clone(),
            None => "<null>".to_string(),
        }
    }

    fn join(&self, parent: Option<usize>, name: &[u16]) -> String {
        let base = self.path(parent);
        let name = text(name);
        if name.is_empty() {
            base
        } else {
            format!("{base}\\{name}")
        }
    }

    /// Shared by `create_key` and `open_key`. A NULL parent fails with 6, and the child
    /// named by `failing` fails with its own code.
    fn open_node(
        &mut self,
        verb: &str,
        parent: Option<usize>,
        name: &[u16],
        failing: (&str, u32),
    ) -> Result<usize, u32> {
        let path = self.join(parent, name);
        let code = match parent {
            None => 6,
            Some(_) if text(name) == failing.0 => failing.1,
            Some(_) => 0,
        };
        self.log.push(format!("{verb} {path} {}", status(code)));
        if code != 0 {
            return Err(code);
        }
        self.paths.push(path);
        Ok(self.paths.len() - 1)
    }
}

impl Registry for Mock {
    type Key = usize;

    fn root(&self, root: RootKey) -> usize {
        root as usize
    }

    fn create_key(&mut self, parent: Option<usize>, name: &[u16]) -> Result<usize, u32> {
        self.open_node("create", parent, name, ("Denied", 5))
    }

    fn open_key(&mut self, parent: Option<usize>, name: &[u16]) -> Result<usize, u32> {
        self.open_node("open", parent, name, ("Missing", 2))
    }

    fn close_key(&mut self, key: usize) {
        let path = self.path(Some(key));
        self.log.push(format!("close {path}"));
    }

    fn delete_key(&mut self, parent: Option<usize>, name: &[u16]) {
        let path = self.join(parent, name);
        self.log.push(format!("delete_key {path}"));
    }

    fn delete_tree(&mut self, parent: Option<usize>, name: &[u16]) {
        let path = self.join(parent, name);
        self.log.push(format!("delete_tree {path}"));
    }

    fn set_value(
        &mut self,
        key: Option<usize>,
        name: Option<&[u16]>,
        value: &Value,
    ) -> Result<(), u32> {
        let kp = self.path(key);
        let vn = name.map_or_else(|| "@".to_string(), text);
        let body = match value {
            Value::Sz(data) => format!("sz={}:{}", (data.len() + 1) * 2, text(data)),
            Value::Dword(number) => format!("dword=0x{number:08x}"),
            Value::Binary(bytes) => format!("bin={}:{}", bytes.len(), hex(bytes)),
        };
        let code = match key {
            None => 6,
            Some(_) if name.is_some_and(|name| text(name) == "Denied") => 5,
            Some(_) => 0,
        };
        self.log
            .push(format!("set {kp} {vn} {body} {}", status(code)));
        if code == 0 {
            Ok(())
        } else {
            Err(code)
        }
    }
}

/// `to_ascii` in the oracle: the units before the first NUL, with each non-ASCII unit
/// shown as `?`.
pub(crate) fn text(units: &[u16]) -> String {
    units
        .iter()
        .take_while(|&&unit| unit != 0)
        .map(|&unit| {
            if unit < 0x80 {
                char::from(unit as u8)
            } else {
                '?'
            }
        })
        .collect()
}

fn status(code: u32) -> String {
    if code == 0 {
        "ok".to_string()
    } else {
        format!("err {code}")
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

/// The replacements the oracle registers. `NAME` is added twice, so `Widget` wins.
pub(crate) fn oracle_registrar() -> Registrar {
    let mut registrar = Registrar::default();
    registrar.add_replacement(&wide("MODULE"), &wide("app.dll"));
    registrar.add_replacement(&wide("NAME"), &wide("Old"));
    registrar.add_replacement(&wide("NAME"), &wide("Widget"));
    registrar
}

/// Runs `script` against a fresh `Mock`, returning the HRESULT and the logged calls.
pub(crate) fn run_script(
    registrar: &Registrar,
    script: &str,
    register: bool,
) -> (u32, Vec<String>) {
    let mut reg = Mock::new();
    let result = registrar.string_register(&mut reg, &wide(script), register);
    (hresult_of(result), reg.log)
}

pub(crate) fn hresult_of(result: Result<(), Error>) -> u32 {
    match result {
        Ok(()) => 0,
        Err(error) => error.hresult(),
    }
}

#[test]
fn oracle_registrar_rows() {
    let registrar = oracle_registrar();
    for row in ROWS {
        let (hresult, log) = run_script(&registrar, row.script, row.register);
        assert_eq!(hresult, row.hresult, "hresult of {}", row.script);
        assert_eq!(log, row.log, "log of {}", row.script);
    }
}
