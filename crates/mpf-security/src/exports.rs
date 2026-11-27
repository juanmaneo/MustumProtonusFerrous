/// One `@ stdcall` line of `dlls/security/security.spec`. Every line forwards to `secur32`, so the
/// entry has no code of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    /// The exported name, spelled as in the spec file. Names are case-sensitive.
    pub name: &'static str,
    /// The ordinal `winebuild` assigns. No line names one, so ordinals run from 1 in spec order.
    pub ordinal: u16,
    /// The number of arguments in the spec's argument list.
    pub nb_args: u8,
    /// The forward string `winebuild` writes for the export, `secur32.<name>`.
    pub forward: &'static str,
}

/// The DLL file name, from `MODULE` in `dlls/security/Makefile.in`.
pub const MODULE: &str = "security.dll";

/// The DLL every export forwards to, from `IMPORTS` in `dlls/security/Makefile.in`.
pub const IMPORTS: &str = "secur32";

/// The `security.spec` export table, in spec order.
pub const EXPORTS: [Export; 36] = [
    Export {
        name: "AcceptSecurityContext",
        ordinal: 1,
        nb_args: 9,
        forward: "secur32.AcceptSecurityContext",
    },
    Export {
        name: "AcquireCredentialsHandleA",
        ordinal: 2,
        nb_args: 9,
        forward: "secur32.AcquireCredentialsHandleA",
    },
    Export {
        name: "AcquireCredentialsHandleW",
        ordinal: 3,
        nb_args: 9,
        forward: "secur32.AcquireCredentialsHandleW",
    },
    Export {
        name: "AddSecurityPackageA",
        ordinal: 4,
        nb_args: 2,
        forward: "secur32.AddSecurityPackageA",
    },
    Export {
        name: "AddSecurityPackageW",
        ordinal: 5,
        nb_args: 2,
        forward: "secur32.AddSecurityPackageW",
    },
    Export {
        name: "ApplyControlToken",
        ordinal: 6,
        nb_args: 2,
        forward: "secur32.ApplyControlToken",
    },
    Export {
        name: "CompleteAuthToken",
        ordinal: 7,
        nb_args: 2,
        forward: "secur32.CompleteAuthToken",
    },
    Export {
        name: "DecryptMessage",
        ordinal: 8,
        nb_args: 4,
        forward: "secur32.DecryptMessage",
    },
    Export {
        name: "DeleteSecurityContext",
        ordinal: 9,
        nb_args: 1,
        forward: "secur32.DeleteSecurityContext",
    },
    Export {
        name: "DeleteSecurityPackageA",
        ordinal: 10,
        nb_args: 1,
        forward: "secur32.DeleteSecurityPackageA",
    },
    Export {
        name: "DeleteSecurityPackageW",
        ordinal: 11,
        nb_args: 1,
        forward: "secur32.DeleteSecurityPackageW",
    },
    Export {
        name: "EncryptMessage",
        ordinal: 12,
        nb_args: 4,
        forward: "secur32.EncryptMessage",
    },
    Export {
        name: "EnumerateSecurityPackagesA",
        ordinal: 13,
        nb_args: 2,
        forward: "secur32.EnumerateSecurityPackagesA",
    },
    Export {
        name: "EnumerateSecurityPackagesW",
        ordinal: 14,
        nb_args: 2,
        forward: "secur32.EnumerateSecurityPackagesW",
    },
    Export {
        name: "ExportSecurityContext",
        ordinal: 15,
        nb_args: 4,
        forward: "secur32.ExportSecurityContext",
    },
    Export {
        name: "FreeContextBuffer",
        ordinal: 16,
        nb_args: 1,
        forward: "secur32.FreeContextBuffer",
    },
    Export {
        name: "FreeCredentialsHandle",
        ordinal: 17,
        nb_args: 1,
        forward: "secur32.FreeCredentialsHandle",
    },
    Export {
        name: "ImpersonateSecurityContext",
        ordinal: 18,
        nb_args: 1,
        forward: "secur32.ImpersonateSecurityContext",
    },
    Export {
        name: "ImportSecurityContextA",
        ordinal: 19,
        nb_args: 4,
        forward: "secur32.ImportSecurityContextA",
    },
    Export {
        name: "ImportSecurityContextW",
        ordinal: 20,
        nb_args: 4,
        forward: "secur32.ImportSecurityContextW",
    },
    Export {
        name: "InitSecurityInterfaceA",
        ordinal: 21,
        nb_args: 0,
        forward: "secur32.InitSecurityInterfaceA",
    },
    Export {
        name: "InitSecurityInterfaceW",
        ordinal: 22,
        nb_args: 0,
        forward: "secur32.InitSecurityInterfaceW",
    },
    Export {
        name: "InitializeSecurityContextA",
        ordinal: 23,
        nb_args: 12,
        forward: "secur32.InitializeSecurityContextA",
    },
    Export {
        name: "InitializeSecurityContextW",
        ordinal: 24,
        nb_args: 12,
        forward: "secur32.InitializeSecurityContextW",
    },
    Export {
        name: "MakeSignature",
        ordinal: 25,
        nb_args: 4,
        forward: "secur32.MakeSignature",
    },
    Export {
        name: "QueryContextAttributesA",
        ordinal: 26,
        nb_args: 3,
        forward: "secur32.QueryContextAttributesA",
    },
    Export {
        name: "QueryContextAttributesW",
        ordinal: 27,
        nb_args: 3,
        forward: "secur32.QueryContextAttributesW",
    },
    Export {
        name: "QueryCredentialsAttributesA",
        ordinal: 28,
        nb_args: 3,
        forward: "secur32.QueryCredentialsAttributesA",
    },
    Export {
        name: "QueryCredentialsAttributesW",
        ordinal: 29,
        nb_args: 3,
        forward: "secur32.QueryCredentialsAttributesW",
    },
    Export {
        name: "QuerySecurityContextToken",
        ordinal: 30,
        nb_args: 2,
        forward: "secur32.QuerySecurityContextToken",
    },
    Export {
        name: "QuerySecurityPackageInfoA",
        ordinal: 31,
        nb_args: 2,
        forward: "secur32.QuerySecurityPackageInfoA",
    },
    Export {
        name: "QuerySecurityPackageInfoW",
        ordinal: 32,
        nb_args: 2,
        forward: "secur32.QuerySecurityPackageInfoW",
    },
    Export {
        name: "RevertSecurityContext",
        ordinal: 33,
        nb_args: 1,
        forward: "secur32.RevertSecurityContext",
    },
    Export {
        name: "SealMessage",
        ordinal: 34,
        nb_args: 4,
        forward: "secur32.SealMessage",
    },
    Export {
        name: "UnsealMessage",
        ordinal: 35,
        nb_args: 4,
        forward: "secur32.UnsealMessage",
    },
    Export {
        name: "VerifySignature",
        ordinal: 36,
        nb_args: 4,
        forward: "secur32.VerifySignature",
    },
];

/// The export called `name`, or `None` when the spec does not export it.
pub fn export_by_name(name: &str) -> Option<&'static Export> {
    EXPORTS.iter().find(|export| export.name == name)
}

/// The export with `ordinal`, or `None` outside `1..=36`. Ordinal 0 is never assigned.
pub fn export_by_ordinal(ordinal: u16) -> Option<&'static Export> {
    let index = usize::from(ordinal.checked_sub(1)?);
    EXPORTS.get(index)
}

/// Splits a forward string at its first `.`, the test `winebuild` uses to mark a name as a
/// forward. `None` means the string is not a forward.
pub fn split_forward(forward: &str) -> Option<(&str, &str)> {
    forward.split_once('.')
}
