/// `NDdeIsValidShareNameA`, exported as `stdcall`. The C body only logs its argument and returns
/// `FALSE` (`BOOL` 0), so every name is rejected.
pub fn ndde_is_valid_share_name_a() -> bool {
    false
}

/// `NDdeIsValidShareNameW`, exported as `stdcall`. The C body only logs its argument and returns
/// `FALSE`.
pub fn ndde_is_valid_share_name_w() -> bool {
    false
}

/// `NDdeIsValidAppTopicListA`, exported as `stdcall`. The C body only logs its argument and
/// returns `FALSE`.
pub fn ndde_is_valid_app_topic_list_a() -> bool {
    false
}

/// `NDdeIsValidAppTopicListW`, exported as `stdcall`. The C body only logs its argument and
/// returns `FALSE`.
pub fn ndde_is_valid_app_topic_list_w() -> bool {
    false
}
