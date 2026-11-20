//! `connect_method_from_mf` from `dlls/mf/topology_loader.c`: maps the `MF_CONNECT_ALLOW_*` flags of
//! a topology load to the connect methods that the loader may try. A direct connection is always
//! allowed.

/// `CONNECT_DIRECT`: connect the two nodes without an intermediate object.
pub const CONNECT_DIRECT: u32 = 1;

/// `CONNECT_CONVERTER`: connect through a converter.
pub const CONNECT_CONVERTER: u32 = 2;

/// `CONNECT_DECODER`: connect through a decoder.
pub const CONNECT_DECODER: u32 = 4;

const MF_CONNECT_ALLOW_CONVERTER: u32 = 0x1;
const MF_CONNECT_ALLOW_DECODER: u32 = 0x3;

/// The connect methods that the `MF_CONNECT_ALLOW_*` bits of `mf_method` allow, as the `CONNECT_*`
/// bits. A converter needs bit 0 set. A decoder needs bits 0 and 1 set, so it needs both.
pub fn connect_method_from_mf(mf_method: u32) -> u32 {
    let mut method = CONNECT_DIRECT;
    if (mf_method & MF_CONNECT_ALLOW_CONVERTER) == MF_CONNECT_ALLOW_CONVERTER {
        method |= CONNECT_CONVERTER;
    }
    if (mf_method & MF_CONNECT_ALLOW_DECODER) == MF_CONNECT_ALLOW_DECODER {
        method |= CONNECT_DECODER;
    }
    method
}
