//! `getItemFromOid` and `getItemAndIntegerInstanceFromOid`.
//!
//! A `None` result is `SNMP_ERRORSTATUS_NOSUCHNAME`.

use crate::oid::oid_n_cmp;

pub const SNMP_PDU_GET: u8 = 0xa0;
pub const SNMP_PDU_GETNEXT: u8 = 0xa1;
pub const SNMP_PDU_SET: u8 = 0xa3;

/// `getItemFromOid`: the item named by `oid` under `base`.
///
/// For GETNEXT, a missing item gives item 1 and an item `n` gives `n + 1`, wrapping at `u32::MAX`.
pub fn get_item_from_oid(oid: &[u32], base: &[u32], pdu: u8) -> Option<u32> {
    let cmp = oid_n_cmp(oid, base, base.len());
    if pdu == SNMP_PDU_GETNEXT {
        if cmp < 0 {
            Some(1)
        } else if cmp == 0 {
            Some(if oid.len() == base.len() {
                1
            } else {
                oid[base.len()].wrapping_add(1)
            })
        } else {
            None
        }
    } else if cmp == 0 && oid.len() != base.len() {
        match oid[base.len()] {
            0 => None,
            item => Some(item),
        }
    } else {
        None
    }
}

/// `getItemAndIntegerInstanceFromOid`: the item and integer instance named by `oid` under `base`.
///
/// For GETNEXT, an OID that stops at the table or the item resolves to instance 1, and a
/// full OID resolves to the instance after it, wrapping at `u32::MAX`.
pub fn get_item_and_integer_instance_from_oid(
    oid: &[u32],
    base: &[u32],
    pdu: u8,
) -> Option<(u32, u32)> {
    let n = base.len();
    let cmp = oid_n_cmp(oid, base, n);
    if pdu == SNMP_PDU_GETNEXT {
        if cmp < 0 {
            Some((1, 1))
        } else if cmp == 0 {
            if oid.len() == n || oid.len() == n + 1 {
                let item = if oid.len() == n + 1 { oid[n] } else { 1 };
                Some((item, 1))
            } else {
                Some((oid[n], oid[n + 1].wrapping_add(1)))
            }
        } else {
            None
        }
    } else if cmp == 0 && oid.len() != n && oid.len() != n + 1 {
        Some((oid[n], oid[n + 1]))
    } else {
        None
    }
}
