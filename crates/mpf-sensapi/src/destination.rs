/// `IsDestinationReachableA` from `dlls/sensapi/sensapi.c`. The C body ignores the destination
/// string and the `QOCINFO` pointer and returns `TRUE`, which is `true` here.
pub fn is_destination_reachable_a() -> bool {
    true
}

/// `IsDestinationReachableW` from `dlls/sensapi/sensapi.c`. The C body ignores the destination
/// string and the `QOCINFO` pointer and returns `TRUE`, which is `true` here.
pub fn is_destination_reachable_w() -> bool {
    true
}
