/// The suffix `GetWin32sDirectory` appends to the system directory, from the
/// `static const char win32s[]` in `dlls/w32skernel.c`.
pub const WIN32S_SUFFIX: &str = "\\win32s";

/// `GetWin32sDirectory` from `dlls/w32skernel.c`. The C body concatenates the system
/// directory with `\\win32s`; the port takes the system directory as an argument and
/// returns the concatenated path.
pub fn get_win32s_directory(system_dir: &str) -> String {
    let mut text = String::with_capacity(system_dir.len() + WIN32S_SUFFIX.len());
    text.push_str(system_dir);
    text.push_str(WIN32S_SUFFIX);
    text
}
