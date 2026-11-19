use crate::c_str;

const BACKSLASH: u16 = b'\\' as u16;
const SLASH: u16 = b'/' as u16;
const QUESTION: u16 = b'?' as u16;

/// The path `install_from_file` builds before canonicalizing it. A leading `\??\` becomes `\\?\`.
/// A separator goes between `dir` and `subdir` unless `dir` is empty or already ends in one.
pub fn build_install_path(dir: &[u16], subdir: &[u16], file_name: &[u16]) -> Vec<u16> {
    let mut path = c_str(dir).to_vec();
    if path.starts_with(&[BACKSLASH, QUESTION, QUESTION, BACKSLASH]) {
        path[1] = BACKSLASH;
    }
    if let Some(&last) = path.last() {
        if last != SLASH && last != BACKSLASH {
            path.push(BACKSLASH);
        }
    }
    path.extend_from_slice(c_str(subdir));
    path.push(BACKSLASH);
    path.extend_from_slice(c_str(file_name));
    path
}

/// The directories `install_from_default_dir` tries before the download, in order. `builddir` is
/// `WINEBUILDDIR` and `datadir` is `WINEDATADIR`. A set `WINEBUILDDIR` replaces `WINEDATADIR`.
/// `install_datadir` is `INSTALL_DATADIR`, and `/usr/share` is only listed once.
pub fn default_dir_candidates(
    builddir: Option<&[u16]>,
    datadir: Option<&[u16]>,
    install_datadir: &str,
) -> Vec<Vec<u16>> {
    let mut dirs: Vec<Vec<u16>> = Vec::new();
    if let Some(builddir) = builddir {
        let mut dir = c_str(builddir).to_vec();
        dir.extend("\\..\\".encode_utf16());
        dirs.push(dir);
    } else if let Some(datadir) = datadir {
        dirs.push(c_str(datadir).to_vec());
    }
    dirs.push(
        format!("\\\\?\\unix{install_datadir}/wine/")
            .encode_utf16()
            .collect(),
    );
    if install_datadir != "/usr/share" {
        dirs.push("\\\\?\\unix/usr/share/wine/".encode_utf16().collect());
    }
    dirs.push("\\\\?\\unix/opt/wine/".encode_utf16().collect());
    dirs
}

/// `get_cache_file_name` with `ensure_exists` false. `xdg` is `WINE_HOST_XDG_CACHE_HOME`, used when
/// set and not empty. Otherwise `home` (`WINEHOMEDIR`, where empty still counts as set) gives
/// `home\.cache`, with its second character forced to `\`. Returns `None` when neither applies.
pub fn cache_file_path(
    xdg: Option<&[u16]>,
    home: Option<&[u16]>,
    file_name: &[u16],
) -> Option<Vec<u16>> {
    let xdg = xdg.map(c_str).filter(|xdg| !xdg.is_empty());
    let mut cache_dir = if let Some(xdg) = xdg {
        let mut dir: Vec<u16> = "\\\\?\\unix".encode_utf16().collect();
        dir.extend_from_slice(xdg);
        dir
    } else {
        let mut dir = c_str(home?).to_vec();
        dir.extend("\\.cache".encode_utf16());
        dir[1] = BACKSLASH;
        dir
    };
    cache_dir.extend("\\wine\\".encode_utf16());
    cache_dir.extend_from_slice(c_str(file_name));
    Some(cache_dir)
}
