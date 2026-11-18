//! msasn1 handle setup: the headless part of Wine's `dlls/msasn1/main.c`.
//!
//! Wine keeps module, encoder and decoder handles as malloc'd C structs that
//! hold raw pointers into caller memory. This port keeps the same bookkeeping
//! over borrowed slices, and the borrow checker takes over the lifetime
//! rules Wine leaves to the caller. Function tables are slices, compared by
//! address where identity matters.
//!
//! Ported (the vectors in the tests come from Wine's C built with ASan/UBSan):
//! - `ASN1_CreateModule` -> [`create_module`]: a missing table gives `None`
//!   (Wine returns NULL). The rule bits pick the PER table (`0x3`) first, then
//!   BER (`0x700`), else [`Codec::Unset`]. The `ver` argument is ignored by
//!   Wine, so it is not taken.
//! - `ASN1_CreateEncoder` -> [`create_encoder`]: a buffer is attached only
//!   when one is given and `bufsize` is non-zero, and then sets
//!   `ASN1ENCODE_SETBUFFER`. It cannot fail: the `ASN1_ERR_BADARGS` returns
//!   for a NULL module or out-pointer are excluded by the types, and Wine's
//!   `ASN1_ERR_MEMORY` is an allocation failure that aborts in Rust.
//! - `ASN1_CreateDecoder` -> [`create_decoder`]: a buffer is attached whenever
//!   one is given, even with size 0, and sets `ASN1DECODE_SETBUFFER`. `size`
//!   is the declared `bufsize` in both cases, as in Wine.
//! - `ASN1_Decode` -> [`decode`]: argument checks only. It returns
//!   `BadArgs` for a missing buffer or size 0, and `BadPdu` otherwise, and
//!   records the code in the handle's `err`. Wine never decodes. `pdunum`,
//!   `flags` and `outdata` are only traced there, so they are not taken.
//!
//! Replaced by ownership (no logic to port):
//! - `ASN1_CloseModule`, `ASN1_CloseEncoder`, `ASN1_CloseDecoder`: `Drop`.
//!   Wine's encoder and decoder close functions are FIXME stubs that never
//!   free, so the port does not leak where Wine does.
//!
//! Skipped (259 exports, all `@ stub` in `msasn1.spec`; Wine has no
//! behaviour to match, and writing one would be new codec code, not a port):
//! - 69 `ASN1BER*` and 13 `ASN1CEREnc*` BER/CER primitives, and 127
//!   `ASN1PER*` PER primitives: no codec exists in Wine.
//! - 8 `ASN1DecAbort`, `ASN1DecAlloc`, `ASN1DecDone`, `ASN1DecRealloc`,
//!   `ASN1DecSetError`, `ASN1EncAbort`, `ASN1EncDone`, `ASN1EncSetError`:
//!   stub error and allocation hooks for a codec that does not exist.
//! - 9 `ASN1_*` entry points: `ASN1_Encode`, `ASN1_FreeEncoded`,
//!   `ASN1_FreeDecoded`, `ASN1_CreateDecoderEx`, `ASN1_CloseEncoder2`, and the
//!   four option get/set calls. They are all stubs.
//! - 33 value helpers (`*_cmp`, `*_free`, `ASN1intx*`,
//!   `ASN1uint32_uoctets`, `ASN1Free`): stubs.

/// Rule bits that select the PER function table (`ASN1_PER_RULE`).
pub const ASN1_PER_RULE: u32 = 0x0003;
/// Rule bits that select the BER function table (`ASN1_BER_RULE`).
pub const ASN1_BER_RULE: u32 = 0x0700;
/// `ASN1ENCODE_SETBUFFER`, set on an encoder once a buffer is attached.
pub const ASN1ENCODE_SETBUFFER: u32 = 0x0008;
/// `ASN1DECODE_SETBUFFER`, set on a decoder once a buffer is attached.
pub const ASN1DECODE_SETBUFFER: u32 = 0x0008;
/// Magic of an encoder handle. Wine stores it in every handle; here it is
/// the same constant for all of them.
pub const ENCODER_MAGIC: u32 = 0x4443_4e45;
/// Magic of a decoder handle.
pub const DECODER_MAGIC: u32 = 0x4443_4544;

/// The `ASN1error_e` codes this port returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// `ASN1_ERR_BADPDU`.
    BadPdu,
    /// `ASN1_ERR_BADARGS`.
    BadArgs,
}

impl Error {
    /// The `ASN1error_e` value Wine returns for this error.
    pub const fn code(self) -> i32 {
        match self {
            Error::BadPdu => -1008,
            Error::BadArgs => -1009,
        }
    }
}

/// The arguments of `ASN1_CreateModule`. A `None` table is a NULL pointer,
/// which Wine rejects.
pub struct ModuleDesc<'a, E, D, F, S> {
    pub name: u32,
    pub rule: u32,
    pub flags: u32,
    pub pdus: u32,
    pub encoder: Option<&'a [E]>,
    pub decoder: Option<&'a [D]>,
    pub free_memory: Option<&'a [F]>,
    pub struct_sizes: Option<&'a [S]>,
}

/// The function tables a module selected from its rule bits. Wine keeps
/// them in one union, so only one variant is ever in use.
pub enum Codec<'a, E, D> {
    Per { encoder: &'a [E], decoder: &'a [D] },
    Ber { encoder: &'a [E], decoder: &'a [D] },
    Unset,
}

/// An `ASN1module_t`.
pub struct Module<'a, E, D, F, S> {
    pub name: u32,
    pub rule: u32,
    pub flags: u32,
    pub pdus: u32,
    pub free_memory: &'a [F],
    pub struct_sizes: &'a [S],
    pub codec: Codec<'a, E, D>,
}

/// `ASN1_CreateModule`. Returns `None` when any table is missing.
pub fn create_module<'a, E, D, F, S>(
    desc: ModuleDesc<'a, E, D, F, S>,
) -> Option<Module<'a, E, D, F, S>> {
    let (Some(encoder), Some(decoder), Some(free_memory), Some(struct_sizes)) = (
        desc.encoder,
        desc.decoder,
        desc.free_memory,
        desc.struct_sizes,
    ) else {
        return None;
    };
    let codec = if (desc.rule & ASN1_PER_RULE) != 0 {
        Codec::Per { encoder, decoder }
    } else if (desc.rule & ASN1_BER_RULE) != 0 {
        Codec::Ber { encoder, decoder }
    } else {
        Codec::Unset
    };
    Some(Module {
        name: desc.name,
        rule: desc.rule,
        flags: desc.flags,
        pdus: desc.pdus,
        free_memory,
        struct_sizes,
        codec,
    })
}

/// An encoder handle (`struct ASN1encoding_s`). `buf` is set only when a
/// buffer was attached, and `size` is the declared buffer size.
pub struct Encoding<'m, 'a, 'b, E, D, F, S> {
    pub module: &'m Module<'a, E, D, F, S>,
    pub rule: u32,
    pub flags: u32,
    pub buf: Option<&'b mut [u8]>,
    pub size: u32,
    pub err: Option<Error>,
}

/// `ASN1_CreateEncoder`. A buffer is attached only when it is given and
/// `bufsize` is non-zero. `err` is `None`, which is `ASN1_SUCCESS`.
pub fn create_encoder<'m, 'a, 'b, E, D, F, S>(
    module: &'m Module<'a, E, D, F, S>,
    buf: Option<&'b mut [u8]>,
    bufsize: u32,
) -> Encoding<'m, 'a, 'b, E, D, F, S> {
    let (buf, size) = if buf.is_some() && bufsize != 0 {
        (buf, bufsize)
    } else {
        (None, 0)
    };
    let flags = if buf.is_some() {
        module.flags | ASN1ENCODE_SETBUFFER
    } else {
        module.flags
    };
    Encoding {
        module,
        rule: module.rule,
        flags,
        buf,
        size,
        err: None,
    }
}

/// A decoder handle (`struct ASN1decoding_s`).
pub struct Decoding<'m, 'a, 'b, E, D, F, S> {
    pub module: &'m Module<'a, E, D, F, S>,
    pub rule: u32,
    pub flags: u32,
    pub buf: Option<&'b [u8]>,
    pub size: u32,
    pub err: Option<Error>,
}

/// `ASN1_CreateDecoder`. A buffer is attached whenever one is given, even
/// with a size of 0. `size` is `bufsize` either way.
pub fn create_decoder<'m, 'a, 'b, E, D, F, S>(
    module: &'m Module<'a, E, D, F, S>,
    buf: Option<&'b [u8]>,
    bufsize: u32,
) -> Decoding<'m, 'a, 'b, E, D, F, S> {
    let flags = if buf.is_some() {
        module.flags | ASN1DECODE_SETBUFFER
    } else {
        module.flags
    };
    Decoding {
        module,
        rule: module.rule,
        flags,
        buf,
        size: bufsize,
        err: None,
    }
}

/// `ASN1_Decode`, argument checks only. Wine never decodes, so every call
/// that passes the checks returns `BadPdu`. The code is also stored in
/// `decoder.err`.
pub fn decode<E, D, F, S>(
    decoder: &mut Decoding<'_, '_, '_, E, D, F, S>,
    buf: Option<&[u8]>,
    bufsize: u32,
) -> Error {
    let error = if buf.is_none() || bufsize == 0 {
        Error::BadArgs
    } else {
        Error::BadPdu
    };
    decoder.err = Some(error);
    error
}

#[cfg(test)]
mod tests {
    use super::*;

    // Stand-ins for the caller's function tables. Only their identity matters.
    static ENC: [u8; 1] = [0];
    static DEC: [u8; 1] = [1];
    static FREE: [u8; 1] = [2];
    static SIZE: [u8; 1] = [3];

    /// The oracle's name for a table pointer: `null`, the role name for one
    /// of the four tables above, or `other`.
    fn token(table: Option<&[u8]>) -> &'static str {
        let Some(table) = table else {
            return "null";
        };
        [
            ("enc", &ENC[..]),
            ("dec", &DEC[..]),
            ("free", &FREE[..]),
            ("size", &SIZE[..]),
        ]
        .into_iter()
        .find(|(_, known)| known.as_ptr() == table.as_ptr())
        .map_or("other", |(name, _)| name)
    }

    fn module_line(module: Option<Module<'_, u8, u8, u8, u8>>) -> String {
        let Some(module) = module else {
            return "mod=0".to_string();
        };
        let (encoder, decoder) = match module.codec {
            Codec::Per { encoder, decoder } | Codec::Ber { encoder, decoder } => {
                (Some(encoder), Some(decoder))
            }
            Codec::Unset => (None, None),
        };
        format!(
            "mod=1 name={:x} rule={:x} flags={:x} pdus={:x} free={} size={} enc={} dec={}",
            module.name,
            module.rule,
            module.flags,
            module.pdus,
            token(Some(module.free_memory)),
            token(Some(module.struct_sizes)),
            token(encoder),
            token(decoder),
        )
    }

    fn table(present: u32, table: &'static [u8]) -> Option<&'static [u8]> {
        (present != 0).then_some(table)
    }

    /// Input `ver rule flags pdu name enc dec free size` (see harness.c `M`).
    fn module_output(n: &[u32]) -> String {
        module_line(create_module(ModuleDesc {
            name: n[4],
            rule: n[1],
            flags: n[2],
            pdus: n[3],
            encoder: table(n[5], &ENC),
            decoder: table(n[6], &DEC),
            free_memory: table(n[7], &FREE),
            struct_sizes: table(n[8], &SIZE),
        }))
    }

    /// A module with all four tables, for the handle inputs.
    fn full_module(rule: u32, flags: u32) -> Module<'static, u8, u8, u8, u8> {
        create_module(ModuleDesc {
            name: 0x61736e31,
            rule,
            flags,
            pdus: 1,
            encoder: Some(&ENC[..]),
            decoder: Some(&DEC[..]),
            free_memory: Some(&FREE[..]),
            struct_sizes: Some(&SIZE[..]),
        })
        .unwrap()
    }

    /// Input `rule flags buf bufsize` (see harness.c `E`).
    fn encoder_output(n: &[u32]) -> String {
        let module = full_module(n[0], n[1]);
        let mut bytes = [0u8; 8];
        let buf = (n[2] != 0).then_some(&mut bytes[..]);
        let enc = create_encoder(&module, buf, n[3]);
        format!(
            "magic={:x} mod=1 buf={} size={} err={} rule={:x} flags={:x}",
            ENCODER_MAGIC,
            u8::from(enc.buf.is_some()),
            enc.size,
            enc.err.map_or(0, Error::code),
            enc.rule,
            enc.flags,
        )
    }

    /// Input `rule flags buf bufsize dbuf dbufsize pdunum dflags` (see
    /// harness.c `D`). `pdunum` and `dflags` do not affect the result.
    fn decoder_output(n: &[u32]) -> String {
        let module = full_module(n[0], n[1]);
        let bytes = [0u8; 8];
        let mut dec = create_decoder(&module, (n[2] != 0).then_some(&bytes[..]), n[3]);
        let ret = decode(&mut dec, (n[4] != 0).then_some(&bytes[..]), n[5]);
        format!(
            "magic={:x} mod=1 buf={} size={} rule={:x} flags={:x} ret={} err={}",
            DECODER_MAGIC,
            u8::from(dec.buf.is_some()),
            dec.size,
            dec.rule,
            dec.flags,
            ret.code(),
            dec.err.map_or(0, Error::code),
        )
    }

    fn numbers(input: &str) -> Vec<u32> {
        input
            .split_whitespace()
            .map(|t| u32::from_str_radix(t, 16).unwrap())
            .collect()
    }

    /// Input lines and the oracle's output, from `harness M`.
    const MODULE_VECTORS: [(&str, &str); 9] = [
        (
            "0 0 0 0 0 1 1 1 1",
            "mod=1 name=0 rule=0 flags=0 pdus=0 free=free size=size enc=null dec=null",
        ),
        (
            "0 1 0 1 61736e31 1 1 1 1",
            "mod=1 name=61736e31 rule=1 flags=0 pdus=1 free=free size=size enc=enc dec=dec",
        ),
        (
            "12345678 400 1000 1 61736e31 1 1 1 1",
            "mod=1 name=61736e31 rule=400 flags=1000 pdus=1 free=free size=size enc=enc dec=dec",
        ),
        (
            "0 703 0 0 0 1 1 1 1",
            "mod=1 name=0 rule=703 flags=0 pdus=0 free=free size=size enc=enc dec=dec",
        ),
        (
            "0 10 0 0 0 1 1 1 1",
            "mod=1 name=0 rule=10 flags=0 pdus=0 free=free size=size enc=null dec=null",
        ),
        ("0 400 0 0 0 0 1 1 1", "mod=0"),
        ("0 400 0 0 0 1 0 1 1", "mod=0"),
        ("0 3 0 0 0 1 1 0 1", "mod=0"),
        ("0 3 0 0 0 1 1 1 0", "mod=0"),
    ];

    /// Input lines and the oracle's output, from `harness E`.
    const ENCODER_VECTORS: [(&str, &str); 6] = [
        (
            "400 1000 0 0",
            "magic=44434e45 mod=1 buf=0 size=0 err=0 rule=400 flags=1000",
        ),
        (
            "400 1000 1 0",
            "magic=44434e45 mod=1 buf=0 size=0 err=0 rule=400 flags=1000",
        ),
        (
            "400 0 1 4",
            "magic=44434e45 mod=1 buf=1 size=4 err=0 rule=400 flags=8",
        ),
        (
            "400 1000 1 2",
            "magic=44434e45 mod=1 buf=1 size=2 err=0 rule=400 flags=1008",
        ),
        (
            "400 0 0 4",
            "magic=44434e45 mod=1 buf=0 size=0 err=0 rule=400 flags=0",
        ),
        (
            "1 1000 1 4",
            "magic=44434e45 mod=1 buf=1 size=4 err=0 rule=1 flags=1008",
        ),
    ];

    /// Input lines and the oracle's output, from `harness D`.
    const DECODER_VECTORS: [(&str, &str); 7] = [
        (
            "400 1000 0 0 0 0 0 0",
            "magic=44434544 mod=1 buf=0 size=0 rule=400 flags=1000 ret=-1009 err=-1009",
        ),
        (
            "400 0 0 4 1 4 7 1234",
            "magic=44434544 mod=1 buf=0 size=4 rule=400 flags=0 ret=-1008 err=-1008",
        ),
        (
            "400 0 1 0 1 0 0 0",
            "magic=44434544 mod=1 buf=1 size=0 rule=400 flags=8 ret=-1009 err=-1009",
        ),
        (
            "400 1000 1 2 0 2 0 0",
            "magic=44434544 mod=1 buf=1 size=2 rule=400 flags=1008 ret=-1009 err=-1009",
        ),
        (
            "400 1000 1 2 1 2 0 0",
            "magic=44434544 mod=1 buf=1 size=2 rule=400 flags=1008 ret=-1008 err=-1008",
        ),
        (
            "1 1000 1 4 1 4 0 0",
            "magic=44434544 mod=1 buf=1 size=4 rule=1 flags=1008 ret=-1008 err=-1008",
        ),
        (
            "0 0 1 4 1 4 0 0",
            "magic=44434544 mod=1 buf=1 size=4 rule=0 flags=8 ret=-1008 err=-1008",
        ),
    ];

    #[test]
    fn module_vectors_match_wine_oracle() {
        for (input, expected) in MODULE_VECTORS {
            assert_eq!(module_output(&numbers(input)), expected, "module `{input}`");
        }
    }

    #[test]
    fn encoder_vectors_match_wine_oracle() {
        for (input, expected) in ENCODER_VECTORS {
            assert_eq!(
                encoder_output(&numbers(input)),
                expected,
                "encoder `{input}`"
            );
        }
    }

    #[test]
    fn decoder_vectors_match_wine_oracle() {
        for (input, expected) in DECODER_VECTORS {
            assert_eq!(
                decoder_output(&numbers(input)),
                expected,
                "decoder `{input}`"
            );
        }
    }

    #[test]
    fn rule_bits_pick_the_codec_with_per_first() {
        assert!(matches!(
            full_module(ASN1_PER_RULE, 0).codec,
            Codec::Per { .. }
        ));
        assert!(matches!(
            full_module(ASN1_BER_RULE, 0).codec,
            Codec::Ber { .. }
        ));
        assert!(matches!(
            full_module(ASN1_PER_RULE | ASN1_BER_RULE, 0).codec,
            Codec::Per { .. }
        ));
        assert!(matches!(full_module(0x10, 0).codec, Codec::Unset));
    }

    #[test]
    fn handles_keep_their_module_and_tables() {
        let module = full_module(ASN1_BER_RULE, 0);
        let encoding = create_encoder(&module, None, 0);
        assert!(std::ptr::eq(encoding.module, &module));
        assert!(std::ptr::eq(module.free_memory, &FREE[..]));
        assert!(matches!(
            module.codec,
            Codec::Ber { encoder, .. } if std::ptr::eq(encoder, &ENC[..])
        ));
    }
}
