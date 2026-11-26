//! Print environment lookup from `validate_envW` in `provider.c`.

use crate::eq_ascii_ignore_case;

/// The `printenv_t` entries of `provider.c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrintEnv {
    Ia64,
    X86,
    X64,
    Arm,
    Arm64,
    Win40,
}

/// `all_printenv`, in the C order.
const ALL_PRINTENV: [PrintEnv; 6] = [
    PrintEnv::X86,
    PrintEnv::X64,
    PrintEnv::Ia64,
    PrintEnv::Arm,
    PrintEnv::Arm64,
    PrintEnv::Win40,
];

impl PrintEnv {
    /// The `envname` of the entry.
    pub fn name(self) -> &'static str {
        match self {
            PrintEnv::Ia64 => "Windows IA64",
            PrintEnv::X86 => "Windows NT x86",
            PrintEnv::X64 => "Windows x64",
            PrintEnv::Arm => "Windows ARM",
            PrintEnv::Arm64 => "Windows ARM64",
            PrintEnv::Win40 => "Windows 4.0",
        }
    }
}

/// `validate_envW`. A `None` or empty `env` selects the default environment: `Win40`
/// when `win9x` (the `GetVersion` high bit) is set, otherwise `arch` (the build-time `env_arch`).
/// Returns `None` where C sets `ERROR_INVALID_ENVIRONMENT`.
pub fn validate_env(env: Option<&[u16]>, win9x: bool, arch: PrintEnv) -> Option<PrintEnv> {
    match env {
        Some(env) if !env.is_empty() => ALL_PRINTENV
            .iter()
            .copied()
            .find(|p| eq_ascii_ignore_case(env, p.name())),
        _ if win9x => Some(PrintEnv::Win40),
        _ => Some(arch),
    }
}
