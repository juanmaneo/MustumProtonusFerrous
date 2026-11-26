use crate::args::{stdcall_arg_bytes, ArgType, Cpu};

/// One forwarded export from `dlls/dpnaddr/dpnaddr.spec`: a `stdcall` name whose body lives in
/// another DLL.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forward {
    /// The ordinal that winebuild assigns, because the spec line gives none.
    pub ordinal: u16,
    pub name: &'static str,
    /// The parameter types from the spec line, in order.
    pub args: &'static [ArgType],
    /// The DLL before the dot in the spec line's forward target.
    pub target_dll: &'static str,
    /// The export name after the dot in the spec line's forward target.
    pub target_name: &'static str,
}

impl Forward {
    /// The argument bytes that `stdcall` pops for this forward on `cpu`.
    pub fn stack_bytes(&self, cpu: Cpu) -> u32 {
        stdcall_arg_bytes(self.args, cpu)
    }
}

/// The one forwarded export, in spec order.
pub const FORWARDS: [Forward; 1] = [Forward {
    ordinal: 1,
    name: "DirectPlay8AddressCreate",
    args: &[ArgType::Ptr, ArgType::Ptr, ArgType::Ptr],
    target_dll: "dpnet",
    target_name: "DirectPlay8Create",
}];

/// The forward for `name`, or `None` when the spec does not forward it. Names are case-sensitive.
pub fn forward_for(name: &str) -> Option<Forward> {
    FORWARDS.iter().find(|f| f.name == name).copied()
}
