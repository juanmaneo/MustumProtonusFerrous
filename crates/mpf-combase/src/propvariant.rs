//! The VARTYPE check of `propvar_validatetype` in Wine's `dlls/combase/combase.c`.

use crate::{HRESULT, STG_E_INVALIDPARAMETER};

const VT_EMPTY: u16 = 0;
const VT_NULL: u16 = 1;
const VT_I2: u16 = 2;
const VT_I4: u16 = 3;
const VT_R4: u16 = 4;
const VT_R8: u16 = 5;
const VT_CY: u16 = 6;
const VT_DATE: u16 = 7;
const VT_BSTR: u16 = 8;
const VT_DISPATCH: u16 = 9;
const VT_ERROR: u16 = 10;
const VT_BOOL: u16 = 11;
const VT_VARIANT: u16 = 12;
const VT_UNKNOWN: u16 = 13;
const VT_DECIMAL: u16 = 14;
const VT_I1: u16 = 16;
const VT_UI1: u16 = 17;
const VT_UI2: u16 = 18;
const VT_UI4: u16 = 19;
const VT_I8: u16 = 20;
const VT_UI8: u16 = 21;
const VT_INT: u16 = 22;
const VT_UINT: u16 = 23;
const VT_LPSTR: u16 = 30;
const VT_LPWSTR: u16 = 31;
const VT_FILETIME: u16 = 64;
const VT_BLOB: u16 = 65;
const VT_STREAM: u16 = 66;
const VT_STORAGE: u16 = 67;
const VT_STREAMED_OBJECT: u16 = 68;
const VT_STORED_OBJECT: u16 = 69;
const VT_BLOB_OBJECT: u16 = 70;
const VT_CF: u16 = 71;
const VT_CLSID: u16 = 72;
const VT_VECTOR: u16 = 0x1000;
const VT_ARRAY: u16 = 0x2000;

/// The VARTYPEs `propvar_validatetype` accepts, in its case-label order: 33
/// scalars, then 21 `VT_VECTOR` forms, then 19 `VT_ARRAY` forms.
const VALID: [u16; 73] = [
    VT_EMPTY,
    VT_NULL,
    VT_I1,
    VT_I2,
    VT_I4,
    VT_I8,
    VT_R4,
    VT_R8,
    VT_CY,
    VT_DATE,
    VT_BSTR,
    VT_ERROR,
    VT_BOOL,
    VT_DECIMAL,
    VT_UI1,
    VT_UI2,
    VT_UI4,
    VT_UI8,
    VT_INT,
    VT_UINT,
    VT_LPSTR,
    VT_LPWSTR,
    VT_FILETIME,
    VT_BLOB,
    VT_DISPATCH,
    VT_UNKNOWN,
    VT_STREAM,
    VT_STORAGE,
    VT_STREAMED_OBJECT,
    VT_STORED_OBJECT,
    VT_BLOB_OBJECT,
    VT_CF,
    VT_CLSID,
    VT_I1 | VT_VECTOR,
    VT_I2 | VT_VECTOR,
    VT_I4 | VT_VECTOR,
    VT_I8 | VT_VECTOR,
    VT_R4 | VT_VECTOR,
    VT_R8 | VT_VECTOR,
    VT_CY | VT_VECTOR,
    VT_DATE | VT_VECTOR,
    VT_BSTR | VT_VECTOR,
    VT_ERROR | VT_VECTOR,
    VT_BOOL | VT_VECTOR,
    VT_VARIANT | VT_VECTOR,
    VT_UI1 | VT_VECTOR,
    VT_UI2 | VT_VECTOR,
    VT_UI4 | VT_VECTOR,
    VT_UI8 | VT_VECTOR,
    VT_LPSTR | VT_VECTOR,
    VT_LPWSTR | VT_VECTOR,
    VT_FILETIME | VT_VECTOR,
    VT_CF | VT_VECTOR,
    VT_CLSID | VT_VECTOR,
    VT_I1 | VT_ARRAY,
    VT_UI1 | VT_ARRAY,
    VT_I2 | VT_ARRAY,
    VT_UI2 | VT_ARRAY,
    VT_I4 | VT_ARRAY,
    VT_UI4 | VT_ARRAY,
    VT_INT | VT_ARRAY,
    VT_UINT | VT_ARRAY,
    VT_R4 | VT_ARRAY,
    VT_R8 | VT_ARRAY,
    VT_CY | VT_ARRAY,
    VT_DATE | VT_ARRAY,
    VT_BSTR | VT_ARRAY,
    VT_BOOL | VT_ARRAY,
    VT_DECIMAL | VT_ARRAY,
    VT_DISPATCH | VT_ARRAY,
    VT_UNKNOWN | VT_ARRAY,
    VT_ERROR | VT_ARRAY,
    VT_VARIANT | VT_ARRAY,
];

/// `propvar_validatetype`: `STG_E_INVALIDPARAMETER` for a VARTYPE that no
/// PROPVARIANT may hold.
pub fn validate_vartype(vt: u16) -> Result<(), HRESULT> {
    if VALID.contains(&vt) {
        Ok(())
    } else {
        Err(STG_E_INVALIDPARAMETER)
    }
}
