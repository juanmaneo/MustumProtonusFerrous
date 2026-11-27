use crate::stdcall::{args_size, ArgType, ArgType::*, Cpu};

/// The module every `rsabase.spec` line forwards to: `rsaenh.<name>`.
const FORWARD_MODULE: &str = "rsaenh";

/// One `@ stdcall` line of `dlls/rsabase/rsabase.spec`, with the ordinal and flags `winebuild` gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    /// The spec has no explicit ordinals, so `winebuild` numbers the lines from 1 in spec order.
    pub ordinal: u16,
    pub args: &'static [ArgType],
    /// `-private` sets `FLAG_PRIVATE`, "function is private (cannot be imported)" in `tools/winebuild/build.h`.
    pub private: bool,
}

/// The `rsabase.spec` export table, in spec order.
pub static EXPORTS: [Export; 27] = [
    Export {
        name: "CPAcquireContext",
        ordinal: 1,
        args: &[Ptr, Str, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPCreateHash",
        ordinal: 2,
        args: &[Long, Long, Ptr, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPDecrypt",
        ordinal: 3,
        args: &[Long, Long, Long, Long, Long, Ptr, Ptr],
        private: false,
    },
    Export {
        name: "CPDeriveKey",
        ordinal: 4,
        args: &[Long, Long, Long, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPDestroyHash",
        ordinal: 5,
        args: &[Long, Long],
        private: false,
    },
    Export {
        name: "CPDestroyKey",
        ordinal: 6,
        args: &[Long, Long],
        private: false,
    },
    Export {
        name: "CPDuplicateHash",
        ordinal: 7,
        args: &[Long, Long, Ptr, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPDuplicateKey",
        ordinal: 8,
        args: &[Long, Long, Ptr, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPEncrypt",
        ordinal: 9,
        args: &[Long, Long, Long, Long, Long, Ptr, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPExportKey",
        ordinal: 10,
        args: &[Long, Long, Long, Long, Long, Ptr, Ptr],
        private: false,
    },
    Export {
        name: "CPGenKey",
        ordinal: 11,
        args: &[Long, Long, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPGenRandom",
        ordinal: 12,
        args: &[Long, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPGetHashParam",
        ordinal: 13,
        args: &[Long, Long, Long, Ptr, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPGetKeyParam",
        ordinal: 14,
        args: &[Long, Long, Long, Ptr, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPGetProvParam",
        ordinal: 15,
        args: &[Long, Long, Ptr, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPGetUserKey",
        ordinal: 16,
        args: &[Long, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPHashData",
        ordinal: 17,
        args: &[Long, Long, Ptr, Long, Long],
        private: false,
    },
    Export {
        name: "CPHashSessionKey",
        ordinal: 18,
        args: &[Long, Long, Long, Long],
        private: false,
    },
    Export {
        name: "CPImportKey",
        ordinal: 19,
        args: &[Long, Ptr, Long, Long, Long, Ptr],
        private: false,
    },
    Export {
        name: "CPReleaseContext",
        ordinal: 20,
        args: &[Long, Long],
        private: false,
    },
    Export {
        name: "CPSetHashParam",
        ordinal: 21,
        args: &[Long, Long, Long, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPSetKeyParam",
        ordinal: 22,
        args: &[Long, Long, Long, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPSetProvParam",
        ordinal: 23,
        args: &[Long, Long, Ptr, Long],
        private: false,
    },
    Export {
        name: "CPSignHash",
        ordinal: 24,
        args: &[Long, Long, Long, WStr, Long, Ptr, Ptr],
        private: false,
    },
    Export {
        name: "CPVerifySignature",
        ordinal: 25,
        args: &[Long, Long, Ptr, Long, Long, WStr, Long],
        private: false,
    },
    Export {
        name: "DllRegisterServer",
        ordinal: 26,
        args: &[],
        private: true,
    },
    Export {
        name: "DllUnregisterServer",
        ordinal: 27,
        args: &[],
        private: true,
    },
];

/// The export named `name`, or `None` when the spec has no such line. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}

/// The exports an importer can bind to: every line except the `-private` ones.
pub fn importable() -> impl Iterator<Item = &'static Export> {
    EXPORTS.iter().filter(|export| !export.private)
}

/// The `EXPORTS` line `winebuild --def` writes for `export`, from `output_def_file` in `tools/winebuild/spec32.c`.
/// The `@N` argument-size suffix is only written for i386 targets.
pub fn def_line(export: &Export, cpu: Cpu) -> String {
    let decoration = if cpu == Cpu::I386 {
        format!("@{}", args_size(export.args, cpu))
    } else {
        String::new()
    };
    let private = if export.private { " PRIVATE" } else { "" };
    format!(
        "  {}{}={}.{} @{}{}",
        export.name, decoration, FORWARD_MODULE, export.name, export.ordinal, private
    )
}
