//! Port of `debugstr_ToastTemplateType` in Wine's `dlls/wpnapps/toast_notification.c`.

const TEMPLATE_NAMES: [&str; 8] = [
    "ToastImageAndText01",
    "ToastImageAndText02",
    "ToastImageAndText03",
    "ToastImageAndText04",
    "ToastText01",
    "ToastText02",
    "ToastText03",
    "ToastText04",
];

/// `debugstr_ToastTemplateType`. The C check is `type < ARRAY_SIZE(str)` on an unsigned size, so a negative value
/// is out of range and prints as its raw `%#x` bit pattern.
pub fn debugstr_toast_template_type(ty: i32) -> String {
    usize::try_from(ty)
        .ok()
        .and_then(|index| TEMPLATE_NAMES.get(index))
        .map_or_else(
            || format!("{{unknown {:#x}}}", ty as u32),
            |name| format!("ToastTemplateType_{name}"),
        )
}
