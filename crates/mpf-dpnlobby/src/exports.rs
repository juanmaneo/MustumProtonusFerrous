/// The module name `winebuild` gives the DLL, from `MODULE = dpnlobby.dll` in
/// `dlls/dpnlobby/Makefile.in`. It is the `LIBRARY` line of the `.def` file and the name in the
/// PE export directory.
pub const DLL_NAME: &str = "dpnlobby.dll";

/// The i386 pointer size. `winebuild` counts it for each `ptr` argument of a stdcall `@N` name.
const I386_POINTER_BYTES: u32 = 4;

/// One entry of the `dlls/dpnlobby/dpnlobby.spec` export table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name, from the `@ stdcall` line.
    pub name: &'static str,
    /// The ordinal `winebuild` assigns to the `@` entry. It is the lowest free ordinal, starting at 1.
    pub ordinal: u16,
    /// The number of `ptr` arguments in the argument list.
    pub pointer_args: u32,
    /// The text after the argument list. A `.` in it makes the entry a forward.
    pub link_name: &'static str,
}

/// The `dpnlobby.spec` export table, in spec order.
pub const EXPORTS: [Export; 1] = [Export {
    name: "DirectPlay8LobbyCreate",
    ordinal: 1,
    pointer_args: 3,
    link_name: "dpnet.DirectPlay8Create",
}];

/// The export named `name`, or `None` when the spec does not export it. Names are matched exactly.
pub fn export_by_name(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}

impl Export {
    /// Whether `winebuild` treats the entry as a forward, which is when its link name contains a
    /// `.` (`tools/winebuild/parser.c`). `--data-only` builds accept only forwards.
    pub fn is_forward(&self) -> bool {
        self.link_name.contains('.')
    }

    /// The `.def` export line for the entry: `name[@N]=link @ordinal`. `@N` is the argument
    /// bytes, which `winebuild` writes for an i386 stdcall entry unless `kill_at` is set.
    pub fn def_line(&self, i386: bool, kill_at: bool) -> String {
        let decorated = if i386 && !kill_at {
            format!("{}@{}", self.name, self.pointer_args * I386_POINTER_BYTES)
        } else {
            self.name.to_string()
        };
        format!("{}={} @{}", decorated, self.link_name, self.ordinal)
    }
}
