//! Oracle rows for `dlls/comctl32`. Each `ORACLE_*` line is what the harness in
//! `target/canon/comctl32` prints for one call sequence over the verbatim `dpa.c`, `dsa.c` and
//! `string.c` bodies, built with ASan and UBSan. Each row function runs the same sequence on the
//! port and prints the same line, and each test compares the two strings.

use std::cell::RefCell;
use std::fmt::Display;

use crate::{
    create_page, create_proxy_page, do_reader_mode, dpa_clone, dpa_clone_into, dpa_create,
    dpa_delete_all_ptrs, dpa_delete_ptr, dpa_destroy, dpa_destroy_callback, dpa_enum_callback,
    dpa_get_ptr, dpa_get_ptr_index, dpa_grow, dpa_insert_ptr, dpa_search, dpa_set_ptr, dpa_sort,
    dsa_create, dsa_delete_all_items, dsa_delete_item, dsa_destroy, dsa_destroy_callback,
    dsa_enum_callback, dsa_get_item, dsa_get_item_ptr, dsa_insert_item, dsa_set_item,
    sh_get_process_dword, str_get_ptr_a, str_get_ptr_w, str_set_ptr_a, str_set_ptr_w, Dpa, Dsa,
    StubException, DPAS_INSERTAFTER, DPAS_SORTED,
};

const ORACLE_STUB: &[&str] = &[
    "stub CreatePage code=0x80000100 flags=0x1 nparams=2 module=comctl32.dll function=CreatePage",
    "stub CreateProxyPage code=0x80000100 flags=0x1 nparams=2 module=comctl32.dll function=CreateProxyPage",
    "stub DoReaderMode code=0x80000100 flags=0x1 nparams=2 module=comctl32.dll function=DoReaderMode",
    "stub SHGetProcessDword code=0x80000100 flags=0x1 nparams=2 module=comctl32.dll function=SHGetProcessDword",
];

const ORACLE_DPA: &[&str] = &[
    "dpa_null_handle ret=[0,0,-1,-1,0,0,0,0,-1] count=- max=- grow=- slots=[] allocs=[] log=[]",
    "dpa_create_default ret=[1] count=0 max=16 grow=8 slots=[] allocs=[32,128] log=[]",
    "dpa_create_struct_fails ret=[0] count=- max=- grow=- slots=[] allocs=[32!] log=[]",
    "dpa_create_block_fails ret=[1] count=0 max=16 grow=8 slots=[] allocs=[32,128!] log=[]",
    "dpa_grow_20 ret=[1] count=0 max=20 grow=20 slots=[] allocs=[32,128,160] log=[]",
    "dpa_grow_alloc_fails ret=[0] count=0 max=16 grow=8 slots=[] allocs=[32,128,160!] log=[]",
    "dpa_insert_order ret=[0,0,1,3] count=4 max=16 grow=8 slots=[20,30,10,40] allocs=[32,128] log=[]",
    "dpa_set_ptr_grows ret=[1] count=21 max=24 grow=8 slots=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7] allocs=[32,128,192] log=[]",
    "dpa_set_ptr_within ret=[1] count=3 max=16 grow=8 slots=[1,9,3] allocs=[32,128] log=[]",
    "dpa_get_ptr ret=[1,3,0,0] count=3 max=16 grow=8 slots=[1,2,3] allocs=[32,128] log=[]",
    "dpa_get_ptr_index ret=[1,-1] count=3 max=16 grow=8 slots=[5,6,7] allocs=[32,128] log=[]",
    "dpa_get_ptr_block_fails ret=[0,-1] count=0 max=16 grow=8 slots=[] allocs=[32,128!] log=[]",
    "dpa_delete_ptr_shifts ret=[2] count=3 max=16 grow=8 slots=[1,3,4] allocs=[32,128,128] log=[]",
    "dpa_delete_ptr_shrinks ret=[3] count=40 max=40 grow=8 slots=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5] allocs=[32,128,384,320] log=[]",
    "dpa_delete_ptr_alloc_fails ret=[0] count=3 max=16 grow=8 slots=[1,3,4] allocs=[32,128,128!] log=[]",
    "dpa_delete_all_ptrs ret=[1] count=0 max=16 grow=8 slots=[] allocs=[32,128,128] log=[]",
    "dpa_delete_all_alloc_fails ret=[1] count=0 max=16 grow=8 slots=[] allocs=[32,128,128!] log=[]",
    "dpa_sort_five ret=[1] count=5 max=16 grow=8 slots=[1,2,3,4,5] allocs=[32,128] log=[5:2,2:4,5:4,1:3,2:1,2:3,4:3]",
    "dpa_search_sorted ret=[2,2,-1] count=4 max=16 grow=8 slots=[1,3,5,7] allocs=[32,128] log=[5:3,5:5,4:3,4:5,9:3,9:5,9:7]",
    "dpa_search_linear ret=[2,-1,-1] count=3 max=16 grow=8 slots=[5,2,8] allocs=[32,128] log=[8:5,8:2,8:8,9:5,9:2,9:8,5:2,5:8]",
    "dpa_enum_stops ret=[] count=3 max=16 grow=8 slots=[1,2,3] allocs=[32,128] log=[1,2]",
    "dpa_clone_null_dst ret=[1] count=3 max=8 grow=8 slots=[1,2,3] allocs=[32,128,32,64] log=[]",
    "dpa_clone_into_existing ret=[1] count=10 max=16 grow=8 slots=[1,2,3,4,5,6,7,8,9,10] allocs=[32,128,32,128,128] log=[]",
    "dpa_destroy_null ret=[0] allocs=[] log=[]",
    "dpa_destroy ret=[1] allocs=[32,128] log=[]",
    "dpa_destroy_callback ret=[] allocs=[32,128] log=[1,2]",
];

const ORACLE_DSA: &[&str] = &[
    "dsa_null_handle ret=[0,0,0,-1,-1,0] count=- max=- grow=- bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[] log=[]",
    "dsa_create ret=[1] count=0 max=0 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32] log=[]",
    "dsa_create_struct_fails ret=[0] count=- max=- grow=- bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32!] log=[]",
    "dsa_set_item_fresh ret=[1] count=1 max=2 grow=2 bytes=[1,2,3,4,0,0,0,0] buf=[88,88,88,88,88,88,88,88] allocs=[32,8] log=[]",
    "dsa_set_item_grow_gap ret=[1] count=6 max=6 grow=2 bytes=[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9,9,9,9] buf=[88,88,88,88,88,88,88,88] allocs=[32,24] log=[]",
    "dsa_set_item_alloc_fails ret=[0] count=0 max=0 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32,8!] log=[]",
    "dsa_set_item_negative_index ret=[0] count=0 max=0 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32] log=[]",
    "dsa_get_item ret=[1,0] count=2 max=2 grow=2 bytes=[1,2,3,4,5,6,7,8] buf=[5,6,7,8,88,88,88,88] allocs=[32,8] log=[]",
    "dsa_get_item_ptr ret=[1,0] count=2 max=2 grow=2 bytes=[1,2,3,4,5,6,7,8] buf=[5,6,7,8,88,88,88,88] allocs=[32,8] log=[]",
    "dsa_insert_front_mid ret=[0,0,1,3] count=4 max=4 grow=2 bytes=[2,2,2,2,3,3,3,3,1,1,1,1,4,4,4,4] buf=[88,88,88,88,88,88,88,88] allocs=[32,8,16] log=[]",
    "dsa_insert_alloc_fails ret=[-1] count=0 max=0 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32,8!] log=[]",
    "dsa_delete_item_shrinks ret=[0,0] count=4 max=4 grow=2 bytes=[0,0,0,0,0,0,0,0,0,0,0,0,9,9,9,9] buf=[88,88,88,88,88,88,88,88] allocs=[32,24,16] log=[]",
    "dsa_delete_item_alloc_fails ret=[0,-1] count=4 max=6 grow=2 bytes=[0,0,0,0,0,0,0,0,0,0,0,0,9,9,9,9,9,9,9,9,9,9,9,9] buf=[88,88,88,88,88,88,88,88] allocs=[32,24,16!] log=[]",
    "dsa_delete_all_items ret=[1] count=0 max=0 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32,16] log=[]",
    "dsa_set_item_zero_size ret=[1] count=1 max=2 grow=2 bytes=[] buf=[88,88,88,88,88,88,88,88] allocs=[32,0] log=[]",
    "dsa_enum_callback ret=[] count=3 max=4 grow=2 bytes=[1,0,0,0,2,0,0,0,3,0,0,0,0,0,0,0] buf=[88,88,88,88,88,88,88,88] allocs=[32,8,16] log=[1,2]",
    "dsa_destroy_null ret=[0] allocs=[] log=[]",
    "dsa_destroy ret=[1] allocs=[32,8] log=[]",
    "dsa_destroy_callback ret=[] allocs=[32,8,16] log=[1,2]",
];

const ORACLE_STR: &[&str] = &[
    "str_get_ptr_a_fits ret=[3] buf=[104,105,0,88,88,88,88,88] dst=- allocs=[]",
    "str_get_ptr_a_truncates ret=[4] buf=[104,101,108,0,88,88,88,88] dst=- allocs=[]",
    "str_get_ptr_a_size_query ret=[4] buf=- dst=- allocs=[]",
    "str_get_ptr_a_null_src ret=[0] buf=[0,88,88,88,88,88,88,88] dst=- allocs=[]",
    "str_set_ptr_a ret=[1] buf=- dst=[120,121,122,0] allocs=[4]",
    "str_set_ptr_a_null ret=[1] buf=- dst=NULL allocs=[4]",
    "str_set_ptr_a_alloc_fails ret=[0] buf=- dst=NULL allocs=[4!]",
    "str_get_ptr_w_fits ret=[2] buf=[104,105,0,22616,22616,22616,22616,22616] dst=- allocs=[]",
    "str_get_ptr_w_truncates ret=[3] buf=[104,101,108,0,22616,22616,22616,22616] dst=- allocs=[]",
    "str_get_ptr_w_size_query ret=[3] buf=- dst=- allocs=[]",
    "str_get_ptr_w_null_src ret=[0] buf=[0,22616,22616,22616,22616,22616,22616,22616] dst=- allocs=[]",
    "str_set_ptr_w ret=[1] buf=- dst=[97,98,0] allocs=[6]",
    "str_set_ptr_w_null ret=[1] buf=- dst=NULL allocs=[6]",
    "str_set_ptr_w_alloc_fails ret=[0] buf=- dst=NULL allocs=[6!]",
];

type RowFn = fn() -> String;

/// The allocations and comparisons of one row. `fail_at` is the 1-based allocation that fails,
/// or 0 for none. `asks` records each allocation size and whether it failed.
pub(crate) struct Row {
    fail_at: usize,
    asks: RefCell<Vec<(usize, bool)>>,
    trace: RefCell<Vec<String>>,
}

impl Row {
    pub(crate) fn new(fail_at: usize) -> Self {
        Self {
            fail_at,
            asks: RefCell::new(Vec::new()),
            trace: RefCell::new(Vec::new()),
        }
    }

    fn alloc(&self, size: usize) -> bool {
        let fail = self.asks.borrow().len() + 1 == self.fail_at;
        self.asks.borrow_mut().push((size, fail));
        !fail
    }

    fn note(&self, entry: impl Display) {
        self.trace.borrow_mut().push(entry.to_string());
    }

    fn cmp(&self, a: usize, b: usize) -> i32 {
        self.note(format!("{a}:{b}"));
        (a > b) as i32 - (a < b) as i32
    }

    pub(crate) fn allocs(&self) -> String {
        let asks: Vec<String> = self
            .asks
            .borrow()
            .iter()
            .map(|&(size, fail)| format!("{size}{}", if fail { "!" } else { "" }))
            .collect();
        format!("allocs=[{}]", asks.join(","))
    }

    pub(crate) fn tail(&self) -> String {
        format!("{} log=[{}]", self.allocs(), self.trace.borrow().join(","))
    }
}

pub(crate) fn alloc_fn(row: &Row) -> impl FnMut(usize) -> bool + '_ {
    move |size| row.alloc(size)
}

pub(crate) fn cmp_fn(row: &Row) -> impl FnMut(usize, usize) -> i32 + '_ {
    move |a, b| row.cmp(a, b)
}

pub(crate) fn dpa_enum_fn(row: &Row, stop: usize) -> impl FnMut(usize) -> bool + '_ {
    move |item| {
        row.note(item);
        item != stop
    }
}

pub(crate) fn dsa_enum_fn(row: &Row, stop: i32) -> impl FnMut(&[u8]) -> bool + '_ {
    move |item| {
        let value = i32::from_ne_bytes(item[..4].try_into().expect("4-byte item"));
        row.note(value);
        value != stop
    }
}

fn list(items: impl IntoIterator<Item = impl Display>) -> String {
    items
        .into_iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn dpa_line(name: &str, rets: &[i64], dpa: Option<&Dpa>, row: &Row) -> String {
    let state = match dpa {
        Some(dpa) => {
            let slots = if dpa.ptrs.is_empty() {
                String::new()
            } else {
                list(&dpa.ptrs[..dpa.item_count as usize])
            };
            format!(
                " count={} max={} grow={} slots=[{slots}]",
                dpa.item_count, dpa.max_count, dpa.grow
            )
        }
        None => " count=- max=- grow=- slots=[]".to_string(),
    };
    format!("{name} ret=[{}]{state} {}", list(rets), row.tail())
}

fn dsa_line(name: &str, rets: &[i64], dsa: Option<&Dsa>, buf: &[u8], row: &Row) -> String {
    let state = match dsa {
        Some(dsa) => format!(
            " count={} max={} grow={} bytes=[{}]",
            dsa.item_count,
            dsa.max_count,
            dsa.grow,
            list(&dsa.data)
        ),
        None => " count=- max=- grow=- bytes=[]".to_string(),
    };
    format!(
        "{name} ret=[{}]{state} buf=[{}] {}",
        list(rets),
        list(buf),
        row.tail()
    )
}

fn destroy_line(name: &str, rets: &[i64], row: &Row) -> String {
    format!("{name} ret=[{}] {}", list(rets), row.tail())
}

fn str_get_line<T: Display>(name: &str, rets: &[i64], buf: Option<&[T]>, row: &Row) -> String {
    let buf = match buf {
        Some(buf) => format!("[{}]", list(buf)),
        None => "-".to_string(),
    };
    format!(
        "{name} ret=[{}] buf={buf} dst=- {}",
        list(rets),
        row.allocs()
    )
}

fn str_set_line<T: Display>(name: &str, rets: &[i64], dst: Option<&[T]>, row: &Row) -> String {
    let dst = match dst {
        Some(dst) => format!("[{}]", list(dst)),
        None => "NULL".to_string(),
    };
    format!(
        "{name} ret=[{}] buf=- dst={dst} {}",
        list(rets),
        row.allocs()
    )
}

fn stub_line(name: &str, stub: StubException) -> String {
    format!(
        "{name} code=0x{:x} flags=0x{:x} nparams={} module={} function={}",
        stub.code, stub.flags, stub.number_parameters, stub.module, stub.function
    )
}

/// The oracle line whose first words are `key`.
fn oracle_row<'a>(oracle: &[&'a str], key: &str) -> &'a str {
    let prefix = format!("{key} ");
    oracle
        .iter()
        .copied()
        .find(|line| line.starts_with(&prefix))
        .expect(key)
}

fn check_rows(oracle: &[&str], rows: &[(&str, RowFn)]) {
    assert_eq!(oracle.len(), rows.len());
    for (key, row) in rows {
        assert_eq!(row(), oracle_row(oracle, key), "{key}");
    }
}

/// Inserts `values` at the end of `dpa`, in order.
fn push_all(dpa: &mut Option<Box<Dpa>>, values: &[usize], alloc: &mut dyn FnMut(usize) -> bool) {
    for &value in values {
        dpa_insert_ptr(dpa.as_deref_mut(), 99, value, alloc);
    }
}

pub(crate) fn dpa_of(values: &[usize], alloc: &mut dyn FnMut(usize) -> bool) -> Option<Box<Dpa>> {
    let mut dpa = dpa_create(8, alloc);
    push_all(&mut dpa, values, alloc);
    dpa
}

pub(crate) fn dsa_of(
    item_size: i32,
    grow: i32,
    alloc: &mut dyn FnMut(usize) -> bool,
) -> Option<Box<Dsa>> {
    dsa_create(item_size, grow, alloc)
}

fn row_stub_create_page() -> String {
    stub_line("stub CreatePage", create_page())
}

fn row_stub_create_proxy_page() -> String {
    stub_line("stub CreateProxyPage", create_proxy_page())
}

fn row_stub_do_reader_mode() -> String {
    stub_line("stub DoReaderMode", do_reader_mode())
}

fn row_stub_sh_get_process_dword() -> String {
    stub_line("stub SHGetProcessDword", sh_get_process_dword())
}

fn row_dpa_null_handle() -> String {
    let row = Row::new(0);
    let rets = [
        dpa_grow(None, 8, &mut alloc_fn(&row)) as i64,
        dpa_get_ptr(None, 0) as i64,
        dpa_get_ptr_index(None, 1) as i64,
        dpa_insert_ptr(None, 0, 1, &mut alloc_fn(&row)) as i64,
        dpa_set_ptr(None, 0, 1, &mut alloc_fn(&row)) as i64,
        dpa_delete_ptr(None, 0, &mut alloc_fn(&row)) as i64,
        dpa_delete_all_ptrs(None, &mut alloc_fn(&row)) as i64,
        dpa_sort(None, &mut cmp_fn(&row)) as i64,
        dpa_search(None, 1, 0, &mut cmp_fn(&row), 0) as i64,
    ];
    dpa_enum_callback(None, &mut dpa_enum_fn(&row, 0));
    dpa_line("dpa_null_handle", &rets, None, &row)
}

fn create_dpa_row(name: &str, fail_at: usize) -> String {
    let row = Row::new(fail_at);
    let dpa = dpa_create(8, &mut alloc_fn(&row));
    dpa_line(name, &[dpa.is_some() as i64], dpa.as_deref(), &row)
}

fn row_dpa_create_default() -> String {
    create_dpa_row("dpa_create_default", 0)
}

fn row_dpa_create_struct_fails() -> String {
    create_dpa_row("dpa_create_struct_fails", 1)
}

fn row_dpa_create_block_fails() -> String {
    create_dpa_row("dpa_create_block_fails", 2)
}

fn grow_dpa_row(name: &str, fail_at: usize) -> String {
    let row = Row::new(fail_at);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_create(8, &mut alloc);
    let ok = dpa_grow(dpa.as_deref_mut(), 20, &mut alloc);
    dpa_line(name, &[ok as i64], dpa.as_deref(), &row)
}

fn row_dpa_grow_20() -> String {
    grow_dpa_row("dpa_grow_20", 0)
}

fn row_dpa_grow_alloc_fails() -> String {
    grow_dpa_row("dpa_grow_alloc_fails", 3)
}

fn row_dpa_insert_order() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_create(8, &mut alloc);
    let rets = [
        dpa_insert_ptr(dpa.as_deref_mut(), 0, 10, &mut alloc),
        dpa_insert_ptr(dpa.as_deref_mut(), 0, 20, &mut alloc),
        dpa_insert_ptr(dpa.as_deref_mut(), 1, 30, &mut alloc),
        dpa_insert_ptr(dpa.as_deref_mut(), 99, 40, &mut alloc),
    ];
    dpa_line(
        "dpa_insert_order",
        &rets.map(i64::from),
        dpa.as_deref(),
        &row,
    )
}

fn row_dpa_set_ptr_grows() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_create(8, &mut alloc);
    let rets = [dpa_set_ptr(dpa.as_deref_mut(), 20, 7, &mut alloc) as i64];
    dpa_line("dpa_set_ptr_grows", &rets, dpa.as_deref(), &row)
}

fn row_dpa_set_ptr_within() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3], &mut alloc);
    let rets = [dpa_set_ptr(dpa.as_deref_mut(), 1, 9, &mut alloc) as i64];
    dpa_line("dpa_set_ptr_within", &rets, dpa.as_deref(), &row)
}

fn row_dpa_get_ptr() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[1, 2, 3], &mut alloc_fn(&row));
    let rets = [
        dpa_get_ptr(dpa.as_deref(), 0) as i64,
        dpa_get_ptr(dpa.as_deref(), 2) as i64,
        dpa_get_ptr(dpa.as_deref(), 3) as i64,
        dpa_get_ptr(dpa.as_deref(), -1) as i64,
    ];
    dpa_line("dpa_get_ptr", &rets, dpa.as_deref(), &row)
}

fn row_dpa_get_ptr_index() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[5, 6, 7], &mut alloc_fn(&row));
    let rets = [
        dpa_get_ptr_index(dpa.as_deref(), 6) as i64,
        dpa_get_ptr_index(dpa.as_deref(), 99) as i64,
    ];
    dpa_line("dpa_get_ptr_index", &rets, dpa.as_deref(), &row)
}

fn row_dpa_get_ptr_block_fails() -> String {
    let row = Row::new(2);
    let dpa = dpa_of(&[], &mut alloc_fn(&row));
    let rets = [
        dpa_get_ptr(dpa.as_deref(), 0) as i64,
        dpa_get_ptr_index(dpa.as_deref(), 1) as i64,
    ];
    dpa_line("dpa_get_ptr_block_fails", &rets, dpa.as_deref(), &row)
}

fn row_dpa_delete_ptr_shifts() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3, 4], &mut alloc);
    let rets = [dpa_delete_ptr(dpa.as_deref_mut(), 1, &mut alloc) as i64];
    dpa_line("dpa_delete_ptr_shifts", &rets, dpa.as_deref(), &row)
}

fn row_dpa_delete_ptr_shrinks() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[3], &mut alloc);
    dpa_set_ptr(dpa.as_deref_mut(), 40, 5, &mut alloc);
    let rets = [dpa_delete_ptr(dpa.as_deref_mut(), 0, &mut alloc) as i64];
    dpa_line("dpa_delete_ptr_shrinks", &rets, dpa.as_deref(), &row)
}

fn row_dpa_delete_ptr_alloc_fails() -> String {
    let row = Row::new(3);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3, 4], &mut alloc);
    let rets = [dpa_delete_ptr(dpa.as_deref_mut(), 1, &mut alloc) as i64];
    dpa_line("dpa_delete_ptr_alloc_fails", &rets, dpa.as_deref(), &row)
}

fn row_dpa_delete_all_ptrs() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3], &mut alloc);
    let rets = [dpa_delete_all_ptrs(dpa.as_deref_mut(), &mut alloc) as i64];
    dpa_line("dpa_delete_all_ptrs", &rets, dpa.as_deref(), &row)
}

fn row_dpa_delete_all_alloc_fails() -> String {
    let row = Row::new(3);
    let mut alloc = alloc_fn(&row);
    let mut dpa = dpa_of(&[1, 2, 3], &mut alloc);
    let rets = [dpa_delete_all_ptrs(dpa.as_deref_mut(), &mut alloc) as i64];
    dpa_line("dpa_delete_all_alloc_fails", &rets, dpa.as_deref(), &row)
}

fn row_dpa_sort_five() -> String {
    let row = Row::new(0);
    let mut dpa = dpa_of(&[5, 2, 4, 1, 3], &mut alloc_fn(&row));
    let rets = [dpa_sort(dpa.as_deref_mut(), &mut cmp_fn(&row)) as i64];
    dpa_line("dpa_sort_five", &rets, dpa.as_deref(), &row)
}

fn row_dpa_search_sorted() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[1, 3, 5, 7], &mut alloc_fn(&row));
    let rets = [
        dpa_search(dpa.as_deref(), 5, 0, &mut cmp_fn(&row), DPAS_SORTED) as i64,
        dpa_search(
            dpa.as_deref(),
            4,
            0,
            &mut cmp_fn(&row),
            DPAS_SORTED | DPAS_INSERTAFTER,
        ) as i64,
        dpa_search(dpa.as_deref(), 9, 0, &mut cmp_fn(&row), DPAS_SORTED) as i64,
    ];
    dpa_line("dpa_search_sorted", &rets, dpa.as_deref(), &row)
}

fn row_dpa_search_linear() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[5, 2, 8], &mut alloc_fn(&row));
    let rets = [
        dpa_search(dpa.as_deref(), 8, 0, &mut cmp_fn(&row), 0) as i64,
        dpa_search(dpa.as_deref(), 9, 0, &mut cmp_fn(&row), 0) as i64,
        dpa_search(dpa.as_deref(), 5, 1, &mut cmp_fn(&row), 0) as i64,
    ];
    dpa_line("dpa_search_linear", &rets, dpa.as_deref(), &row)
}

fn row_dpa_enum_stops() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[1, 2, 3], &mut alloc_fn(&row));
    dpa_enum_callback(dpa.as_deref(), &mut dpa_enum_fn(&row, 2));
    dpa_line("dpa_enum_stops", &[], dpa.as_deref(), &row)
}

fn row_dpa_clone_null_dst() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let src = dpa_of(&[1, 2, 3], &mut alloc);
    let clone = dpa_clone(src.as_deref(), &mut alloc);
    dpa_line(
        "dpa_clone_null_dst",
        &[clone.is_some() as i64],
        clone.as_deref(),
        &row,
    )
}

fn row_dpa_clone_into_existing() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let src = dpa_of(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &mut alloc);
    let mut dst = dpa_of(&[7], &mut alloc);
    let ok = dpa_clone_into(src.as_deref(), dst.as_deref_mut().expect("dst"), &mut alloc);
    dpa_line(
        "dpa_clone_into_existing",
        &[ok as i64],
        dst.as_deref(),
        &row,
    )
}

fn row_dpa_destroy_null() -> String {
    let row = Row::new(0);
    destroy_line("dpa_destroy_null", &[dpa_destroy(None) as i64], &row)
}

fn row_dpa_destroy() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[], &mut alloc_fn(&row));
    destroy_line("dpa_destroy", &[dpa_destroy(dpa) as i64], &row)
}

fn row_dpa_destroy_callback() -> String {
    let row = Row::new(0);
    let dpa = dpa_of(&[1, 2, 3], &mut alloc_fn(&row));
    dpa_destroy_callback(dpa, &mut dpa_enum_fn(&row, 2));
    destroy_line("dpa_destroy_callback", &[], &row)
}

fn create_dsa_row(name: &str, fail_at: usize) -> String {
    let row = Row::new(fail_at);
    let buf = [0x58u8; 8];
    let dsa = dsa_of(4, 2, &mut alloc_fn(&row));
    dsa_line(name, &[dsa.is_some() as i64], dsa.as_deref(), &buf, &row)
}

fn row_dsa_null_handle() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let rets = [
        dsa_get_item(None, 0, &mut buf) as i64,
        dsa_get_item_ptr(None, 0).is_some() as i64,
        dsa_set_item(None, 0, &[0; 4], &mut alloc_fn(&row)) as i64,
        dsa_insert_item(None, 0, &[0; 4], &mut alloc_fn(&row)) as i64,
        dsa_delete_item(None, 0, &mut alloc_fn(&row)) as i64,
        dsa_delete_all_items(None) as i64,
    ];
    dsa_enum_callback(None, &mut dsa_enum_fn(&row, 0));
    dsa_line("dsa_null_handle", &rets, None, &buf, &row)
}

fn row_dsa_create() -> String {
    create_dsa_row("dsa_create", 0)
}

fn row_dsa_create_struct_fails() -> String {
    create_dsa_row("dsa_create_struct_fails", 1)
}

fn row_dsa_set_item_fresh() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc) as i64];
    dsa_line("dsa_set_item_fresh", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_set_item_grow_gap() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [dsa_set_item(dsa.as_deref_mut(), 5, &[9; 4], &mut alloc) as i64];
    dsa_line("dsa_set_item_grow_gap", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_set_item_alloc_fails() -> String {
    let row = Row::new(2);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc) as i64];
    dsa_line(
        "dsa_set_item_alloc_fails",
        &rets,
        dsa.as_deref(),
        &buf,
        &row,
    )
}

fn row_dsa_set_item_negative_index() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [dsa_set_item(dsa.as_deref_mut(), -1, &[1, 2, 3, 4], &mut alloc) as i64];
    dsa_line(
        "dsa_set_item_negative_index",
        &rets,
        dsa.as_deref(),
        &buf,
        &row,
    )
}

fn row_dsa_get_item() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 1, &[5, 6, 7, 8], &mut alloc);
    let rets = [
        dsa_get_item(dsa.as_deref(), 1, &mut buf) as i64,
        dsa_get_item(dsa.as_deref(), 5, &mut buf) as i64,
    ];
    dsa_line("dsa_get_item", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_get_item_ptr() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 0, &[1, 2, 3, 4], &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 1, &[5, 6, 7, 8], &mut alloc);
    let mut rets = Vec::new();
    for index in [1, 2] {
        rets.push(match dsa_get_item_ptr(dsa.as_deref(), index) {
            Some(item) => {
                buf[..item.len()].copy_from_slice(item);
                1
            }
            None => 0,
        });
    }
    dsa_line("dsa_get_item_ptr", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_insert_front_mid() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [
        dsa_insert_item(dsa.as_deref_mut(), 0, &[1; 4], &mut alloc),
        dsa_insert_item(dsa.as_deref_mut(), 0, &[2; 4], &mut alloc),
        dsa_insert_item(dsa.as_deref_mut(), 1, &[3; 4], &mut alloc),
        dsa_insert_item(dsa.as_deref_mut(), 99, &[4; 4], &mut alloc),
    ];
    dsa_line(
        "dsa_insert_front_mid",
        &rets.map(i64::from),
        dsa.as_deref(),
        &buf,
        &row,
    )
}

fn row_dsa_insert_alloc_fails() -> String {
    let row = Row::new(2);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    let rets = [dsa_insert_item(dsa.as_deref_mut(), 0, &[1; 4], &mut alloc) as i64];
    dsa_line("dsa_insert_alloc_fails", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_delete_item_shrinks() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 5, &[9; 4], &mut alloc);
    let rets = [
        dsa_delete_item(dsa.as_deref_mut(), 0, &mut alloc) as i64,
        dsa_delete_item(dsa.as_deref_mut(), 0, &mut alloc) as i64,
    ];
    dsa_line("dsa_delete_item_shrinks", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_delete_item_alloc_fails() -> String {
    let row = Row::new(3);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 5, &[9; 4], &mut alloc);
    let rets = [
        dsa_delete_item(dsa.as_deref_mut(), 0, &mut alloc) as i64,
        dsa_delete_item(dsa.as_deref_mut(), 0, &mut alloc) as i64,
    ];
    dsa_line(
        "dsa_delete_item_alloc_fails",
        &rets,
        dsa.as_deref(),
        &buf,
        &row,
    )
}

fn row_dsa_delete_all_items() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 2, &[1; 4], &mut alloc);
    let rets = [dsa_delete_all_items(dsa.as_deref_mut()) as i64];
    dsa_line("dsa_delete_all_items", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_set_item_zero_size() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(0, 2, &mut alloc);
    let rets = [dsa_set_item(dsa.as_deref_mut(), 0, &[1; 4], &mut alloc) as i64];
    dsa_line("dsa_set_item_zero_size", &rets, dsa.as_deref(), &buf, &row)
}

fn row_dsa_enum_callback() -> String {
    let row = Row::new(0);
    let buf = [0x58u8; 8];
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    for index in 0..3 {
        dsa_set_item(
            dsa.as_deref_mut(),
            index,
            &(index + 1).to_ne_bytes(),
            &mut alloc,
        );
    }
    dsa_enum_callback(dsa.as_deref(), &mut dsa_enum_fn(&row, 2));
    dsa_line("dsa_enum_callback", &[], dsa.as_deref(), &buf, &row)
}

fn row_dsa_destroy_null() -> String {
    let row = Row::new(0);
    destroy_line("dsa_destroy_null", &[dsa_destroy(None) as i64], &row)
}

fn row_dsa_destroy() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    dsa_set_item(dsa.as_deref_mut(), 0, &1i32.to_ne_bytes(), &mut alloc);
    destroy_line("dsa_destroy", &[dsa_destroy(dsa) as i64], &row)
}

fn row_dsa_destroy_callback() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dsa = dsa_of(4, 2, &mut alloc);
    for index in 0..3 {
        dsa_set_item(
            dsa.as_deref_mut(),
            index,
            &(index + 1).to_ne_bytes(),
            &mut alloc,
        );
    }
    dsa_destroy_callback(dsa, &mut dsa_enum_fn(&row, 2));
    destroy_line("dsa_destroy_callback", &[], &row)
}

fn row_str_get_ptr_a_fits() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let ret = str_get_ptr_a(Some(b"hi\0"), Some(&mut buf), 8);
    str_get_line("str_get_ptr_a_fits", &[ret as i64], Some(&buf), &row)
}

fn row_str_get_ptr_a_truncates() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let ret = str_get_ptr_a(Some(b"hello\0"), Some(&mut buf), 4);
    str_get_line("str_get_ptr_a_truncates", &[ret as i64], Some(&buf), &row)
}

fn row_str_get_ptr_a_size_query() -> String {
    let row = Row::new(0);
    let ret = str_get_ptr_a(Some(b"abc\0"), None, 0);
    str_get_line::<u8>("str_get_ptr_a_size_query", &[ret as i64], None, &row)
}

fn row_str_get_ptr_a_null_src() -> String {
    let row = Row::new(0);
    let mut buf = [0x58u8; 8];
    let ret = str_get_ptr_a(None, Some(&mut buf), 4);
    str_get_line("str_get_ptr_a_null_src", &[ret as i64], Some(&buf), &row)
}

fn row_str_set_ptr_a() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u8>> = None;
    let ok = str_set_ptr_a(&mut dst, Some(b"xyz\0"), &mut alloc);
    str_set_line("str_set_ptr_a", &[ok as i64], dst.as_deref(), &row)
}

fn row_str_set_ptr_a_null() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u8>> = None;
    str_set_ptr_a(&mut dst, Some(b"xyz\0"), &mut alloc);
    let ok = str_set_ptr_a(&mut dst, None, &mut alloc);
    str_set_line("str_set_ptr_a_null", &[ok as i64], dst.as_deref(), &row)
}

fn row_str_set_ptr_a_alloc_fails() -> String {
    let row = Row::new(1);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u8>> = None;
    let ok = str_set_ptr_a(&mut dst, Some(b"xyz\0"), &mut alloc);
    str_set_line(
        "str_set_ptr_a_alloc_fails",
        &[ok as i64],
        dst.as_deref(),
        &row,
    )
}

fn row_str_get_ptr_w_fits() -> String {
    let row = Row::new(0);
    let mut buf = [u16::from_ne_bytes([0x58, 0x58]); 8];
    let ret = str_get_ptr_w(Some(&[104, 105, 0]), Some(&mut buf), 8);
    str_get_line("str_get_ptr_w_fits", &[ret as i64], Some(&buf), &row)
}

fn row_str_get_ptr_w_truncates() -> String {
    let row = Row::new(0);
    let mut buf = [u16::from_ne_bytes([0x58, 0x58]); 8];
    let ret = str_get_ptr_w(Some(&[104, 101, 108, 108, 111, 0]), Some(&mut buf), 4);
    str_get_line("str_get_ptr_w_truncates", &[ret as i64], Some(&buf), &row)
}

fn row_str_get_ptr_w_size_query() -> String {
    let row = Row::new(0);
    let ret = str_get_ptr_w(Some(&[97, 98, 99, 0]), None, 0);
    str_get_line::<u16>("str_get_ptr_w_size_query", &[ret as i64], None, &row)
}

fn row_str_get_ptr_w_null_src() -> String {
    let row = Row::new(0);
    let mut buf = [u16::from_ne_bytes([0x58, 0x58]); 8];
    let ret = str_get_ptr_w(None, Some(&mut buf), 3);
    str_get_line("str_get_ptr_w_null_src", &[ret as i64], Some(&buf), &row)
}

fn row_str_set_ptr_w() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u16>> = None;
    let ok = str_set_ptr_w(&mut dst, Some(&[97, 98, 0]), &mut alloc);
    str_set_line("str_set_ptr_w", &[ok as i64], dst.as_deref(), &row)
}

fn row_str_set_ptr_w_null() -> String {
    let row = Row::new(0);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u16>> = None;
    str_set_ptr_w(&mut dst, Some(&[97, 98, 0]), &mut alloc);
    let ok = str_set_ptr_w(&mut dst, None, &mut alloc);
    str_set_line("str_set_ptr_w_null", &[ok as i64], dst.as_deref(), &row)
}

fn row_str_set_ptr_w_alloc_fails() -> String {
    let row = Row::new(1);
    let mut alloc = alloc_fn(&row);
    let mut dst: Option<Vec<u16>> = None;
    let ok = str_set_ptr_w(&mut dst, Some(&[97, 98, 0]), &mut alloc);
    str_set_line(
        "str_set_ptr_w_alloc_fails",
        &[ok as i64],
        dst.as_deref(),
        &row,
    )
}

const STUB_ROWS: &[(&str, RowFn)] = &[
    ("stub CreatePage", row_stub_create_page),
    ("stub CreateProxyPage", row_stub_create_proxy_page),
    ("stub DoReaderMode", row_stub_do_reader_mode),
    ("stub SHGetProcessDword", row_stub_sh_get_process_dword),
];

const DPA_ROWS: &[(&str, RowFn)] = &[
    ("dpa_null_handle", row_dpa_null_handle),
    ("dpa_create_default", row_dpa_create_default),
    ("dpa_create_struct_fails", row_dpa_create_struct_fails),
    ("dpa_create_block_fails", row_dpa_create_block_fails),
    ("dpa_grow_20", row_dpa_grow_20),
    ("dpa_grow_alloc_fails", row_dpa_grow_alloc_fails),
    ("dpa_insert_order", row_dpa_insert_order),
    ("dpa_set_ptr_grows", row_dpa_set_ptr_grows),
    ("dpa_set_ptr_within", row_dpa_set_ptr_within),
    ("dpa_get_ptr", row_dpa_get_ptr),
    ("dpa_get_ptr_index", row_dpa_get_ptr_index),
    ("dpa_get_ptr_block_fails", row_dpa_get_ptr_block_fails),
    ("dpa_delete_ptr_shifts", row_dpa_delete_ptr_shifts),
    ("dpa_delete_ptr_shrinks", row_dpa_delete_ptr_shrinks),
    ("dpa_delete_ptr_alloc_fails", row_dpa_delete_ptr_alloc_fails),
    ("dpa_delete_all_ptrs", row_dpa_delete_all_ptrs),
    ("dpa_delete_all_alloc_fails", row_dpa_delete_all_alloc_fails),
    ("dpa_sort_five", row_dpa_sort_five),
    ("dpa_search_sorted", row_dpa_search_sorted),
    ("dpa_search_linear", row_dpa_search_linear),
    ("dpa_enum_stops", row_dpa_enum_stops),
    ("dpa_clone_null_dst", row_dpa_clone_null_dst),
    ("dpa_clone_into_existing", row_dpa_clone_into_existing),
    ("dpa_destroy_null", row_dpa_destroy_null),
    ("dpa_destroy", row_dpa_destroy),
    ("dpa_destroy_callback", row_dpa_destroy_callback),
];

const DSA_ROWS: &[(&str, RowFn)] = &[
    ("dsa_null_handle", row_dsa_null_handle),
    ("dsa_create", row_dsa_create),
    ("dsa_create_struct_fails", row_dsa_create_struct_fails),
    ("dsa_set_item_fresh", row_dsa_set_item_fresh),
    ("dsa_set_item_grow_gap", row_dsa_set_item_grow_gap),
    ("dsa_set_item_alloc_fails", row_dsa_set_item_alloc_fails),
    (
        "dsa_set_item_negative_index",
        row_dsa_set_item_negative_index,
    ),
    ("dsa_get_item", row_dsa_get_item),
    ("dsa_get_item_ptr", row_dsa_get_item_ptr),
    ("dsa_insert_front_mid", row_dsa_insert_front_mid),
    ("dsa_insert_alloc_fails", row_dsa_insert_alloc_fails),
    ("dsa_delete_item_shrinks", row_dsa_delete_item_shrinks),
    (
        "dsa_delete_item_alloc_fails",
        row_dsa_delete_item_alloc_fails,
    ),
    ("dsa_delete_all_items", row_dsa_delete_all_items),
    ("dsa_set_item_zero_size", row_dsa_set_item_zero_size),
    ("dsa_enum_callback", row_dsa_enum_callback),
    ("dsa_destroy_null", row_dsa_destroy_null),
    ("dsa_destroy", row_dsa_destroy),
    ("dsa_destroy_callback", row_dsa_destroy_callback),
];

const STR_ROWS: &[(&str, RowFn)] = &[
    ("str_get_ptr_a_fits", row_str_get_ptr_a_fits),
    ("str_get_ptr_a_truncates", row_str_get_ptr_a_truncates),
    ("str_get_ptr_a_size_query", row_str_get_ptr_a_size_query),
    ("str_get_ptr_a_null_src", row_str_get_ptr_a_null_src),
    ("str_set_ptr_a", row_str_set_ptr_a),
    ("str_set_ptr_a_null", row_str_set_ptr_a_null),
    ("str_set_ptr_a_alloc_fails", row_str_set_ptr_a_alloc_fails),
    ("str_get_ptr_w_fits", row_str_get_ptr_w_fits),
    ("str_get_ptr_w_truncates", row_str_get_ptr_w_truncates),
    ("str_get_ptr_w_size_query", row_str_get_ptr_w_size_query),
    ("str_get_ptr_w_null_src", row_str_get_ptr_w_null_src),
    ("str_set_ptr_w", row_str_set_ptr_w),
    ("str_set_ptr_w_null", row_str_set_ptr_w_null),
    ("str_set_ptr_w_alloc_fails", row_str_set_ptr_w_alloc_fails),
];

#[test]
fn stub_rows_match_oracle() {
    check_rows(ORACLE_STUB, STUB_ROWS);
}

#[test]
fn dpa_rows_match_oracle() {
    check_rows(ORACLE_DPA, DPA_ROWS);
}

#[test]
fn dsa_rows_match_oracle() {
    check_rows(ORACLE_DSA, DSA_ROWS);
}

#[test]
fn str_rows_match_oracle() {
    check_rows(ORACLE_STR, STR_ROWS);
}
