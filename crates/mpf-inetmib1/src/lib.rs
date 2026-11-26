//! Port of the portable logic in Wine's `dlls/inetmib1/main.c`.
//!
//! Ported:
//! - `SnmpUtilOidNCmp`, `oidToIpAddr` and the `setOidWithItem*` builders: [`oid_n_cmp`],
//!   [`oid_to_ip_addr`], [`set_oid_with_item`], [`set_oid_with_item_and_ip_addr`],
//!   [`set_oid_with_item_and_integer`]
//! - `getItemFromOid` and `getItemAndIntegerInstanceFromOid`: [`get_item_from_oid`],
//!   [`get_item_and_integer_instance_from_oid`]
//! - `findValueInTable`, `findOidInTable`, `findNextOidInTable` and
//!   `getItemAndInstanceFromTable`: [`find_value_in_table`], [`find_oid_in_table`],
//!   [`find_next_oid_in_table`], [`get_item_and_instance_from_table`]
//! - `DWORD_cmp`, `compareIpAddrRow` (and `compareIpForwardRow`, whose body is the same),
//!   `oidToUdpRow` and `compareUdpRow`: [`dword_cmp`], [`compare_ip_addr_row`],
//!   [`oid_to_udp_row`], [`compare_udp_row`]
//! - `copyOperStatus`: [`oper_status_to_admin_status`]
//! - `findSupportedQuery` and the longest-match loop of `SnmpExtensionQuery`:
//!   [`find_supported_query`], [`find_longest_supported_query`]
//! - The MIB-2 prefix filter of `SnmpExtensionQuery`: [`in_mib2`]
//!
//! Skipped:
//! - `SnmpExtensionInit`, the `SnmpExtensionQuery` dispatch and the `mib2*Init`, `mib2*Query`
//!   and `mib2*Cleanup` handlers. They call IP Helper and other OS APIs.
//! - `mapStructEntryToValue` and the `copyInt`, `copyIpAddr`, `setStringValue` and `copyIf*`
//!   helpers. They fill `AsnAny` values from OS structures.
//! - Allocation-failure paths (`SnmpUtilOid*`, `HeapAlloc`), `DllMain`, and `TRACE` and
//!   `FIXME` output.

mod item;
mod oid;
mod row;
mod status;
mod supported;
mod table;

pub use item::{
    get_item_and_integer_instance_from_oid, get_item_from_oid, SNMP_PDU_GET, SNMP_PDU_GETNEXT,
    SNMP_PDU_SET,
};
pub use oid::{
    oid_n_cmp, oid_to_ip_addr, set_oid_with_item, set_oid_with_item_and_integer,
    set_oid_with_item_and_ip_addr,
};
pub use row::{compare_ip_addr_row, compare_udp_row, dword_cmp, oid_to_udp_row, UdpRow};
pub use status::{
    oper_status_to_admin_status, MIB_IF_ADMIN_STATUS_DOWN, MIB_IF_ADMIN_STATUS_TESTING,
    MIB_IF_ADMIN_STATUS_UP, MIB_IF_OPER_STATUS_CONNECTED, MIB_IF_OPER_STATUS_CONNECTING,
    MIB_IF_OPER_STATUS_DISCONNECTED, MIB_IF_OPER_STATUS_NON_OPERATIONAL,
    MIB_IF_OPER_STATUS_OPERATIONAL, MIB_IF_OPER_STATUS_UNREACHABLE,
};
pub use supported::{find_longest_supported_query, find_supported_query, SUPPORTED_IDS};
pub use table::{
    find_next_oid_in_table, find_oid_in_table, find_value_in_table,
    get_item_and_instance_from_table,
};

/// `DEFINE_OID(mib2)` from `main.c`: the MIB-2 prefix 1.3.6.1.2.1.
pub const MIB2: &[u32] = &[1, 3, 6, 1, 2, 1];

/// The first check in `SnmpExtensionQuery`: OIDs outside MIB-2 are ignored.
pub fn in_mib2(oid: &[u32]) -> bool {
    oid_n_cmp(oid, MIB2, MIB2.len()) == 0
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
