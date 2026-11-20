use super::*;

/// Rows emitted by `target/canon/wsock32/oracle.c`, built from the Wine
/// mapping bodies with `gcc -fsanitize=address,undefined -g` and run clean.
#[test]
fn matches_wine_oracle_rows() {
    let service_rows = [
        ("service_none", 0, 0),
        (
            "service_connectionless",
            XP1_CONNECTIONLESS,
            XP_CONNECTIONLESS,
        ),
        (
            "service_delivery",
            XP1_GUARANTEED_DELIVERY,
            XP_GUARANTEED_DELIVERY,
        ),
        ("service_order", XP1_GUARANTEED_ORDER, XP_GUARANTEED_ORDER),
        ("service_message", XP1_MESSAGE_ORIENTED, XP_MESSAGE_ORIENTED),
        ("service_pseudo_stream", XP1_PSEUDO_STREAM, XP_PSEUDO_STREAM),
        (
            "service_graceful_close",
            XP1_GRACEFUL_CLOSE,
            XP_GRACEFUL_CLOSE,
        ),
        ("service_expedited", XP1_EXPEDITED_DATA, XP_EXPEDITED_DATA),
        ("service_connect_data", XP1_CONNECT_DATA, XP_CONNECT_DATA),
        (
            "service_disconnect_data",
            XP1_DISCONNECT_DATA,
            XP_DISCONNECT_DATA,
        ),
        (
            "service_broadcast",
            XP1_SUPPORT_BROADCAST,
            XP_SUPPORTS_BROADCAST,
        ),
        (
            "service_multipoint",
            XP1_SUPPORT_MULTIPOINT,
            XP_SUPPORTS_MULTICAST,
        ),
        ("service_qos", XP1_QOS_SUPPORTED, XP_BANDWIDTH_ALLOCATION),
        ("service_partial", XP1_PARTIAL_MESSAGE, XP_FRAGMENTATION),
        (
            "service_all",
            XP1_CONNECTIONLESS
                | XP1_GUARANTEED_DELIVERY
                | XP1_GUARANTEED_ORDER
                | XP1_MESSAGE_ORIENTED
                | XP1_PSEUDO_STREAM
                | XP1_GRACEFUL_CLOSE
                | XP1_EXPEDITED_DATA
                | XP1_CONNECT_DATA
                | XP1_DISCONNECT_DATA
                | XP1_SUPPORT_BROADCAST
                | XP1_SUPPORT_MULTIPOINT
                | XP1_QOS_SUPPORTED
                | XP1_PARTIAL_MESSAGE,
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
                | XP_FRAGMENTATION,
        ),
    ];
    for (name, input, expected) in service_rows {
        assert_eq!(map_service_flags(input), expected, "{name}");
    }

    let option_rows = [
        ("option_multicast_if", IPPROTO_IP, 2, 9),
        ("option_multicast_ttl", IPPROTO_IP, 3, 10),
        ("option_multicast_loop", IPPROTO_IP, 4, 11),
        ("option_add_membership", IPPROTO_IP, 5, 12),
        ("option_drop_membership", IPPROTO_IP, 6, 13),
        ("option_ttl", IPPROTO_IP, 7, 4),
        ("option_tos", IPPROTO_IP, 8, 3),
        ("option_dont_fragment", IPPROTO_IP, 9, 14),
        ("option_unknown_ip", IPPROTO_IP, 99, 99),
        ("option_other_level", 6, 2, 2),
    ];
    for (name, level, input, expected) in option_rows {
        assert_eq!(remap_socket_option(level, input), expected, "{name}");
    }
}
