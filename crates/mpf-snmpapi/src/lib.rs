//! snmpapi subset for MPF: the OID, `AsnAny` and VarBind helpers of `dlls/snmpapi`.
//!
//! Reference Wine's `dlls/snmpapi/main.c` has bodies for 24 of the 45 entries in
//! `snmpapi.spec`; the other 21 are spec-only stubs. Thirteen of the 24 are ported
//! here. They work on plain values: an OID is a `Vec<u32>`, an `AsnAny` is an enum,
//! and a VarBind pairs an OID with an `AsnAny`. No SNMP session, transport, registry
//! or live agent state is read.
//!
//! Ported (table tests against a verbatim Wine C oracle, see the tests):
//! - `SnmpUtilOidNCmp` as `oid_ncmp` and `SnmpUtilOidCmp` as `oid_cmp`.
//! - `SnmpUtilOidCpy` as `oid_cpy`, `SnmpUtilOidAppend` as `oid_append` and
//!   `SnmpUtilOidFree` as `oid_free`.
//! - `SnmpUtilIdsToA` as `ids_to_a` and `SnmpUtilOidToA` as `oid_to_a`.
//! - `SnmpUtilAsnAnyCpy` as `asn_any_cpy` and `SnmpUtilAsnAnyFree` as `asn_any_free`.
//! - `SnmpUtilVarBindCpy` as `varbind_cpy`, `SnmpUtilVarBindFree` as `varbind_free`,
//!   `SnmpUtilVarBindListCpy` as `varbind_list_cpy` and `SnmpUtilVarBindListFree`
//!   as `varbind_list_free`.
//!
//! Divergences from the C bodies:
//! - `AsnAny::Null` is rejected by `asn_any_cpy`, as the C switch rejects `ASN_NULL`
//!   (it has no case for it). C zeroes `dst` before that rejection; the port leaves
//!   `dst` untouched.
//! - On failure, `varbind_cpy` and `varbind_list_cpy` leave `dst` untouched. The C
//!   bodies leave name storage freed but still referenced, and `varbind_list_cpy`
//!   leaks entry 0 when entry `k >= 1` fails. When entry 0 fails, C frees memory
//!   before the array; the oracle reports that as undefined behaviour.
//! - `ids_to_a` prints every component in full. C formats each component into a
//!   10-byte buffer, so a component of 10 or more characters overflows it.
//! - The C NULL and empty pointer states are not separate. `None` stands for a NULL
//!   pointer where C accepts one, and an empty `Vec` for a zero-length OID.
//! - `asn_any_free` ignores the C `dynamic` flag: every byte string is owned.
//! - `TRACE`, `WARN` and `FIXME` logging is not modelled. Nor is the allocation
//!   failure branch of `SnmpUtilOidAppend`.
//!
//! Skipped (32 of 45 spec entries):
//! - 21 spec-only entries with no body in `main.c`: nothing to port.
//! - `SnmpSvcGetUptime`: reports the live tick count since `DllMain`.
//! - `DllMain`: not in the spec. It only records the start time.
//! - `SnmpUtilDbgPrint`: its body only logs `FIXME`.
//! - `SnmpUtilMemAlloc`, `SnmpUtilMemReAlloc` and `SnmpUtilMemFree`: thin allocator
//!   wrappers with no logic to port.
//! - `SnmpUtilPrintOid` and `SnmpUtilPrintAsnAny`: their bodies only log `TRACE`.
//! - `SnmpUtilOctetsCpy`, `SnmpUtilOctetsFree`, `SnmpUtilOctetsNCmp` and
//!   `SnmpUtilOctetsCmp`: octet-string helpers outside this first pass. They are a
//!   follow-up candidate.

/// The error of the C bodies that return `SNMPAPI_ERROR`.
pub struct SnmpError;

/// An `AsnAny` value: the ASN type and its payload. `Null` has no payload.
pub enum AsnAny {
    Null,
    Integer32(i32),
    Unsigned32(u32),
    Counter64(u64),
    Counter32(u32),
    Gauge32(u32),
    Timeticks(u32),
    OctetString(Vec<u8>),
    Bits(Vec<u8>),
    Sequence(Vec<u8>),
    IpAddress(Vec<u8>),
    Opaque(Vec<u8>),
    ObjectIdentifier(Vec<u32>),
}

/// An OID name and the value bound to it.
pub struct VarBind {
    pub name: Vec<u32>,
    pub value: AsnAny,
}

const NULL_OID: &str = "<null oid>";

/// Components in `SnmpUtilIdsToA` are written into a 513-character buffer.
const IDS_TO_A_CAPACITY: usize = 513;

/// `SnmpUtilOidNCmp`: compares at most `count` components. A `None` operand gives 0.
pub fn oid_ncmp(oid1: Option<&[u32]>, oid2: Option<&[u32]>, count: u32) -> i32 {
    match (oid1, oid2) {
        (Some(oid1), Some(oid2)) => ncmp_slices(oid1, oid2, count as usize),
        _ => 0,
    }
}

/// `SnmpUtilOidCmp`: orders by length first, then by the components.
pub fn oid_cmp(oid1: &[u32], oid2: &[u32]) -> i32 {
    if oid1.len() < oid2.len() {
        return -1;
    }
    if oid1.len() > oid2.len() {
        return 1;
    }
    ncmp_slices(oid1, oid2, oid1.len())
}

fn ncmp_slices(oid1: &[u32], oid2: &[u32], count: usize) -> i32 {
    let len = count.min(oid1.len()).min(oid2.len());
    for (a, b) in oid1[..len].iter().zip(&oid2[..len]) {
        if a != b {
            return if a > b { 1 } else { -1 };
        }
    }
    if len == count {
        return 0;
    }
    if oid1.len() < oid2.len() {
        -1
    } else if oid1.len() > oid2.len() {
        1
    } else {
        0
    }
}

/// `SnmpUtilOidCpy`: `dst` takes a copy of `src`, or is emptied when `src` is `None`.
pub fn oid_cpy(dst: Option<&mut Vec<u32>>, src: Option<&[u32]>) -> Result<(), SnmpError> {
    let Some(dst) = dst else {
        return Err(SnmpError);
    };
    match src {
        Some(src) => *dst = src.to_vec(),
        None => *dst = Vec::new(),
    }
    Ok(())
}

/// `SnmpUtilOidAppend`: appends `src` to `dst`. A `None` `src` changes nothing.
pub fn oid_append(dst: Option<&mut Vec<u32>>, src: Option<&[u32]>) -> Result<(), SnmpError> {
    let Some(dst) = dst else {
        return Err(SnmpError);
    };
    if let Some(src) = src {
        dst.extend_from_slice(src);
    }
    Ok(())
}

/// `SnmpUtilOidFree`: empties the OID. A `None` OID changes nothing.
pub fn oid_free(oid: Option<&mut Vec<u32>>) {
    if let Some(oid) = oid {
        *oid = Vec::new();
    }
}

/// `SnmpUtilIdsToA`: the dotted decimal text of `ids`, cut short once the 513-character
/// buffer is full. Components print as signed 32-bit values, as C's `%d` does.
/// An empty list gives `<null oid>`.
pub fn ids_to_a(ids: &[u32]) -> String {
    if ids.is_empty() {
        return NULL_OID.to_string();
    }
    let mut out = String::new();
    let mut left = IDS_TO_A_CAPACITY;
    for (i, id) in ids.iter().enumerate() {
        let one = (*id as i32).to_string();
        if left < one.len() {
            return out;
        }
        out.push_str(&one);
        left -= one.len();
        if i + 1 < ids.len() {
            if left == 0 {
                return out;
            }
            out.push('.');
            left -= 1;
        }
    }
    out
}

/// `SnmpUtilOidToA`: `None` gives `<null oid>`, as an empty OID does.
pub fn oid_to_a(oid: Option<&[u32]>) -> String {
    ids_to_a(oid.unwrap_or(&[]))
}

/// `SnmpUtilAsnAnyCpy`: `dst` takes a copy of `src`. `Null` is rejected, as in C.
pub fn asn_any_cpy(dst: &mut AsnAny, src: &AsnAny) -> Result<(), SnmpError> {
    *dst = copy_any(src)?;
    Ok(())
}

fn copy_any(src: &AsnAny) -> Result<AsnAny, SnmpError> {
    Ok(match src {
        AsnAny::Null => return Err(SnmpError),
        AsnAny::Integer32(v) => AsnAny::Integer32(*v),
        AsnAny::Unsigned32(v) => AsnAny::Unsigned32(*v),
        AsnAny::Counter64(v) => AsnAny::Counter64(*v),
        AsnAny::Counter32(v) => AsnAny::Counter32(*v),
        AsnAny::Gauge32(v) => AsnAny::Gauge32(*v),
        AsnAny::Timeticks(v) => AsnAny::Timeticks(*v),
        AsnAny::OctetString(b) => AsnAny::OctetString(b.clone()),
        AsnAny::Bits(b) => AsnAny::Bits(b.clone()),
        AsnAny::Sequence(b) => AsnAny::Sequence(b.clone()),
        AsnAny::IpAddress(b) => AsnAny::IpAddress(b.clone()),
        AsnAny::Opaque(b) => AsnAny::Opaque(b.clone()),
        AsnAny::ObjectIdentifier(ids) => AsnAny::ObjectIdentifier(ids.clone()),
    })
}

/// `SnmpUtilAsnAnyFree`: releases the payload and leaves `Null`.
pub fn asn_any_free(any: &mut AsnAny) {
    *any = AsnAny::Null;
}

/// `SnmpUtilVarBindCpy`: `dst` takes a copy of `src`. A `None` `src` leaves the name
/// and sets the value to `Null`. On failure `dst` is untouched.
pub fn varbind_cpy(dst: Option<&mut VarBind>, src: Option<&VarBind>) -> Result<(), SnmpError> {
    let Some(dst) = dst else {
        return Err(SnmpError);
    };
    let Some(src) = src else {
        dst.value = AsnAny::Null;
        return Ok(());
    };
    let value = copy_any(&src.value)?;
    dst.name = src.name.clone();
    dst.value = value;
    Ok(())
}

/// `SnmpUtilVarBindFree`: empties the name and the value. A `None` VarBind changes nothing.
pub fn varbind_free(vb: Option<&mut VarBind>) {
    if let Some(vb) = vb {
        vb.value = AsnAny::Null;
        vb.name = Vec::new();
    }
}

/// `SnmpUtilVarBindListCpy`: `dst` takes a copy of `src`, or is emptied when `src` is
/// `None`. On failure `dst` is untouched.
pub fn varbind_list_cpy(dst: &mut Vec<VarBind>, src: Option<&[VarBind]>) -> Result<(), SnmpError> {
    let Some(src) = src else {
        dst.clear();
        return Ok(());
    };
    let mut list = Vec::with_capacity(src.len());
    for entry in src {
        let mut copy = VarBind {
            name: Vec::new(),
            value: AsnAny::Null,
        };
        varbind_cpy(Some(&mut copy), Some(entry))?;
        list.push(copy);
    }
    *dst = list;
    Ok(())
}

/// `SnmpUtilVarBindListFree`: releases every entry and empties the list.
pub fn varbind_list_free(vb: &mut Vec<VarBind>) {
    vb.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dump_oid(ids: &[u32]) -> String {
        let parts: Vec<String> = ids.iter().map(u32::to_string).collect();
        format!("oid[{}]", parts.join("."))
    }

    fn dump_any(any: &AsnAny) -> String {
        fn hex(bytes: &[u8]) -> String {
            let parts: Vec<String> = bytes.iter().map(|b| format!("{b:02x}")).collect();
            format!("[{}]", parts.concat())
        }
        match any {
            AsnAny::Null => "any:Null".to_string(),
            AsnAny::Integer32(v) => format!("any:Integer32={v}"),
            AsnAny::Unsigned32(v) => format!("any:Unsigned32={v}"),
            AsnAny::Counter64(v) => format!("any:Counter64={v}"),
            AsnAny::Counter32(v) => format!("any:Counter32={v}"),
            AsnAny::Gauge32(v) => format!("any:Gauge32={v}"),
            AsnAny::Timeticks(v) => format!("any:Timeticks={v}"),
            AsnAny::OctetString(b) => format!("any:OctetString={}", hex(b)),
            AsnAny::Bits(b) => format!("any:Bits={}", hex(b)),
            AsnAny::Sequence(b) => format!("any:Sequence={}", hex(b)),
            AsnAny::IpAddress(b) => format!("any:IpAddress={}", hex(b)),
            AsnAny::Opaque(b) => format!("any:Opaque={}", hex(b)),
            AsnAny::ObjectIdentifier(ids) => format!("any:ObjectIdentifier={}", dump_oid(ids)),
        }
    }

    fn dump_vb(vb: &VarBind) -> String {
        format!(
            "vb{{name={} value={}}}",
            dump_oid(&vb.name),
            dump_any(&vb.value)
        )
    }

    fn dump_vbl(list: &[VarBind]) -> String {
        let parts: Vec<String> = list.iter().map(dump_vb).collect();
        format!("vbl[{}]", parts.join(", "))
    }

    /// The oracle's line for a call returning `SNMPAPI_*`: `rc=0`, or `rc=1` and the
    /// dump of the destination on success.
    fn line(rc: Result<(), SnmpError>, dump: Option<String>) -> String {
        match (rc, dump) {
            (Ok(()), Some(dump)) => format!("rc=1 {dump}"),
            _ => "rc=0".to_string(),
        }
    }

    #[test]
    fn ncmp_and_cmp_treat_a_missing_operand_as_zero() {
        assert_eq!(oid_ncmp(None, Some(&[1]), 1), 0);
        assert_eq!(oid_ncmp(Some(&[1]), None, 1), 0);
        assert_eq!(oid_cmp(&[], &[]), 0);
    }

    #[test]
    fn ids_to_a_keeps_components_of_ten_characters_in_full() {
        // The C body writes each component into a 10-byte buffer and overflows it here.
        assert_eq!(ids_to_a(&[1_000_000_000, 4_294_967_295]), "1000000000.-1");
        assert_eq!(ids_to_a(&[0x8000_0000]), "-2147483648");
    }

    #[test]
    fn ids_to_a_stops_at_the_513_character_buffer() {
        let text = ids_to_a(&[1; 1000]);
        assert_eq!(text.len(), IDS_TO_A_CAPACITY);
        assert!(text.starts_with("1.1.1"));
    }

    #[test]
    fn oid_to_a_of_none_is_the_null_oid_text() {
        assert_eq!(oid_to_a(None), NULL_OID);
    }

    #[test]
    fn varbind_list_cpy_first_entry_failure_leaves_dst_unchanged() {
        let pre = VarBind {
            name: vec![7],
            value: AsnAny::Integer32(9),
        };
        let mut dst = vec![pre];
        let src = [VarBind {
            name: vec![1],
            value: AsnAny::Null,
        }];
        let before = dump_vbl(&dst);
        assert!(varbind_list_cpy(&mut dst, Some(&src)).is_err());
        assert_eq!(dump_vbl(&dst), before);
    }

    #[test]
    fn varbind_cpy_null_value_keeps_name_and_value() {
        let mut dst = VarBind {
            name: vec![7],
            value: AsnAny::Integer32(9),
        };
        let src = VarBind {
            name: vec![1, 3],
            value: AsnAny::Null,
        };
        assert!(varbind_cpy(Some(&mut dst), Some(&src)).is_err());
        assert_eq!(dump_vb(&dst), "vb{name=oid[7] value=any:Integer32=9}");
    }

    #[test]
    fn asn_any_cpy_null_keeps_dst() {
        let mut dst = AsnAny::Integer32(3);
        assert!(asn_any_cpy(&mut dst, &AsnAny::Null).is_err());
        assert_eq!(dump_any(&dst), "any:Integer32=3");
    }

    #[test]
    fn varbind_cpy_none_source_sets_value_null_and_keeps_name() {
        let mut dst = VarBind {
            name: vec![7],
            value: AsnAny::Integer32(9),
        };
        assert!(varbind_cpy(Some(&mut dst), None).is_ok());
        assert_eq!(dump_vb(&dst), "vb{name=oid[7] value=any:Null}");
    }

    #[test]
    fn varbind_list_cpy_none_source_empties_dst() {
        let mut dst = vec![VarBind {
            name: vec![7],
            value: AsnAny::Null,
        }];
        assert!(varbind_list_cpy(&mut dst, None).is_ok());
        assert_eq!(dump_vbl(&dst), "vbl[]");
    }

    // BEGIN ORACLE ROWS (generated by target/canon/snmpapi/gen.py; do not edit)
    // Oracle rows of `SnmpUtilOidNCmp`: 22 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_ncmp() {
        let rows = [
            (None, Some(vec![1]), 1, "rc=0"),
            (Some(vec![1]), None, 1, "rc=0"),
            (None, None, 0, "rc=0"),
            (Some(Vec::new()), Some(Vec::new()), 0, "rc=0"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 6]), 3, "rc=0"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 7]), 3, "rc=-1"),
            (Some(vec![1, 3, 7]), Some(vec![1, 3, 6]), 3, "rc=1"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 6, 1]), 4, "rc=-1"),
            (Some(vec![1, 3, 6, 1]), Some(vec![1, 3, 6]), 4, "rc=1"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 6, 1]), 3, "rc=0"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 6, 1]), 2, "rc=0"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3]), 2, "rc=0"),
            (Some(vec![1, 3]), Some(vec![1, 3, 6]), 3, "rc=-1"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3]), 3, "rc=1"),
            (Some(vec![1, 3, 6]), Some(vec![1, 4]), 1, "rc=0"),
            (Some(vec![2]), Some(vec![1, 9]), 2, "rc=1"),
            (Some(vec![1, 2]), Some(vec![1, 9]), 2, "rc=-1"),
            (Some(Vec::new()), Some(vec![1]), 5, "rc=-1"),
            (Some(vec![1]), Some(Vec::new()), 5, "rc=1"),
            (Some(vec![4294967295]), Some(vec![0]), 1, "rc=1"),
            (Some(vec![1, 3, 6]), Some(vec![1, 3, 6]), 0, "rc=0"),
            (Some(vec![1, 3]), Some(vec![1, 3]), 5, "rc=0"),
        ];
        for (i, (oid1, oid2, count, want)) in rows.into_iter().enumerate() {
            let got = format!("rc={}", oid_ncmp(oid1.as_deref(), oid2.as_deref(), count));
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilOidCmp`: 16 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_cmp() {
        let rows = [
            (Vec::new(), Vec::new(), "rc=0"),
            (vec![1], Vec::new(), "rc=1"),
            (Vec::new(), vec![1], "rc=-1"),
            (vec![1, 3], vec![1, 3], "rc=0"),
            (vec![1, 3], vec![1, 4], "rc=-1"),
            (vec![1, 5], vec![1, 4], "rc=1"),
            (vec![1, 3], vec![1, 3, 6], "rc=-1"),
            (vec![1, 3, 6], vec![1, 3], "rc=1"),
            (vec![2], vec![10], "rc=-1"),
            (vec![4294967295], vec![1], "rc=1"),
            (vec![0], vec![0], "rc=0"),
            (vec![1, 2, 3, 4], vec![1, 2, 3, 5], "rc=-1"),
            (vec![9, 9, 9], vec![1, 1, 1], "rc=1"),
            (vec![1, 3, 6, 1, 2, 1], vec![1, 3, 6, 1, 2, 1], "rc=0"),
            (vec![1, 3, 6, 1, 2, 1, 5], vec![1, 3, 6, 1, 2, 1, 4], "rc=1"),
            (vec![0, 0], vec![0, 0, 0], "rc=-1"),
        ];
        for (i, (oid1, oid2, want)) in rows.into_iter().enumerate() {
            let got = format!("rc={}", oid_cmp(&oid1, &oid2));
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilOidCpy`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_cpy() {
        let rows = [
            (None, Some(vec![1]), "rc=0"),
            (Some(vec![7, 7]), None, "rc=1 oid[]"),
            (Some(vec![7, 7]), Some(Vec::new()), "rc=1 oid[]"),
            (Some(vec![7, 7]), Some(vec![1]), "rc=1 oid[1]"),
            (
                Some(vec![7, 7]),
                Some(vec![1, 3, 6, 1, 2, 1, 1, 1, 0]),
                "rc=1 oid[1.3.6.1.2.1.1.1.0]",
            ),
            (Some(Vec::new()), Some(vec![5]), "rc=1 oid[5]"),
            (Some(Vec::new()), Some(Vec::new()), "rc=1 oid[]"),
            (Some(vec![7]), Some(vec![0, 0]), "rc=1 oid[0.0]"),
            (
                Some(vec![1, 2, 3]),
                Some(vec![4294967295]),
                "rc=1 oid[4294967295]",
            ),
            (
                Some(vec![7, 7]),
                Some(vec![1, 3, 6, 1, 4, 1, 311, 1, 1, 3, 1]),
                "rc=1 oid[1.3.6.1.4.1.311.1.1.3.1]",
            ),
            (None, None, "rc=0"),
            (Some(vec![9, 9, 9, 9]), Some(vec![2]), "rc=1 oid[2]"),
            (Some(vec![7, 7]), Some(vec![1, 0]), "rc=1 oid[1.0]"),
            (
                Some(Vec::new()),
                Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
                "rc=1 oid[1.2.3.4.5.6.7.8.9.10]",
            ),
            (
                Some(vec![5, 5]),
                Some(vec![4294967294, 1]),
                "rc=1 oid[4294967294.1]",
            ),
        ];
        for (i, (dst, src, want)) in rows.into_iter().enumerate() {
            let mut dst = dst;
            let before = dst.as_deref().map(dump_oid);
            let rc = oid_cpy(dst.as_mut(), src.as_deref());
            let after = dst.as_deref().map(dump_oid);
            if rc.is_err() {
                assert_eq!(after, before, "row {i}");
            }
            assert_eq!(line(rc, after), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilOidAppend`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_append() {
        let rows = [
            (None, Some(vec![1]), "rc=0"),
            (None, None, "rc=0"),
            (Some(vec![1, 3]), None, "rc=1 oid[1.3]"),
            (Some(vec![1, 3]), Some(Vec::new()), "rc=1 oid[1.3]"),
            (Some(Vec::new()), Some(Vec::new()), "rc=1 oid[]"),
            (Some(Vec::new()), Some(vec![1]), "rc=1 oid[1]"),
            (Some(vec![1, 3]), Some(vec![6]), "rc=1 oid[1.3.6]"),
            (
                Some(vec![1, 3, 6, 1]),
                Some(vec![2, 1, 1]),
                "rc=1 oid[1.3.6.1.2.1.1]",
            ),
            (
                Some(vec![1]),
                Some(vec![4294967295]),
                "rc=1 oid[1.4294967295]",
            ),
            (
                Some(vec![1, 3, 6]),
                Some(vec![1, 3, 6, 1, 2, 1]),
                "rc=1 oid[1.3.6.1.3.6.1.2.1]",
            ),
            (
                Some(vec![0]),
                Some(vec![0, 0, 0, 0, 0, 0, 0, 0]),
                "rc=1 oid[0.0.0.0.0.0.0.0.0]",
            ),
            (Some(vec![9]), Some(vec![8]), "rc=1 oid[9.8]"),
            (
                Some(vec![1, 3, 6, 1, 4, 1, 311]),
                Some(vec![1, 2]),
                "rc=1 oid[1.3.6.1.4.1.311.1.2]",
            ),
            (Some(vec![2, 2]), Some(vec![3]), "rc=1 oid[2.2.3]"),
            (Some(Vec::new()), Some(vec![0]), "rc=1 oid[0]"),
        ];
        for (i, (dst, src, want)) in rows.into_iter().enumerate() {
            let mut dst = dst;
            let before = dst.as_deref().map(dump_oid);
            let rc = oid_append(dst.as_mut(), src.as_deref());
            let after = dst.as_deref().map(dump_oid);
            if rc.is_err() {
                assert_eq!(after, before, "row {i}");
            }
            assert_eq!(line(rc, after), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilOidFree`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_free() {
        let rows = [
            (None, "none"),
            (Some(Vec::new()), "oid[]"),
            (Some(vec![1]), "oid[]"),
            (Some(vec![1, 3]), "oid[]"),
            (Some(vec![1, 3, 6, 1, 2, 1, 1, 1, 0]), "oid[]"),
            (Some(vec![0]), "oid[]"),
            (Some(vec![4294967295]), "oid[]"),
            (Some(vec![5, 5, 5]), "oid[]"),
            (Some(vec![1; 20]), "oid[]"),
            (Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), "oid[]"),
            (Some(vec![2]), "oid[]"),
            (Some(vec![3, 1]), "oid[]"),
            (Some(vec![40, 1]), "oid[]"),
            (Some(vec![7, 7, 7, 7]), "oid[]"),
            (Some(vec![100]), "oid[]"),
        ];
        for (i, (pre, want)) in rows.into_iter().enumerate() {
            let mut oid = pre;
            oid_free(oid.as_mut());
            let got = match oid {
                Some(oid) => dump_oid(&oid),
                None => "none".to_string(),
            };
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilIdsToA`: 17 table vectors from the verbatim C harness.
    #[test]
    fn oracle_ids_to_a() {
        let rows = [
            (Vec::new(), "str=<null oid>"),
            (vec![0], "str=0"),
            (vec![1, 3, 6, 1, 2, 1], "str=1.3.6.1.2.1"),
            (vec![4294967295], "str=-1"),
            (vec![999999999], "str=999999999"),
            (vec![1, 3, 6, 1, 4, 1, 311, 1, 1, 3, 1], "str=1.3.6.1.4.1.311.1.1.3.1"),
            (vec![99999999], "str=99999999"),
            (vec![4194967297], "str=-99999999"),
            (vec![0, 0], "str=0.0"),
            (vec![1, 2], "str=1.2"),
            (vec![7], "str=7"),
            (vec![10, 100, 1000], "str=10.100.1000"),
            (vec![4294967295, 4294967294], "str=-1.-2"),
            (vec![1; 257], "str=1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1"),
            (vec![1; 258], "str=1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1"),
            ([vec![1; 256], vec![10; 1]].concat(), "str=1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1."),
            (vec![10; 172], "str=10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10.10."),
        ];
        for (i, (ids, want)) in rows.into_iter().enumerate() {
            let got = format!("str={}", ids_to_a(&ids));
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilOidToA`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_oid_to_a() {
        let rows = [
            (None, "str=<null oid>"),
            (Some(Vec::new()), "str=<null oid>"),
            (Some(vec![1, 3, 6, 1, 2, 1, 1, 1, 0]), "str=1.3.6.1.2.1.1.1.0"),
            (Some(vec![0]), "str=0"),
            (Some(vec![4294967295]), "str=-1"),
            (Some(vec![2, 999999999]), "str=2.999999999"),
            (Some(vec![1, 3, 6, 1, 4, 1, 311]), "str=1.3.6.1.4.1.311"),
            (Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]), "str=1.2.3.4.5.6.7.8.9.10.11.12.13.14.15.16"),
            (Some(vec![4194967297]), "str=-99999999"),
            (Some(vec![0, 0, 0]), "str=0.0.0"),
            (Some(vec![5]), "str=5"),
            (Some(vec![1, 1]), "str=1.1"),
            (Some(vec![40, 1]), "str=40.1"),
            (Some(vec![1; 100]), "str=1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1.1"),
            (Some(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), "str=9.8.7.6.5.4.3.2.1.0"),
        ];
        for (i, (oid, want)) in rows.into_iter().enumerate() {
            let got = format!("str={}", oid_to_a(oid.as_deref()));
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilAsnAnyCpy`: 20 table vectors from the verbatim C harness.
    #[test]
    fn oracle_asn_any_cpy() {
        let rows = [
            (AsnAny::Integer32(-5), "rc=1 any:Integer32=-5"),
            (AsnAny::Integer32(0), "rc=1 any:Integer32=0"),
            (
                AsnAny::Integer32(2147483647),
                "rc=1 any:Integer32=2147483647",
            ),
            (
                AsnAny::Integer32(-2147483648),
                "rc=1 any:Integer32=-2147483648",
            ),
            (AsnAny::Unsigned32(0), "rc=1 any:Unsigned32=0"),
            (
                AsnAny::Unsigned32(4294967295),
                "rc=1 any:Unsigned32=4294967295",
            ),
            (AsnAny::Counter64(0), "rc=1 any:Counter64=0"),
            (
                AsnAny::Counter64(18446744073709551615),
                "rc=1 any:Counter64=18446744073709551615",
            ),
            (
                AsnAny::Counter32(4294967295),
                "rc=1 any:Counter32=4294967295",
            ),
            (AsnAny::Gauge32(1), "rc=1 any:Gauge32=1"),
            (AsnAny::Timeticks(360000), "rc=1 any:Timeticks=360000"),
            (AsnAny::OctetString(Vec::new()), "rc=1 any:OctetString=[]"),
            (
                AsnAny::OctetString(vec![10, 11, 255]),
                "rc=1 any:OctetString=[0a0bff]",
            ),
            (AsnAny::Bits(vec![128]), "rc=1 any:Bits=[80]"),
            (
                AsnAny::Sequence(vec![1, 2, 3, 4, 5]),
                "rc=1 any:Sequence=[0102030405]",
            ),
            (
                AsnAny::IpAddress(vec![10, 0, 0, 1]),
                "rc=1 any:IpAddress=[0a000001]",
            ),
            (AsnAny::Opaque(vec![222, 173]), "rc=1 any:Opaque=[dead]"),
            (
                AsnAny::ObjectIdentifier(Vec::new()),
                "rc=1 any:ObjectIdentifier=oid[]",
            ),
            (
                AsnAny::ObjectIdentifier(vec![1, 3, 6, 1, 2, 1]),
                "rc=1 any:ObjectIdentifier=oid[1.3.6.1.2.1]",
            ),
            (AsnAny::Null, "rc=0"),
        ];
        for (i, (src, want)) in rows.into_iter().enumerate() {
            let mut dst = AsnAny::Null;
            let before = dump_any(&dst);
            let rc = asn_any_cpy(&mut dst, &src);
            let after = dump_any(&dst);
            if rc.is_err() {
                assert_eq!(after, before, "row {i}");
            }
            assert_eq!(line(rc, Some(after)), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilAsnAnyFree`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_asn_any_free() {
        let rows = [
            (AsnAny::Null, "any:Null"),
            (AsnAny::Integer32(-5), "any:Null"),
            (AsnAny::Unsigned32(1), "any:Null"),
            (AsnAny::Counter64(1), "any:Null"),
            (AsnAny::Counter32(1), "any:Null"),
            (AsnAny::Gauge32(1), "any:Null"),
            (AsnAny::Timeticks(1), "any:Null"),
            (AsnAny::OctetString(vec![10]), "any:Null"),
            (AsnAny::OctetString(Vec::new()), "any:Null"),
            (AsnAny::Bits(vec![255]), "any:Null"),
            (AsnAny::Sequence(vec![1, 2]), "any:Null"),
            (AsnAny::IpAddress(vec![1, 2, 3, 4]), "any:Null"),
            (AsnAny::Opaque(vec![9]), "any:Null"),
            (AsnAny::ObjectIdentifier(vec![1, 3, 6]), "any:Null"),
            (AsnAny::OctetString(vec![10]), "any:Null"),
        ];
        for (i, (mut any, want)) in rows.into_iter().enumerate() {
            asn_any_free(&mut any);
            assert_eq!(dump_any(&any), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilVarBindCpy`: 18 table vectors from the verbatim C harness.
    #[test]
    fn oracle_varbind_cpy() {
        let rows = [
            (None, Some(VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }), "rc=0"),
            (None, None, "rc=0"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), None, "rc=1 vb{name=oid[7.7] value=any:Null}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3, 6], value: AsnAny::Integer32(-5) }), "rc=1 vb{name=oid[1.3.6] value=any:Integer32=-5}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: Vec::new(), value: AsnAny::Null }), "rc=0"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1], value: AsnAny::Unsigned32(4294967295) }), "rc=1 vb{name=oid[1] value=any:Unsigned32=4294967295}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1], value: AsnAny::Counter64(1099511627776) }), "rc=1 vb{name=oid[1] value=any:Counter64=1099511627776}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1], value: AsnAny::Counter32(7) }), "rc=1 vb{name=oid[1] value=any:Counter32=7}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1], value: AsnAny::Gauge32(8) }), "rc=1 vb{name=oid[1] value=any:Gauge32=8}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1], value: AsnAny::Timeticks(9) }), "rc=1 vb{name=oid[1] value=any:Timeticks=9}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::OctetString(vec![10, 11]) }), "rc=1 vb{name=oid[1.3] value=any:OctetString=[0a0b]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::Bits(vec![255]) }), "rc=1 vb{name=oid[1.3] value=any:Bits=[ff]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::Sequence(Vec::new()) }), "rc=1 vb{name=oid[1.3] value=any:Sequence=[]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::IpAddress(vec![192, 168, 0, 1]) }), "rc=1 vb{name=oid[1.3] value=any:IpAddress=[c0a80001]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::Opaque(vec![0]) }), "rc=1 vb{name=oid[1.3] value=any:Opaque=[00]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 3], value: AsnAny::ObjectIdentifier(vec![1, 3, 6]) }), "rc=1 vb{name=oid[1.3] value=any:ObjectIdentifier=oid[1.3.6]}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: Vec::new(), value: AsnAny::Integer32(0) }), "rc=1 vb{name=oid[] value=any:Integer32=0}"),
            (Some(VarBind { name: vec![7, 7], value: AsnAny::Integer32(9) }), Some(VarBind { name: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], value: AsnAny::OctetString(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }), "rc=1 vb{name=oid[1.2.3.4.5.6.7.8.9.10.11.12] value=any:OctetString=[000102030405060708090a0b0c0d0e0f]}"),
        ];
        for (i, (dst, src, want)) in rows.into_iter().enumerate() {
            let mut dst = dst;
            let before = dst.as_ref().map(dump_vb);
            let rc = varbind_cpy(dst.as_mut(), src.as_ref());
            let after = dst.as_ref().map(dump_vb);
            if rc.is_err() {
                assert_eq!(after, before, "row {i}");
            }
            assert_eq!(line(rc, after), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilVarBindFree`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_varbind_free() {
        let rows = [
            (None, "none"),
            (
                Some(VarBind {
                    name: vec![1, 3],
                    value: AsnAny::Integer32(-5),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1, 3],
                    value: AsnAny::OctetString(vec![10, 11]),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: Vec::new(),
                    value: AsnAny::Null,
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![7],
                    value: AsnAny::Bits(vec![1]),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1, 3, 6],
                    value: AsnAny::Sequence(vec![2, 3]),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1],
                    value: AsnAny::IpAddress(vec![1, 2, 3, 4]),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1],
                    value: AsnAny::Opaque(Vec::new()),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1, 3, 6, 1],
                    value: AsnAny::ObjectIdentifier(vec![1, 3]),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: Vec::new(),
                    value: AsnAny::OctetString(Vec::new()),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1, 2],
                    value: AsnAny::Counter64(5),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1],
                    value: AsnAny::Unsigned32(6),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![1],
                    value: AsnAny::Timeticks(7),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![9, 9],
                    value: AsnAny::Counter32(8),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
            (
                Some(VarBind {
                    name: vec![5, 6],
                    value: AsnAny::Counter32(0),
                }),
                "vb{name=oid[] value=any:Null}",
            ),
        ];
        for (i, (pre, want)) in rows.into_iter().enumerate() {
            let mut vb = pre;
            varbind_free(vb.as_mut());
            let got = match vb {
                Some(vb) => dump_vb(&vb),
                None => "none".to_string(),
            };
            assert_eq!(got, want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilVarBindListCpy`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_varbind_list_cpy() {
        let rows = [
            (vec![VarBind { name: vec![7], value: AsnAny::Integer32(9) }], None, "rc=1 vbl[]"),
            (Vec::new(), None, "rc=1 vbl[]"),
            (Vec::new(), Some(Vec::new()), "rc=1 vbl[]"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }]), "rc=1 vbl[vb{name=oid[1.3] value=any:Integer32=5}]"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1], value: AsnAny::Null }]), "rc=0"),
            (vec![VarBind { name: vec![7], value: AsnAny::Integer32(9) }], Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }]), "rc=1 vbl[vb{name=oid[1.3] value=any:Integer32=5}, vb{name=oid[1.3.6] value=any:OctetString=[0a0b]}]"),
            (vec![VarBind { name: vec![7], value: AsnAny::Integer32(9) }], Some(vec![VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }, VarBind { name: Vec::new(), value: AsnAny::Opaque(vec![222]) }, VarBind { name: vec![7], value: AsnAny::ObjectIdentifier(vec![1, 3]) }]), "rc=1 vbl[vb{name=oid[1.3.6] value=any:OctetString=[0a0b]}, vb{name=oid[] value=any:Opaque=[de]}, vb{name=oid[7] value=any:ObjectIdentifier=oid[1.3]}]"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }, VarBind { name: Vec::new(), value: AsnAny::Opaque(vec![222]) }, VarBind { name: vec![7], value: AsnAny::ObjectIdentifier(vec![1, 3]) }, VarBind { name: vec![2, 2], value: AsnAny::Sequence(Vec::new()) }]), "rc=1 vbl[vb{name=oid[1.3] value=any:Integer32=5}, vb{name=oid[1.3.6] value=any:OctetString=[0a0b]}, vb{name=oid[] value=any:Opaque=[de]}, vb{name=oid[7] value=any:ObjectIdentifier=oid[1.3]}, vb{name=oid[2.2] value=any:Sequence=[]}]"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1], value: AsnAny::Null }, VarBind { name: Vec::new(), value: AsnAny::Opaque(vec![222]) }]), "rc=0"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }, VarBind { name: vec![1], value: AsnAny::Null }]), "rc=0"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1], value: AsnAny::Null }, VarBind { name: vec![1], value: AsnAny::Null }]), "rc=0"),
            (Vec::new(), Some(vec![VarBind { name: vec![2, 2], value: AsnAny::Sequence(Vec::new()) }, VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }]), "rc=1 vbl[vb{name=oid[2.2] value=any:Sequence=[]}, vb{name=oid[1.3] value=any:Integer32=5}]"),
            (Vec::new(), Some(vec![VarBind { name: Vec::new(), value: AsnAny::Integer32(0) }, VarBind { name: vec![1], value: AsnAny::Sequence(vec![1, 2]) }]), "rc=1 vbl[vb{name=oid[] value=any:Integer32=0}, vb{name=oid[1] value=any:Sequence=[0102]}]"),
            (vec![VarBind { name: vec![7], value: AsnAny::Integer32(9) }], Some(vec![VarBind { name: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], value: AsnAny::OctetString(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]) }]), "rc=1 vbl[vb{name=oid[1.2.3.4.5.6.7.8.9.10.11.12] value=any:OctetString=[000102030405060708090a0b0c0d0e0f]}]"),
            (Vec::new(), Some(vec![VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }, VarBind { name: Vec::new(), value: AsnAny::Opaque(vec![222]) }, VarBind { name: vec![7], value: AsnAny::ObjectIdentifier(vec![1, 3]) }, VarBind { name: vec![2, 2], value: AsnAny::Sequence(Vec::new()) }, VarBind { name: vec![1, 3], value: AsnAny::Integer32(5) }, VarBind { name: vec![1, 3, 6], value: AsnAny::OctetString(vec![10, 11]) }, VarBind { name: Vec::new(), value: AsnAny::Opaque(vec![222]) }]), "rc=1 vbl[vb{name=oid[1.3] value=any:Integer32=5}, vb{name=oid[1.3.6] value=any:OctetString=[0a0b]}, vb{name=oid[] value=any:Opaque=[de]}, vb{name=oid[7] value=any:ObjectIdentifier=oid[1.3]}, vb{name=oid[2.2] value=any:Sequence=[]}, vb{name=oid[1.3] value=any:Integer32=5}, vb{name=oid[1.3.6] value=any:OctetString=[0a0b]}, vb{name=oid[] value=any:Opaque=[de]}]"),
        ];
        for (i, (dst, src, want)) in rows.into_iter().enumerate() {
            let mut dst = dst;
            let before = dump_vbl(&dst);
            let rc = varbind_list_cpy(&mut dst, src.as_deref());
            let after = dump_vbl(&dst);
            if rc.is_err() {
                assert_eq!(after, before, "row {i}");
            }
            assert_eq!(line(rc, Some(after)), want, "row {i}");
        }
    }

    // Oracle rows of `SnmpUtilVarBindListFree`: 15 table vectors from the verbatim C harness.
    #[test]
    fn oracle_varbind_list_free() {
        let rows = [
            (Vec::new(), "vbl[]"),
            (
                vec![VarBind {
                    name: vec![1, 3],
                    value: AsnAny::Integer32(5),
                }],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                    VarBind {
                        name: vec![1, 3, 6],
                        value: AsnAny::OctetString(vec![10, 11]),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                    VarBind {
                        name: vec![1, 3, 6],
                        value: AsnAny::OctetString(vec![10, 11]),
                    },
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![VarBind {
                    name: vec![7],
                    value: AsnAny::ObjectIdentifier(vec![1, 3]),
                }],
                "vbl[]",
            ),
            (
                vec![VarBind {
                    name: vec![1],
                    value: AsnAny::Null,
                }],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                    VarBind {
                        name: vec![1],
                        value: AsnAny::Null,
                    },
                ],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![1, 3, 6],
                        value: AsnAny::OctetString(vec![10, 11]),
                    },
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                    VarBind {
                        name: vec![7],
                        value: AsnAny::ObjectIdentifier(vec![1, 3]),
                    },
                    VarBind {
                        name: vec![2, 2],
                        value: AsnAny::Sequence(Vec::new()),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![2, 2],
                        value: AsnAny::Sequence(Vec::new()),
                    },
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                    VarBind {
                        name: vec![1, 3, 6],
                        value: AsnAny::OctetString(vec![10, 11]),
                    },
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                    VarBind {
                        name: vec![7],
                        value: AsnAny::ObjectIdentifier(vec![1, 3]),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![VarBind {
                    name: Vec::new(),
                    value: AsnAny::Null,
                }],
                "vbl[]",
            ),
            (
                vec![VarBind {
                    name: vec![1],
                    value: AsnAny::OctetString(Vec::new()),
                }],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                    VarBind {
                        name: vec![1, 3],
                        value: AsnAny::Integer32(5),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                    VarBind {
                        name: Vec::new(),
                        value: AsnAny::Opaque(vec![222]),
                    },
                ],
                "vbl[]",
            ),
            (
                vec![VarBind {
                    name: vec![1, 3, 6],
                    value: AsnAny::OctetString(vec![10, 11]),
                }],
                "vbl[]",
            ),
            (
                vec![
                    VarBind {
                        name: vec![7],
                        value: AsnAny::ObjectIdentifier(vec![1, 3]),
                    },
                    VarBind {
                        name: vec![2, 2],
                        value: AsnAny::Sequence(Vec::new()),
                    },
                ],
                "vbl[]",
            ),
        ];
        for (i, (mut list, want)) in rows.into_iter().enumerate() {
            varbind_list_free(&mut list);
            assert_eq!(dump_vbl(&list), want, "row {i}");
        }
    }
    // END ORACLE ROWS
}
