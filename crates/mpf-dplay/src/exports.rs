/// The pointer size of the i386 build, from `ENABLE_ARCHS = i386` in `dlls/dplay/Makefile.in`.
const I386_POINTER_SIZE: u32 = 4;

/// One `dplay.spec` entry. Both entries are forwards to `dplayx.dll`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The name on the spec line.
    pub name: &'static str,
    /// The ordinal `winebuild` assigns. No entry names one, so ordinals follow spec order from 1.
    pub ordinal: u16,
    /// The number of `ptr` arguments on the spec line.
    pub nb_args: u32,
    /// The forwarder string from the spec line.
    pub forward: &'static str,
}

impl Export {
    /// The stdcall stack size that `winebuild` writes into the decorated name `name@N` on i386.
    pub fn stack_bytes(&self) -> u32 {
        self.nb_args * I386_POINTER_SIZE
    }
}

/// The `dplay.spec` export table, in spec order.
pub const EXPORTS: [Export; 2] = [
    Export {
        name: "DirectPlayCreate",
        ordinal: 1,
        nb_args: 3,
        forward: "dplayx.DirectPlayCreate",
    },
    Export {
        name: "DirectPlayEnumerate",
        ordinal: 2,
        nb_args: 2,
        forward: "dplayx.DirectPlayEnumerate",
    },
];

/// The export named `name`, or `None` when `dplay.spec` does not export it. Names are case-sensitive.
pub fn export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|e| e.name == name).copied()
}

/// Whether `winebuild` accepts every link name in `--data-only` mode. A data-only module may hold
/// only forwards, and `winebuild` treats a link name as a forward when it contains a `.`.
pub fn data_only_accepts(link_names: &[&str]) -> bool {
    link_names.iter().all(|name| name.contains('.'))
}
