/// Splits a forwarded export target at its last `.`, the way `check_undefined_forwards` in
/// `tools/winebuild/import.c` does with `strrchr`. Returns the module part and the function part.
///
/// `winebuild` only handles targets that contain a dot (`FLAG_FORWARD`), where the C code writes
/// through the `strrchr` result. The port returns `None` for any other target.
pub fn split_forward(target: &str) -> Option<(&str, &str)> {
    let dot = target.rfind('.')?;
    Some((&target[..dot], &target[dot + 1..]))
}

/// `get_dll_name` from `tools/winebuild/import.c`: the file name of a forwarded module, with
/// `.dll` appended when the name has no dot.
pub fn get_dll_name(name: &str) -> String {
    if name.contains('.') {
        name.to_owned()
    } else {
        format!("{name}.dll")
    }
}
