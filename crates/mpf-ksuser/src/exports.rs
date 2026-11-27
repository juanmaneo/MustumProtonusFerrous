use crate::stub::{
    ks_create_allocator, ks_create_clock, ks_create_pin, ks_create_topology_node, StubException,
};

/// The helper of a `@ stub` export, which returns the record its `__wine_spec_unimplemented_stub`
/// raises.
type StubHelper = fn() -> StubException;

/// The `dlls/ksuser/ksuser.spec` export table, in spec order. Every entry is `@ stub`, so each
/// name is paired with its helper.
pub const EXPORTS: [(&str, StubHelper); 4] = [
    ("KsCreateAllocator", ks_create_allocator),
    ("KsCreateClock", ks_create_clock),
    ("KsCreatePin", ks_create_pin),
    ("KsCreateTopologyNode", ks_create_topology_node),
];

/// The record that the export `name` raises when called, or `None` when `ksuser.spec` does not
/// export it. Names are case-sensitive.
pub fn stub_for_export(name: &str) -> Option<StubException> {
    EXPORTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|&(_, stub)| stub())
}
