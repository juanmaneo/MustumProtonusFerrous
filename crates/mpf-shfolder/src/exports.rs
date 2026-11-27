/// The argument types `dlls/shfolder/shfolder.spec` declares. Only `long` and `ptr` appear there.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgType {
    /// `long`. winebuild sizes it as a pointer, see [`crate::get_args_size`].
    Long,
    /// `ptr`.
    Ptr,
}

/// One `@ stdcall` line of `dlls/shfolder/shfolder.spec`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub ordinal: u32,
    pub args: &'static [ArgType],
    /// The `module.function` target after the argument list. Both lines forward to `shell32`.
    pub forward: &'static str,
}

const SHGET_FOLDER_PATH_ARGS: &[ArgType] = &[
    ArgType::Long,
    ArgType::Long,
    ArgType::Long,
    ArgType::Long,
    ArgType::Ptr,
];

/// The export table of `dlls/shfolder/shfolder.spec`, in spec order.
pub static EXPORTS: [Export; 2] = [
    Export {
        name: "SHGetFolderPathA",
        ordinal: 1,
        args: SHGET_FOLDER_PATH_ARGS,
        forward: "shell32.SHGetFolderPathA",
    },
    Export {
        name: "SHGetFolderPathW",
        ordinal: 2,
        args: SHGET_FOLDER_PATH_ARGS,
        forward: "shell32.SHGetFolderPathW",
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn export_by_name(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}
