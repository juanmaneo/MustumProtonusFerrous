use super::*;

#[test]
fn service_mapping_ignores_unrecognized_bits() {
    assert_eq!(map_service_flags(0x8000_0000), 0);
    assert_eq!(
        map_service_flags(u32::MAX),
        XP_CONNECTIONLESS
            | XP_GUARANTEED_DELIVERY
            | XP_GUARANTEED_ORDER
            | XP_MESSAGE_ORIENTED
            | XP_PSEUDO_STREAM
            | XP_GRACEFUL_CLOSE
            | XP_EXPEDITED_DATA
            | XP_CONNECT_DATA
            | XP_DISCONNECT_DATA
            | XP_SUPPORTS_BROADCAST
            | XP_SUPPORTS_MULTICAST
            | XP_BANDWIDTH_ALLOCATION
            | XP_FRAGMENTATION
    );
}

#[test]
fn option_mapping_preserves_unmapped_values_and_levels() {
    for optname in [i32::MIN, -1, 0, 1, 10, i32::MAX] {
        assert_eq!(remap_socket_option(IPPROTO_IP, optname), optname);
    }
    for level in [i32::MIN, -1, 1, 6, i32::MAX] {
        assert_eq!(remap_socket_option(level, 2), 2);
    }
}
