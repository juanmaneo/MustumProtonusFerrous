//! `SnmpUtilOidNCmp`, `oidToIpAddr` and the `setOidWithItem*` builders.

/// `SnmpUtilOidNCmp`: compares the first `count` arcs of `a` and `b`.
///
/// The common prefix (at most `count` arcs) is compared first. If all of it matches and it
/// covers `count` arcs, the result is 0. Otherwise the shorter OID sorts first.
pub fn oid_n_cmp(a: &[u32], b: &[u32], count: usize) -> i32 {
    let len = count.min(a.len()).min(b.len());
    for (x, y) in a.iter().zip(b).take(len) {
        if x != y {
            return x.cmp(y) as i32;
        }
    }
    if len == count {
        0
    } else {
        a.len().cmp(&b.len()) as i32
    }
}

/// `oidToIpAddr`: the first four arcs as IPv4 octets in memory order.
///
/// Each arc is truncated to a byte, like the `(BYTE)` casts in the C code.
pub fn oid_to_ip_addr(ids: &[u32]) -> [u8; 4] {
    assert!(ids.len() >= 4, "oidToIpAddr needs at least 4 arcs");
    [ids[0] as u8, ids[1] as u8, ids[2] as u8, ids[3] as u8]
}

/// `setOidWithItem`: `base` followed by `item`.
pub fn set_oid_with_item(base: &[u32], item: u32) -> Vec<u32> {
    let mut dst = base.to_vec();
    dst.push(item);
    dst
}

/// `setOidWithItemAndIpAddr`: `base`, `item`, then the four octets of `addr` in order.
pub fn set_oid_with_item_and_ip_addr(base: &[u32], item: u32, addr: [u8; 4]) -> Vec<u32> {
    let mut dst = set_oid_with_item(base, item);
    dst.extend(addr.iter().map(|&octet| u32::from(octet)));
    dst
}

/// `setOidWithItemAndInteger`: `base`, `item`, then `instance`.
pub fn set_oid_with_item_and_integer(base: &[u32], item: u32, instance: u32) -> Vec<u32> {
    let mut dst = set_oid_with_item(base, item);
    dst.push(instance);
    dst
}
