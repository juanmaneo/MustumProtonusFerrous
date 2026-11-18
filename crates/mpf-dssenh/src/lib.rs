//! dssenh subset: the headless logic of Wine's SymCrypt-backed DSS/DH CSP.
//!
//! Reference: `dlls/dssenh/main.c` and `dssenh.spec`. The spec has 27 exports
//! (25 `CP*` functions and `DllRegisterServer`/`DllUnregisterServer`). Only the
//! pure computations behind seven of them are ported; the rest need a live
//! SymCrypt key or hash, registry-backed containers, the OS random source, or
//! have no implementation.
//!
//! Ported:
//! - `CPCreateHash`: [`hash_len`], the MD5 and SHA-1 digest lengths.
//! - `CPSignHash`: [`signature_len`], the length gate only; signing needs a live key.
//! - `CPVerifySignature`: [`signature_len`], the same length gate.
//! - `CPGenKey`: [`gen_key_params`], the bit length and low flags.
//! - `CPGetProvParam`: [`provider_name`], [`copy_param`], [`next_base_alg`] for `PP_NAME`
//!   and `PP_ENUMALGS_EX`; the legacy `PP_ENUMALGS` struct layout is not ported.
//! - `CPExportKey`: [`dss_blob_len`] and [`encode_dss_blob`], the DSS1/DSS2 key blob writer.
//! - `CPImportKey`: [`parse_dss_blob`], the blob checks from `import_key`; the wrapper's
//!   DSS-only gate and the key install into the container are not ported.
//!
//! Skipped:
//! - `CPAcquireContext`: opens or creates registry-backed key containers.
//! - `CPReleaseContext`: tears down a live container and its registry state.
//! - `CPDecrypt`: spec stub, no implementation.
//! - `CPEncrypt`: spec stub, no implementation.
//! - `CPDeriveKey`: returns FALSE unconditionally in main.c, so there is no logic to port.
//! - `CPDestroyHash`: frees a live SymCrypt hash handle.
//! - `CPDestroyKey`: frees live SymCrypt key handles.
//! - `CPDuplicateHash`: copies live SymCrypt hash state.
//! - `CPDuplicateKey`: copies live SymCrypt key handles.
//! - `CPGenRandom`: fills the buffer through `RtlGenRandom`, an OS call.
//! - `CPGetHashParam`: `HP_HASHVAL` needs a SymCrypt digest of live state.
//! - `CPGetKeyParam`: spec stub, no implementation.
//! - `CPGetUserKey`: returns key handles stored in the registry.
//! - `CPHashData`: appends to a live SymCrypt hash state.
//! - `CPHashSessionKey`: spec stub, no implementation.
//! - `CPSetHashParam`: writes into a live hash handle.
//! - `CPSetKeyParam`: always fails with `ERROR_INVALID_PARAMETER`; nothing to port.
//! - `CPSetProvParam`: spec stub, no implementation.
//! - `DllRegisterServer`: declared in the spec only; main.c has no body for it.
//! - `DllUnregisterServer`: declared in the spec only; main.c has no body for it.
//!
//! `DllMain` and the SymCrypt callback glue are also outside this crate.
//! Error codes are returned as enums instead of Win32 `HRESULT`/`DWORD` values.

const CALG_MD5: u32 = 0x8003;
const CALG_SHA1: u32 = 0x8004;
const CALG_DSS_SIGN: u32 = 0x2200;
const CALG_DH_SF: u32 = 0xaa01;
const AT_SIGNATURE: u32 = 2;
const DSS_SIGNATURE_LEN: u32 = 40;
const DEFAULT_KEY_BITS: u32 = 1024;
const PROV_DSS_DH: u32 = 13;
const PROV_DH_SCHANNEL: u32 = 18;
const CRYPT_FIRST: u32 = 1;
const CRYPT_FLAG_SIGNING: u32 = 0x20;

const PUBLICKEYBLOB: u8 = 0x6;
const PRIVATEKEYBLOB: u8 = 0x7;
const BLOB_VERSION: u8 = 2;
const BLOB_HEADER_LEN: usize = 8;
const DSS_PUBKEY_HEADER_LEN: usize = 8;
const DSS_HEADER_LEN: usize = BLOB_HEADER_LEN + DSS_PUBKEY_HEADER_LEN;
const DSS_Q_LEN: usize = 20;
const DSS_PRIV_LEN: usize = 20;
const DSS_SEED_LEN: usize = 20;
const DSS_COUNT_LEN: usize = 4;

/// Digest length for the hash algorithms `CPCreateHash` accepts.
pub fn hash_len(algid: u32) -> Option<u32> {
    match algid {
        CALG_MD5 => Some(16),
        CALG_SHA1 => Some(20),
        _ => None,
    }
}

/// Signature length for a DSS key or the `AT_SIGNATURE` keyspec.
pub fn signature_len(algid: u32) -> Option<u32> {
    match algid {
        AT_SIGNATURE | CALG_DSS_SIGN => Some(DSS_SIGNATURE_LEN),
        _ => None,
    }
}

/// Failure from [`gen_key_params`], matching the errors `CPGenKey` raises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyGenError {
    /// The bit length is odd (`ERROR_INVALID_PARAMETER`).
    InvalidParameter,
    /// The bit length is not 512, 768 or 1024 (`NTE_BAD_FLAGS`).
    BadFlags,
}

/// Splits `CPGenKey` flags into `(bit length, low flags)`.
///
/// A zero high word selects the default of 1024 bits.
pub fn gen_key_params(flags: u32) -> Result<(u32, u32), KeyGenError> {
    let high = flags >> 16;
    let bitlen = if high != 0 { high } else { DEFAULT_KEY_BITS };
    if bitlen % 2 != 0 {
        return Err(KeyGenError::InvalidParameter);
    }
    match bitlen {
        512 | 768 | 1024 => Ok((bitlen, flags & 0xffff)),
        _ => Err(KeyGenError::BadFlags),
    }
}

/// NUL-terminated provider name for a container type (`PP_NAME`).
pub fn provider_name(container_type: u32) -> &'static [u8] {
    match container_type {
        PROV_DSS_DH => b"Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider\0",
        PROV_DH_SCHANNEL => b"Microsoft DH SChannel Cryptographic Provider\0",
        _ => b"Microsoft Base DSS Cryptographic Provider\0",
    }
}

/// Failure from [`copy_param`] when the destination is too small.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoreData;

/// Copies a provider parameter, as `copy_param` does.
///
/// With `None` the call only reports the required length. Otherwise `dst` must
/// be at least `src.len()` bytes and receives a copy of `src`.
pub fn copy_param(dst: Option<&mut [u8]>, src: &[u8]) -> Result<usize, MoreData> {
    match dst {
        None => Ok(src.len()),
        Some(dst) if dst.len() < src.len() => Err(MoreData),
        Some(dst) => {
            dst[..src.len()].copy_from_slice(src);
            Ok(src.len())
        }
    }
}

/// One entry of the base algorithm list reported by `PP_ENUMALGS_EX`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvAlg {
    pub alg_id: u32,
    pub default_len: u32,
    pub min_len: u32,
    pub max_len: u32,
    pub protocols: u32,
    /// Short name, including the trailing NUL that the Win32 struct carries.
    pub name: &'static [u8],
    /// Long name, including the trailing NUL.
    pub long_name: &'static [u8],
}

static SUPPORTED_BASE_ALGS: [ProvAlg; 3] = [
    ProvAlg {
        alg_id: CALG_SHA1,
        default_len: 160,
        min_len: 160,
        max_len: 160,
        protocols: CRYPT_FLAG_SIGNING,
        name: b"SHA-1\0",
        long_name: b"Secure Hash Algorithm (SHA-1)\0",
    },
    ProvAlg {
        alg_id: CALG_MD5,
        default_len: 128,
        min_len: 128,
        max_len: 128,
        protocols: 0,
        name: b"MD5\0",
        long_name: b"Message Digest 5 (MD5)\0",
    },
    ProvAlg {
        alg_id: CALG_DSS_SIGN,
        default_len: 1024,
        min_len: 512,
        max_len: 1024,
        protocols: CRYPT_FLAG_SIGNING,
        name: b"DSA_SIGN\0",
        long_name: b"Digital Signature Algorithm\0",
    },
];

/// Advances the enumeration cursor and returns the entry it now points at.
///
/// `CRYPT_FIRST` restarts at the first entry; any other call moves forward.
/// `None` means the list is exhausted.
pub fn next_base_alg(alg_idx: &mut u32, flags: u32) -> Option<&'static ProvAlg> {
    *alg_idx = if flags & CRYPT_FIRST != 0 {
        0
    } else {
        alg_idx.wrapping_add(1)
    };
    SUPPORTED_BASE_ALGS.get(*alg_idx as usize)
}

/// Byte length of a DSS key blob with the given bit length.
///
/// Matches the size `export_key` reports: header, `p`, `q`, `g`, the public
/// value or private `x`, the 4-byte count, and the seed.
pub fn dss_blob_len(bitlen: u32, private: bool) -> usize {
    let ks = key_size(bitlen);
    DSS_HEADER_LEN + 2 * ks + DSS_Q_LEN + DSS_COUNT_LEN + DSS_SEED_LEN + xy_field_len(ks, private)
}

fn key_size(bitlen: u32) -> usize {
    (bitlen / 8) as usize
}

fn xy_field_len(ks: usize, private: bool) -> usize {
    if private {
        DSS_PRIV_LEN
    } else {
        ks
    }
}

fn le_u32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

/// The fields of a DSS key blob, borrowed from the blob bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DssBlob<'a> {
    pub alg_id: u32,
    pub private: bool,
    pub bitlen: u32,
    pub p: &'a [u8],
    pub q: &'a [u8],
    pub g: &'a [u8],
    /// The public value `y` or the 20-byte private value `x`.
    pub xy: &'a [u8],
    pub count: u32,
    pub seed: &'a [u8],
}

/// Parses and validates a DSS/DH key blob, as `import_key` does.
///
/// Checks the header type, version, algorithm, bit length and exact total
/// length. The magic is not checked, as in Wine. SymCrypt's group validation
/// is not modelled, so only structural checks happen here.
pub fn parse_dss_blob(input: &[u8]) -> Option<DssBlob<'_>> {
    if input.len() < DSS_HEADER_LEN {
        return None;
    }
    let private = match input[0] {
        PRIVATEKEYBLOB => true,
        PUBLICKEYBLOB => false,
        _ => return None,
    };
    let alg_id = le_u32(&input[4..8]);
    if input[1] != BLOB_VERSION || (alg_id != CALG_DSS_SIGN && alg_id != CALG_DH_SF) {
        return None;
    }
    let bitlen = le_u32(&input[12..16]);
    if (bitlen & 0xf) != 0 || input.len() != dss_blob_len(bitlen, private) {
        return None;
    }

    let ks = key_size(bitlen);
    let p_end = DSS_HEADER_LEN + ks;
    let q_end = p_end + DSS_Q_LEN;
    let g_end = q_end + ks;
    let xy_end = g_end + xy_field_len(ks, private);
    let count_end = xy_end + DSS_COUNT_LEN;
    Some(DssBlob {
        alg_id,
        private,
        bitlen,
        p: &input[DSS_HEADER_LEN..p_end],
        q: &input[p_end..q_end],
        g: &input[q_end..g_end],
        xy: &input[g_end..xy_end],
        count: le_u32(&input[xy_end..count_end]),
        seed: &input[count_end..count_end + DSS_SEED_LEN],
    })
}

/// Failure from [`encode_dss_blob`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobError {
    /// The output is shorter than the blob; the payload is the required length.
    TooSmall(usize),
    /// A field slice does not have the length its bit length requires.
    Malformed,
}

/// Writes a DSS key blob into `out`, as `export_key` does.
///
/// The blob type follows `private`, and the algorithm is always `CALG_DSS_SIGN`,
/// as in Wine. Returns the number of bytes written.
pub fn encode_dss_blob(blob: &DssBlob<'_>, out: &mut [u8]) -> Result<usize, BlobError> {
    let ks = key_size(blob.bitlen);
    if blob.p.len() != ks
        || blob.q.len() != DSS_Q_LEN
        || blob.g.len() != ks
        || blob.xy.len() != xy_field_len(ks, blob.private)
        || blob.seed.len() != DSS_SEED_LEN
    {
        return Err(BlobError::Malformed);
    }
    let len = dss_blob_len(blob.bitlen, blob.private);
    if out.len() < len {
        return Err(BlobError::TooSmall(len));
    }

    let (btype, magic) = if blob.private {
        (PRIVATEKEYBLOB, b"DSS2")
    } else {
        (PUBLICKEYBLOB, b"DSS1")
    };
    let mut pos = 0;
    put(out, &mut pos, &[btype, BLOB_VERSION, 0, 0]);
    put(out, &mut pos, &CALG_DSS_SIGN.to_le_bytes());
    put(out, &mut pos, magic);
    put(out, &mut pos, &blob.bitlen.to_le_bytes());
    put(out, &mut pos, blob.p);
    put(out, &mut pos, blob.q);
    put(out, &mut pos, blob.g);
    put(out, &mut pos, blob.xy);
    put(out, &mut pos, &blob.count.to_le_bytes());
    put(out, &mut pos, blob.seed);
    Ok(len)
}

fn put(out: &mut [u8], pos: &mut usize, src: &[u8]) {
    out[*pos..*pos + src.len()].copy_from_slice(src);
    *pos += src.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Count written by the oracle's blob pattern.
    const COUNT: u32 = 0x0102_0304;

    /// FNV-1a 32-bit, the checksum the oracle prints.
    fn fnv(bytes: &[u8]) -> u32 {
        bytes.iter().fold(0x811c_9dc5u32, |h, &b| {
            (h ^ u32::from(b)).wrapping_mul(0x0100_0193)
        })
    }

    /// Mirrors `pat` in `oracle.c`: `seed * 31 + i * 7 + 1`, truncated to a byte.
    fn fill(seed: u32, n: usize) -> Vec<u8> {
        (0..n)
            .map(|i| (seed * 31 + i as u32 * 7 + 1) as u8)
            .collect()
    }

    /// Byte offset of `part` inside `base`; `part` must be a subslice of `base`.
    fn offset(base: &[u8], part: &[u8]) -> usize {
        part.as_ptr() as usize - base.as_ptr() as usize
    }

    /// Mirrors `make_blob` in `oracle.c`.
    fn make_blob(btype: u8, version: u8, alg: u32, bitlen: u32) -> Vec<u8> {
        let private = btype == PRIVATEKEYBLOB;
        let ks = bitlen as usize / 8;
        let mut b = vec![btype, version, 0, 0];
        b.extend_from_slice(&alg.to_le_bytes());
        b.extend_from_slice(if private { b"DSS2" } else { b"DSS1" });
        b.extend_from_slice(&bitlen.to_le_bytes());
        b.extend(fill(1, ks));
        b.extend(fill(2, 20));
        b.extend(fill(3, ks));
        b.extend(fill(4, xy_field_len(ks, private)));
        b.extend_from_slice(&COUNT.to_le_bytes());
        b.extend(fill(5, 20));
        b
    }

    /// Expected field layout of an accepted blob, as printed by the oracle's `parse` lines.
    struct Accept {
        alg_id: u32,
        private: bool,
        bitlen: u32,
        p: usize,
        q: usize,
        g: usize,
        xy: usize,
        xy_len: usize,
        seed: usize,
    }

    fn assert_accept(input: &[u8], want: &Accept) {
        let blob = parse_dss_blob(input).expect("oracle accepts this blob");
        assert_eq!(blob.alg_id, want.alg_id);
        assert_eq!(blob.private, want.private);
        assert_eq!(blob.bitlen, want.bitlen);
        assert_eq!(offset(input, blob.p), want.p);
        assert_eq!(offset(input, blob.q), want.q);
        assert_eq!(offset(input, blob.g), want.g);
        assert_eq!(offset(input, blob.xy), want.xy);
        assert_eq!(blob.xy.len(), want.xy_len);
        assert_eq!(offset(input, blob.seed), want.seed);
        assert_eq!(blob.count, COUNT);
        assert_eq!(blob.p.len(), want.bitlen as usize / 8);
        assert_eq!(blob.g.len(), want.bitlen as usize / 8);
        assert_eq!(blob.q.len(), DSS_Q_LEN);
        assert_eq!(blob.seed.len(), DSS_SEED_LEN);
    }

    /// Owned field bytes for building a blob to encode.
    struct Parts {
        p: Vec<u8>,
        q: Vec<u8>,
        g: Vec<u8>,
        xy: Vec<u8>,
        seed: Vec<u8>,
    }

    impl Parts {
        fn new(bitlen: u32, private: bool) -> Self {
            let ks = bitlen as usize / 8;
            Parts {
                p: fill(1, ks),
                q: fill(2, 20),
                g: fill(3, ks),
                xy: fill(4, xy_field_len(ks, private)),
                seed: fill(5, 20),
            }
        }

        fn blob(&self, bitlen: u32, private: bool) -> DssBlob<'_> {
            DssBlob {
                alg_id: CALG_DSS_SIGN,
                private,
                bitlen,
                p: &self.p,
                q: &self.q,
                g: &self.g,
                xy: &self.xy,
                count: COUNT,
                seed: &self.seed,
            }
        }
    }

    #[test]
    fn hash_len_matches_oracle() {
        assert_eq!(hash_len(CALG_MD5), Some(16));
        assert_eq!(hash_len(CALG_SHA1), Some(20));
        assert_eq!(hash_len(0x800c), None);
    }

    #[test]
    fn signature_len_matches_oracle() {
        assert_eq!(signature_len(CALG_DSS_SIGN), Some(40));
        assert_eq!(signature_len(0x2400), None);
        // Sweep case: `oracle sweep` reports the AT_SIGNATURE keyspec as 40.
        assert_eq!(signature_len(AT_SIGNATURE), Some(40));
    }

    #[test]
    fn gen_key_params_matches_oracle() {
        assert_eq!(gen_key_params(0), Ok((1024, 0)));
        assert_eq!(gen_key_params(0x0200_0001), Ok((512, 1)));
        assert_eq!(
            gen_key_params(0x0401_0000),
            Err(KeyGenError::InvalidParameter)
        );
        assert_eq!(gen_key_params(0x0800_0000), Err(KeyGenError::BadFlags));
        // Sweep case: 768 bits is accepted.
        assert_eq!(gen_key_params(0x0300_0000), Ok((768, 0)));
    }

    #[test]
    fn provider_name_matches_oracle() {
        let rows = [
            (
                13,
                "Microsoft Enhanced DSS and Diffie-Hellman Cryptographic Provider",
                65,
            ),
            (18, "Microsoft DH SChannel Cryptographic Provider", 45),
            (3, "Microsoft Base DSS Cryptographic Provider", 42),
        ];
        for (container_type, text, oracle_len) in rows {
            let want = [text.as_bytes(), &[0]].concat();
            assert_eq!(provider_name(container_type), want.as_slice());
            assert_eq!(provider_name(container_type).len(), oracle_len);
        }
    }

    #[test]
    fn copy_param_matches_oracle() {
        let name = provider_name(3);
        assert_eq!(copy_param(None, name), Ok(42));

        let mut small = [0xaau8; 8];
        assert_eq!(copy_param(Some(&mut small), name), Err(MoreData));
        assert_eq!(small, [0xaa; 8]);

        let mut exact = [0u8; 42];
        assert_eq!(copy_param(Some(&mut exact), name), Ok(42));
        assert_eq!(fnv(&exact), 0x8ada_c336);

        // Sweep case: a larger buffer keeps its tail untouched.
        let mut big = [0xaau8; 43];
        assert_eq!(copy_param(Some(&mut big), name), Ok(42));
        assert_eq!(fnv(&big[..42]), 0x8ada_c336);
        assert_eq!(big[42], 0xaa);
    }

    #[test]
    fn next_base_alg_matches_oracle() {
        let sha1 = ProvAlg {
            alg_id: CALG_SHA1,
            default_len: 160,
            min_len: 160,
            max_len: 160,
            protocols: CRYPT_FLAG_SIGNING,
            name: b"SHA-1\0",
            long_name: b"Secure Hash Algorithm (SHA-1)\0",
        };
        let md5 = ProvAlg {
            alg_id: CALG_MD5,
            default_len: 128,
            min_len: 128,
            max_len: 128,
            protocols: 0,
            name: b"MD5\0",
            long_name: b"Message Digest 5 (MD5)\0",
        };
        let dsa = ProvAlg {
            alg_id: CALG_DSS_SIGN,
            default_len: 1024,
            min_len: 512,
            max_len: 1024,
            protocols: CRYPT_FLAG_SIGNING,
            name: b"DSA_SIGN\0",
            long_name: b"Digital Signature Algorithm\0",
        };
        let rows = [
            (CRYPT_FIRST, Some(sha1)),
            (0, Some(md5)),
            (0, Some(dsa)),
            (0, None),
        ];
        let mut idx = 0;
        for (flags, want) in rows {
            assert_eq!(next_base_alg(&mut idx, flags), want.as_ref());
        }
        assert_eq!(idx, 3);
    }

    #[test]
    fn dss_blob_len_matches_oracle() {
        assert_eq!(dss_blob_len(512, true), 208);
        assert_eq!(dss_blob_len(768, false), 348);
    }

    #[test]
    fn parse_dss_blob_matches_oracle() {
        let priv512 = make_blob(PRIVATEKEYBLOB, 2, CALG_DSS_SIGN, 512);
        assert_accept(
            &priv512,
            &Accept {
                alg_id: CALG_DSS_SIGN,
                private: true,
                bitlen: 512,
                p: 16,
                q: 80,
                g: 100,
                xy: 164,
                xy_len: 20,
                seed: 188,
            },
        );

        let pub1024 = make_blob(PUBLICKEYBLOB, 2, CALG_DH_SF, 1024);
        assert_accept(
            &pub1024,
            &Accept {
                alg_id: CALG_DH_SF,
                private: false,
                bitlen: 1024,
                p: 16,
                q: 144,
                g: 164,
                xy: 292,
                xy_len: 128,
                seed: 424,
            },
        );

        assert_eq!(parse_dss_blob(&priv512[..7]), None);
        assert_eq!(parse_dss_blob(&make_blob(3, 2, CALG_DSS_SIGN, 512)), None);
    }

    #[test]
    fn parse_dss_blob_branch_cases() {
        assert_accept(
            &make_blob(PUBLICKEYBLOB, 2, CALG_DSS_SIGN, 768),
            &Accept {
                alg_id: CALG_DSS_SIGN,
                private: false,
                bitlen: 768,
                p: 16,
                q: 112,
                g: 132,
                xy: 228,
                xy_len: 96,
                seed: 328,
            },
        );
        assert_accept(
            &make_blob(PRIVATEKEYBLOB, 2, CALG_DSS_SIGN, 1024),
            &Accept {
                alg_id: CALG_DSS_SIGN,
                private: true,
                bitlen: 1024,
                p: 16,
                q: 144,
                g: 164,
                xy: 292,
                xy_len: 20,
                seed: 316,
            },
        );
        assert_accept(
            &make_blob(PUBLICKEYBLOB, 2, CALG_DSS_SIGN, 512),
            &Accept {
                alg_id: CALG_DSS_SIGN,
                private: false,
                bitlen: 512,
                p: 16,
                q: 80,
                g: 100,
                xy: 164,
                xy_len: 64,
                seed: 232,
            },
        );

        let priv512 = make_blob(PRIVATEKEYBLOB, 2, CALG_DSS_SIGN, 512);
        let mut trailing = priv512.clone();
        trailing.push(0);
        assert_eq!(
            parse_dss_blob(&make_blob(PRIVATEKEYBLOB, 1, CALG_DSS_SIGN, 512)),
            None
        );
        assert_eq!(
            parse_dss_blob(&make_blob(PRIVATEKEYBLOB, 2, CALG_SHA1, 512)),
            None
        );
        assert_eq!(
            parse_dss_blob(&make_blob(PRIVATEKEYBLOB, 2, 0x2000, 512)),
            None
        );
        assert_eq!(
            parse_dss_blob(&make_blob(PRIVATEKEYBLOB, 2, CALG_DSS_SIGN, 520)),
            None
        );
        assert_eq!(parse_dss_blob(&priv512[..15]), None);
        assert_eq!(parse_dss_blob(&priv512[..16]), None);
        assert_eq!(parse_dss_blob(&trailing), None);
    }

    #[test]
    fn encode_dss_blob_matches_oracle() {
        let priv512 = Parts::new(512, true);
        let mut out = vec![0u8; dss_blob_len(512, true)];
        assert_eq!(encode_dss_blob(&priv512.blob(512, true), &mut out), Ok(208));
        assert_eq!(fnv(&out), 0xa86f_d1cc);

        let pub768 = Parts::new(768, false);
        let mut small = vec![0u8; 347];
        assert_eq!(
            encode_dss_blob(&pub768.blob(768, false), &mut small),
            Err(BlobError::TooSmall(348))
        );

        // Sweep cases: a zero-length output reports the size; the full public blob encodes.
        assert_eq!(
            encode_dss_blob(&priv512.blob(512, true), &mut []),
            Err(BlobError::TooSmall(208))
        );
        let mut full = vec![0u8; 348];
        assert_eq!(
            encode_dss_blob(&pub768.blob(768, false), &mut full),
            Ok(348)
        );
        assert_eq!(fnv(&full), 0xec90_8057);
    }

    #[test]
    fn encode_dss_blob_roundtrips_parsed_blobs() {
        for input in [
            make_blob(PRIVATEKEYBLOB, 2, CALG_DSS_SIGN, 512),
            make_blob(PUBLICKEYBLOB, 2, CALG_DSS_SIGN, 768),
        ] {
            let blob = parse_dss_blob(&input).expect("valid blob");
            let mut out = vec![0u8; input.len()];
            assert_eq!(encode_dss_blob(&blob, &mut out), Ok(input.len()));
            assert_eq!(out, input);
        }
    }

    #[test]
    fn encode_dss_blob_rejects_malformed_fields() {
        type Mutate = fn(&mut Parts);
        let mutations: [Mutate; 6] = [
            |m| {
                m.p.pop();
            },
            |m| m.q.push(0),
            |m| m.g.clear(),
            |m| {
                m.xy.pop();
            },
            |m| m.seed.truncate(19),
            |m| m.xy.truncate(19),
        ];
        let mut out = vec![0u8; 348];
        for mutate in mutations {
            let mut parts = Parts::new(512, true);
            mutate(&mut parts);
            assert_eq!(
                encode_dss_blob(&parts.blob(512, true), &mut out),
                Err(BlobError::Malformed)
            );
        }

        let mut parts = Parts::new(768, false);
        parts.xy.pop();
        assert_eq!(
            encode_dss_blob(&parts.blob(768, false), &mut out),
            Err(BlobError::Malformed)
        );
    }
}
