use crate::stub::{
    c_sm16_release_handler_release32, callback_processing_3216, convert_hr1632_thunk,
    convert_hr3216_thunk, convert_obj_descriptor, i_unknown_obj32, int_op_initialize,
    int_op_uninitialize, invoke_on_32, thk_add_app_compat_flag, thk_mgr_initialize,
    thk_mgr_uninitialize, transform_hresult_1632, transform_hresult_3216,
};
use crate::StubException;

/// One `@ stub` line of `dlls/olethk32/olethk32.spec`.
#[derive(Clone, Copy, Debug)]
pub struct Export {
    /// The ordinal before `stub` in the spec.
    pub ordinal: u16,
    /// The export name, which is the `function` argument `winebuild` passes to the stub.
    pub name: &'static str,
    /// The helper that raises the stub record for this export.
    pub stub: fn() -> StubException,
}

/// The `olethk32.spec` export table, in spec order. Every export is `@ stub`.
pub const EXPORTS: [Export; 14] = [
    Export {
        ordinal: 3,
        name: "InvokeOn32",
        stub: invoke_on_32,
    },
    Export {
        ordinal: 4,
        name: "IntOpInitialize",
        stub: int_op_initialize,
    },
    Export {
        ordinal: 5,
        name: "CallbackProcessing_3216",
        stub: callback_processing_3216,
    },
    Export {
        ordinal: 6,
        name: "IUnknownObj32",
        stub: i_unknown_obj32,
    },
    Export {
        ordinal: 8,
        name: "CSm16ReleaseHandler_Release32",
        stub: c_sm16_release_handler_release32,
    },
    Export {
        ordinal: 9,
        name: "ThkMgrInitialize",
        stub: thk_mgr_initialize,
    },
    Export {
        ordinal: 10,
        name: "ThkMgrUninitialize",
        stub: thk_mgr_uninitialize,
    },
    Export {
        ordinal: 11,
        name: "TransformHRESULT_1632",
        stub: transform_hresult_1632,
    },
    Export {
        ordinal: 12,
        name: "TransformHRESULT_3216",
        stub: transform_hresult_3216,
    },
    Export {
        ordinal: 13,
        name: "ConvertObjDescriptor",
        stub: convert_obj_descriptor,
    },
    Export {
        ordinal: 14,
        name: "ConvertHr1632Thunk",
        stub: convert_hr1632_thunk,
    },
    Export {
        ordinal: 15,
        name: "ConvertHr3216Thunk",
        stub: convert_hr3216_thunk,
    },
    Export {
        ordinal: 16,
        name: "IntOpUninitialize",
        stub: int_op_uninitialize,
    },
    Export {
        ordinal: 18,
        name: "ThkAddAppCompatFlag",
        stub: thk_add_app_compat_flag,
    },
];

/// The stub record that the export `name` raises, or `None` when the spec does not export it.
/// Names are case-sensitive.
pub fn export_stub(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|export| export.name == name)
        .map(|export| (export.stub)())
}
