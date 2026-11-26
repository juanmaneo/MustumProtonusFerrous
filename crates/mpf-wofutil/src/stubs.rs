/// `EXCEPTION_WINE_STUB` from `include/winternl.h`.
pub const EXCEPTION_WINE_STUB: u32 = 0x8000_0100;

/// `EXCEPTION_NONCONTINUABLE` from `include/winnt.h`.
pub const EXCEPTION_NONCONTINUABLE: u32 = 0x01;

/// The module name `winebuild` passes to stubs, from `MODULE = wofutil.dll` in `dlls/wofutil/Makefile.in`.
pub const WOFUTIL_DLL_NAME: &str = "wofutil.dll";

/// The record that `__wine_spec_unimplemented_stub` raises.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StubException {
    pub code: u32,
    pub flags: u32,
    pub number_parameters: u32,
    pub module: &'static str,
    pub function: &'static str,
}

/// `__wine_spec_unimplemented_stub` from `dlls/ntdll/exception.c`. The C function raises
/// the record in a loop and never returns, so the port returns the record instead.
pub fn wine_spec_unimplemented_stub(module: &'static str, function: &'static str) -> StubException {
    StubException {
        code: EXCEPTION_WINE_STUB,
        flags: EXCEPTION_NONCONTINUABLE,
        // The module and the function name.
        number_parameters: 2,
        module,
        function,
    }
}

/// Defines one helper per `@ stub` export, and the table of them in spec order.
macro_rules! stub_exports {
    ($($helper:ident => $export:literal,)*) => {
        $(
            #[doc = concat!("`", $export, "`, exported as `@ stub` in `dlls/wofutil/wofutil.spec`.")]
            pub fn $helper() -> StubException {
                wine_spec_unimplemented_stub(WOFUTIL_DLL_NAME, $export)
            }
        )*

        /// The `@ stub` exports of `wofutil.spec`, in spec order, with their helpers.
        pub const STUB_EXPORTS: &[(&str, fn() -> StubException)] = &[$(($export, $helper)),*];
    };
}

stub_exports! {
    wof_enum_entries => "WofEnumEntries",
    wof_file_enum_files => "WofFileEnumFiles",
    wof_get_driver_version => "WofGetDriverVersion",
    wof_set_file_data_location => "WofSetFileDataLocation",
    wof_wim_add_entry => "WofWimAddEntry",
    wof_wim_enum_files => "WofWimEnumFiles",
    wof_wim_remove_entry => "WofWimRemoveEntry",
    wof_wim_suspend_entry => "WofWimSuspendEntry",
    wof_wim_update_entry => "WofWimUpdateEntry",
}
