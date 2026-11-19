/// `enum install_res` in `addons.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstallRes {
    Ok = 0,
    Failed = 1,
    Next = 2,
}

const ERROR_SUCCESS: u32 = 0;
const ERROR_PRODUCT_VERSION: u32 = 1638;

/// `install_file`. `first` is the `MsiInstallProductW` result. ERROR_PRODUCT_VERSION makes the C
/// code call again with the reinstall properties, which `retry` stands for, and only that second
/// result is checked.
pub fn install_file_result(first: u32, retry: impl FnOnce() -> u32) -> InstallRes {
    let res = if first == ERROR_PRODUCT_VERSION {
        retry()
    } else {
        first
    };
    if res == ERROR_SUCCESS {
        InstallRes::Ok
    } else {
        InstallRes::Failed
    }
}

/// The end of `install_from_file`. `final_path` is the path from `GetFinalPathNameByHandleW`.
/// A path of 6 WCHARs or fewer gives `None`, which is `INSTALL_NEXT`. Otherwise a `\\?\` prefix in
/// front of a drive letter is removed, and the result is what gets installed.
pub fn install_target(final_path: &[u16]) -> Option<&[u16]> {
    if final_path.len() <= 6 {
        return None;
    }
    Some(if final_path[5] == u16::from(b':') {
        &final_path[4..]
    } else {
        final_path
    })
}

/// The search of `install_from_default_dir`. `install` runs on each directory in order, and the
/// first result other than `Next` ends the search. When every directory gives `Next`, so does the
/// search.
pub fn search_dirs<F>(dirs: &[Vec<u16>], mut install: F) -> InstallRes
where
    F: FnMut(&[u16]) -> InstallRes,
{
    for dir in dirs {
        let res = install(dir.as_slice());
        if res != InstallRes::Next {
            return res;
        }
    }
    InstallRes::Next
}

/// The source chain of `install_addon`. The sources run in order and the chain stops at the first
/// result other than `Next`. `None` means every source gave `Next`, so the URL download runs.
pub fn first_settled<I, F>(sources: I) -> Option<InstallRes>
where
    I: IntoIterator<Item = F>,
    F: FnOnce() -> InstallRes,
{
    sources
        .into_iter()
        .map(|source| source())
        .find(|res| *res != InstallRes::Next)
}
