//! Port of `DllGetClassObject` and the `l3codecx.ax.spec` export table.

use crate::class_factory::{ClassFactory, MPEG_LAYER3_DECODER_FACTORY};
use crate::com::{Guid, Hresult, CLASS_E_CLASSNOTAVAILABLE};
use crate::registration::CLSID_MPEG_LAYER3_DECODER;

/// The `@ stdcall -private` lines of `l3codecx.ax.spec`: export name and argument list.
pub const EXPORTS: [(&str, &str); 3] = [
    ("DllGetClassObject", "ptr ptr ptr"),
    ("DllRegisterServer", ""),
    ("DllUnregisterServer", ""),
];

/// The argument list of `name` in the spec file, or `None` when the spec does not export it.
pub fn export_signature(name: &str) -> Option<&'static str> {
    EXPORTS
        .iter()
        .find(|(export, _)| *export == name)
        .map(|(_, args)| *args)
}

/// `DllGetClassObject`. Only `CLSID_MPEGLayer3Decoder` is served, through the static factory.
/// For any other class the `out` pointer is NULL and the result is `CLASS_E_CLASSNOTAVAILABLE`.
pub fn dll_get_class_object(clsid: &Guid, iid: &Guid) -> (Hresult, Option<&'static ClassFactory>) {
    if *clsid == CLSID_MPEG_LAYER3_DECODER {
        return MPEG_LAYER3_DECODER_FACTORY.query_interface(iid);
    }
    (CLASS_E_CLASSNOTAVAILABLE, None)
}
