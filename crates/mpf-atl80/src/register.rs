/// `HRESULT`.
pub type Hresult = i32;

/// `S_OK` from `include/winerror.h`.
pub const S_OK: Hresult = 0;

/// `FAILED` from `include/winerror.h`. A failure HRESULT is negative.
pub fn failed(hr: Hresult) -> bool {
    hr < 0
}

/// `GUID`, the type behind `CLSID` and `CATID`. `IsEqualCLSID` compares all 16 bytes, which is
/// what the derived `PartialEq` does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `HINSTANCE` of the module that holds the type library. The port never reads it.
pub type HInstance = usize;

/// `struct _ATL_CATMAP_ENTRY`. The C category map is a NUL-terminated array. The port takes the
/// entries as a slice, so the terminator is not part of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CatMapEntry {
    pub i_type: i32,
    pub pcatid: Guid,
}

/// `_ATL_OBJMAP_ENTRY`, the fields that `AtlComModuleRegisterServer` reads.
pub struct ObjMapEntry {
    /// `pclsid`.
    pub pclsid: Guid,
    /// `pfnUpdateRegistry`. The C code always calls it with `TRUE`.
    pub update_registry: fn(bool) -> Hresult,
    /// `pfnGetCategoryMap`. `None` is the NULL map that skips the category pass.
    pub get_category_map: fn() -> Option<&'static [CatMapEntry]>,
}

/// `AtlComModuleRegisterServer` from `dlls/atl80/atl80.c`, ordinal 18.
///
/// `obj_map` is the range `m_ppAutoObjMapFirst..m_ppAutoObjMapLast`, and a `None` entry is a
/// NULL pointer. For each non-NULL entry whose CLSID equals `clsid`, or for every entry when
/// `clsid` is `None`, the port calls `update_registry(true)`. Then it calls
/// `register_class_categories` with the entry's CLSID, its category map and `true`, if the
/// entry returns a map. The first failing call returns its HRESULT and nothing after it runs.
/// When `register_type_lib` is set, `register_type_lib_fn` runs last with `hinst_type_lib`.
/// The C code passes a NULL index, so the port has no index parameter.
pub fn atl_com_module_register_server(
    obj_map: &[Option<ObjMapEntry>],
    hinst_type_lib: HInstance,
    register_type_lib: bool,
    clsid: Option<&Guid>,
    register_class_categories: fn(&Guid, &[CatMapEntry], bool) -> Hresult,
    register_type_lib_fn: fn(HInstance) -> Hresult,
) -> Hresult {
    for entry in obj_map {
        let Some(entry) = entry else {
            continue;
        };
        if clsid.is_some_and(|clsid| *clsid != entry.pclsid) {
            continue;
        }
        let hres = (entry.update_registry)(true);
        if failed(hres) {
            return hres;
        }
        if let Some(catmap) = (entry.get_category_map)() {
            let hres = register_class_categories(&entry.pclsid, catmap, true);
            if failed(hres) {
                return hres;
            }
        }
    }

    if register_type_lib {
        let hres = register_type_lib_fn(hinst_type_lib);
        if failed(hres) {
            return hres;
        }
    }
    S_OK
}
