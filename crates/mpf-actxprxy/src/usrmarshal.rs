/// `HRESULT` as the C bodies compare it.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `S_FALSE` from `include/winerror.h`.
pub const S_FALSE: Hresult = 1;

/// The midl-generated `IEnumShellItems_RemoteNext_Proxy` and `IEnumObjects_RemoteNext_Proxy`
/// callees, and the `IEnumShellItems_Next` and `IEnumObjects_Next` callees of the stubs, reduced
/// to the `pceltFetched` out-parameter. The other arguments are not modelled.
pub type NextFn = fn(celt: u32, celt_fetched: &mut u32) -> Hresult;

/// `IEnumShellItems_Next_Proxy` and `IEnumObjects_Next_Proxy`. A NULL `pceltFetched` is replaced
/// by a local, so the callee always receives a valid pointer.
pub fn enum_next_proxy(celt: u32, celt_fetched: Option<&mut u32>, next: NextFn) -> Hresult {
    match celt_fetched {
        Some(fetched) => next(celt, fetched),
        // The C local is uninitialised. It is zeroed here, and the callee's value is discarded.
        None => next(celt, &mut 0),
    }
}

/// `IEnumShellItems_Next_Stub`. Clears `*pceltFetched`, calls the implementation, and on `S_OK`
/// reports `celt` items. Any other return value keeps what the implementation wrote.
pub fn enum_shell_items_next_stub(celt: u32, celt_fetched: &mut u32, next: NextFn) -> Hresult {
    *celt_fetched = 0;
    let hr = next(celt, celt_fetched);
    if hr == S_OK {
        *celt_fetched = celt;
    }
    hr
}

/// `IEnumObjects_Next_Stub`. Clears `*pceltFetched` and returns the implementation's value with
/// whatever count it wrote.
pub fn enum_objects_next_stub(celt: u32, celt_fetched: &mut u32, next: NextFn) -> Hresult {
    *celt_fetched = 0;
    next(celt, celt_fetched)
}
