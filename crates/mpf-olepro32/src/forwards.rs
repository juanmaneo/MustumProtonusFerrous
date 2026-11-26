/// One forwarded export from `dlls/olepro32/olepro32.spec`: a `stdcall` name whose body lives in
/// another DLL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forward {
    /// The ordinal in the spec line.
    pub ordinal: u16,
    pub name: &'static str,
    /// Argument bytes that `stdcall` pops on i386: 4 per `long` or `ptr` parameter.
    pub stack_bytes: u32,
    /// The DLL before the dot in the spec line's forward target.
    pub target_dll: &'static str,
    /// The export name after the dot in the spec line's forward target.
    pub target_name: &'static str,
}

/// The seven forwarded exports, in spec order.
pub const FORWARDS: [Forward; 7] = [
    Forward {
        ordinal: 248,
        name: "OleIconToCursor",
        stack_bytes: 8,
        target_dll: "oleaut32",
        target_name: "OleIconToCursor",
    },
    Forward {
        ordinal: 249,
        name: "OleCreatePropertyFrameIndirect",
        stack_bytes: 4,
        target_dll: "oleaut32",
        target_name: "OleCreatePropertyFrameIndirect",
    },
    Forward {
        ordinal: 250,
        name: "OleCreatePropertyFrame",
        stack_bytes: 44,
        target_dll: "oleaut32",
        target_name: "OleCreatePropertyFrame",
    },
    Forward {
        ordinal: 251,
        name: "OleLoadPicture",
        stack_bytes: 20,
        target_dll: "oleaut32",
        target_name: "OleLoadPicture",
    },
    Forward {
        ordinal: 252,
        name: "OleCreatePictureIndirect",
        stack_bytes: 16,
        target_dll: "oleaut32",
        target_name: "OleCreatePictureIndirect",
    },
    Forward {
        ordinal: 253,
        name: "OleCreateFontIndirect",
        stack_bytes: 12,
        target_dll: "oleaut32",
        target_name: "OleCreateFontIndirect",
    },
    Forward {
        ordinal: 254,
        name: "OleTranslateColor",
        stack_bytes: 12,
        target_dll: "oleaut32",
        target_name: "OleTranslateColor",
    },
];

/// The forward for `name`, or `None` when the spec does not forward it. Names are case-sensitive.
pub fn forward_for(name: &str) -> Option<Forward> {
    FORWARDS.iter().find(|f| f.name == name).copied()
}
