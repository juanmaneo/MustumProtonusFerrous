/// How `dlls/msvcrt40/msvcrt40.spec` declares an export: the keyword after `@`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ cdecl`.
    Cdecl,
    /// `@ extern`: a data export, such as a vtable.
    Extern,
    /// `@ stdcall`.
    Stdcall,
    /// `@ stub`: raises `EXCEPTION_WINE_STUB` through [`crate::wine_spec_unimplemented_stub`].
    Stub,
    /// `@ thiscall`.
    Thiscall,
    /// `@ varargs`: a variadic function, `TYPE_VARARGS` in `tools/winebuild/build.h`.
    Varargs,
}

/// One line of the `msvcrt40.spec` export table: `@ <kind> <name>` for a stub, and
/// `@ <kind> <name>(<args>) <module>.<name>` for a forward.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub kind: ExportKind,
    /// The DLL the export forwards to, `msvcrt` or `msvcirt`. `None` for a `stub`.
    pub module: Option<&'static str>,
}

/// Where a forwarded export points: the export of the same name in `module`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forward {
    pub module: &'static str,
    pub function: &'static str,
}

/// The `msvcrt40.spec` export table in spec order. The spec gives no ordinals, so winebuild
/// numbers the entries 1 to 1164 in this order (`assign_ordinals` in `tools/winebuild/parser.c`).
pub static EXPORTS: [Export; 1164] = [
    Export {
        name: "??0Iostream_init@@QAE@AAVios@@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0Iostream_init@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0__non_rtti_object@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0__non_rtti_object@@QAE@PBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0bad_cast@@QAE@ABQBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0bad_cast@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0bad_typeid@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0bad_typeid@@QAE@PBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0exception@@QAE@ABQBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0exception@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0exception@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??0filebuf@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0filebuf@@QAE@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0filebuf@@QAE@HPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0filebuf@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0fstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0fstream@@QAE@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0fstream@@QAE@HPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0fstream@@QAE@PBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0fstream@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ifstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ifstream@@QAE@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ifstream@@QAE@HPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ifstream@@QAE@PBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ifstream@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ios@@IAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ios@@IAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ios@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0iostream@@IAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0iostream@@IAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0iostream@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream@@IAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream@@IAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream_withassign@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream_withassign@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istream_withassign@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istrstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istrstream@@QAE@PAD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0istrstream@@QAE@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0logic_error@@QAE@ABQBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0logic_error@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ofstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ofstream@@QAE@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ofstream@@QAE@HPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ofstream@@QAE@PBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ofstream@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream@@IAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream@@IAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream_withassign@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream_withassign@@QAE@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostream_withassign@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostrstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostrstream@@QAE@PADHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0ostrstream@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0stdiobuf@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0stdiobuf@@QAE@PAU_iobuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0stdiostream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0stdiostream@@QAE@PAU_iobuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0streambuf@@IAE@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0streambuf@@IAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0streambuf@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstream@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstream@@QAE@PADHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstream@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstreambuf@@QAE@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstreambuf@@QAE@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstreambuf@@QAE@P6APAXJ@ZP6AXPAX@Z@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstreambuf@@QAE@PADH0@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??0strstreambuf@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1Iostream_init@@QAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1__non_rtti_object@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??1bad_cast@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??1bad_typeid@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??1exception@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??1filebuf@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1fstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ifstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ios@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1iostream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1istream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1istream_withassign@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1istrstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1logic_error@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ofstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ostream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ostream_withassign@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1ostrstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1stdiobuf@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1stdiostream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1streambuf@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1strstream@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1strstreambuf@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??1type_info@@UAE@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??2@YAPAXI@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "??3@YAXPAX@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "??4Iostream_init@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4__non_rtti_object@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??4bad_cast@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??4bad_typeid@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??4exception@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??4filebuf@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4fstream@@QAEAAV0@AAV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ifstream@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ios@@IAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4iostream@@IAEAAV0@AAV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4iostream@@IAEAAV0@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istream@@IAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istream@@IAEAAV0@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istream_withassign@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istream_withassign@@QAEAAVistream@@ABV1@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istream_withassign@@QAEAAVistream@@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4istrstream@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4logic_error@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ofstream@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostream@@IAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostream@@IAEAAV0@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostream_withassign@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostream_withassign@@QAEAAVostream@@ABV1@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostream_withassign@@QAEAAVostream@@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4ostrstream@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4stdiobuf@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4stdiostream@@QAEAAV0@AAV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4streambuf@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4strstream@@QAEAAV0@AAV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??4strstreambuf@@QAEAAV0@ABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAC@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAE@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAF@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAG@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAJ@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAK@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAM@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAN@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@AAO@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@P6AAAV0@AAV0@@Z@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@P6AAAVios@@AAV1@@Z@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@PAC@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@PAD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@PAE@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??5istream@@QAEAAV0@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@C@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@D@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@E@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@F@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@G@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@I@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@J@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@K@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@M@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@N@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@O@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@P6AAAV0@AAV0@@Z@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@P6AAAVios@@AAV1@@Z@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@PAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@PBC@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@PBD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@PBE@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??6ostream@@QAEAAV0@PBX@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??7ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??8type_info@@QBEHABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??9type_info@@QBEHABV0@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??Bios@@QBEPAXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7__non_rtti_object@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_7bad_cast@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_7bad_typeid@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_7exception@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_7filebuf@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7fstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ifstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ios@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7iostream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7istream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7istream_withassign@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7istrstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7logic_error@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ofstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ostream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ostream_withassign@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7ostrstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7stdiobuf@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7stdiostream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7streambuf@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7strstream@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_7strstreambuf@@6B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8fstream@@7Bistream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8fstream@@7Bostream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8ifstream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8iostream@@7Bistream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8iostream@@7Bostream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8istream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8istream_withassign@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8istrstream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8ofstream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8ostream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8ostream_withassign@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8ostrstream@@7B@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8stdiostream@@7Bistream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8stdiostream@@7Bostream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8strstream@@7Bistream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_8strstream@@7Bostream@@@",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dfstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Difstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Diostream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Distream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Distream_withassign@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Distrstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dofstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dostream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dostream_withassign@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dostrstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dstdiostream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Dstrstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_EIostream_init@@QAEPAXI@Z",
        kind: ExportKind::Stub,
        module: None,
    },
    Export {
        name: "??_E__non_rtti_object@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Ebad_cast@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Ebad_typeid@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Eexception@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Efilebuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Efstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eifstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eios@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eiostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eistream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eistream_withassign@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eistrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Elogic_error@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eofstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eostream_withassign@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Eostrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Estdiobuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Estdiostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Estreambuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Estrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Estrstreambuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_GIostream_init@@QAEPAXI@Z",
        kind: ExportKind::Stub,
        module: None,
    },
    Export {
        name: "??_G__non_rtti_object@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Gbad_cast@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Gbad_typeid@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Gexception@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "??_Gfilebuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gfstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gifstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gios@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Giostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gistream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gistream_withassign@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gistrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Glogic_error@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gofstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gostream_withassign@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gostrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gstdiobuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gstdiostream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gstreambuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gstrstream@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "??_Gstrstreambuf@@UAEPAXI@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?_query_new_handler@@YAP6AHI@ZXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?_query_new_mode@@YAHXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?_set_new_handler@@YAP6AHI@ZP6AHI@Z@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?_set_new_mode@@YAHH@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?_set_se_translator@@YAP6AXIPAU_EXCEPTION_POINTERS@@@ZP6AXI0@Z@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?adjustfield@ios@@2JB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?allocate@streambuf@@IAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?attach@filebuf@@QAEPAV1@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?attach@fstream@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?attach@ifstream@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?attach@ofstream@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?bad@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?base@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?basefield@ios@@2JB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?before@type_info@@QBEHABV1@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "?binary@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?bitalloc@ios@@SAJXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?blen@streambuf@@IBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?cerr@@3Vostream_withassign@@A",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?cin@@3Vistream_withassign@@A",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?clear@ios@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?clog@@3Vostream_withassign@@A",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?close@filebuf@@QAEPAV1@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?close@fstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?close@ifstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?close@ofstream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?clrlock@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?clrlock@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?cout@@3Vostream_withassign@@A",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?dbp@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?dec@@YAAAVios@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?delbuf@ios@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?delbuf@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?doallocate@streambuf@@MAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?doallocate@strstreambuf@@MAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?eatwhite@istream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?eback@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?ebuf@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?egptr@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?endl@@YAAAVostream@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?ends@@YAAAVostream@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?eof@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?epptr@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fLockcInit@ios@@0HA",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fail@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fd@filebuf@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fd@fstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fd@ifstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fd@ofstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fill@ios@@QAEDD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?fill@ios@@QBEDXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?flags@ios@@QAEJJ@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?flags@ios@@QBEJXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?floatfield@ios@@2JB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?flush@@YAAAVostream@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?flush@ostream@@QAEAAV1@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?freeze@strstreambuf@@QAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?gbump@streambuf@@IAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?gcount@istream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@IAEAAV1@PADHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@AAC@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@AAD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@AAE@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@AAVstreambuf@@D@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@PACHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@PADHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEAAV1@PAEHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?get@istream@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?getdouble@istream@@AAEHPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?getint@istream@@AAEHPAD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?getline@istream@@QAEAAV1@PACHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?getline@istream@@QAEAAV1@PADHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?getline@istream@@QAEAAV1@PAEHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?good@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?gptr@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?hex@@YAAAVios@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?ignore@istream@@QAEAAV1@HH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?in_avail@streambuf@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?init@ios@@IAEXPAVstreambuf@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?ipfx@istream@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?is_open@filebuf@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?is_open@fstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?is_open@ifstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?is_open@ofstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?isfx@istream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?iword@ios@@QBEAAJH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lock@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lock@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lockbuf@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lockc@ios@@KAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lockptr@ios@@IAEPAU_CRT_CRITICAL_SECTION@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?lockptr@streambuf@@IAEPAU_CRT_CRITICAL_SECTION@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?name@type_info@@QBEPBDXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "?oct@@YAAAVios@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?open@filebuf@@QAEPAV1@PBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?open@fstream@@QAEXPBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?open@ifstream@@QAEXPBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?open@ofstream@@QAEXPBDHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?openprot@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?opfx@ostream@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?osfx@ostream@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?out_waiting@streambuf@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?overflow@filebuf@@UAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?overflow@stdiobuf@@UAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?overflow@strstreambuf@@UAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pbackfail@stdiobuf@@UAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pbackfail@streambuf@@UAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pbase@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pbump@streambuf@@IAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pcount@ostrstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pcount@strstream@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?peek@istream@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pptr@streambuf@@IBEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?precision@ios@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?precision@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?put@ostream@@QAEAAV1@C@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?put@ostream@@QAEAAV1@D@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?put@ostream@@QAEAAV1@E@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?putback@istream@@QAEAAV1@D@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?pword@ios@@QBEAAPAXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?raw_name@type_info@@QBEPBDXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "?rdbuf@fstream@@QBEPAVfilebuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@ifstream@@QBEPAVfilebuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@ios@@QBEPAVstreambuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@istrstream@@QBEPAVstrstreambuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@ofstream@@QBEPAVfilebuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@ostrstream@@QBEPAVstrstreambuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@stdiostream@@QBEPAVstdiobuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdbuf@strstream@@QBEPAVstrstreambuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?rdstate@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?read@istream@@QAEAAV1@PACH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?read@istream@@QAEAAV1@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?read@istream@@QAEAAV1@PAEH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sbumpc@streambuf@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekg@istream@@QAEAAV1@J@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekg@istream@@QAEAAV1@JW4seek_dir@ios@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekoff@filebuf@@UAEJJW4seek_dir@ios@@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekoff@stdiobuf@@UAEJJW4seek_dir@ios@@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekoff@streambuf@@UAEJJW4seek_dir@ios@@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekoff@strstreambuf@@UAEJJW4seek_dir@ios@@H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekp@ostream@@QAEAAV1@J@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekp@ostream@@QAEAAV1@JW4seek_dir@ios@@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?seekpos@streambuf@@UAEJJH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?set_new_handler@@YAP6AXXZP6AXXZ@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?set_terminate@@YAP6AXXZP6AXXZ@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?set_unexpected@@YAP6AXXZP6AXXZ@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?setb@streambuf@@IAEXPAD0H@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@filebuf@@UAEPAVstreambuf@@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@fstream@@QAEPAVstreambuf@@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@ifstream@@QAEPAVstreambuf@@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@ofstream@@QAEPAVstreambuf@@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@streambuf@@UAEPAV1@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setbuf@strstreambuf@@UAEPAVstreambuf@@PADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setf@ios@@QAEJJ@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setf@ios@@QAEJJJ@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setg@streambuf@@IAEXPAD00@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setlock@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setlock@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setmode@filebuf@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setmode@fstream@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setmode@ifstream@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setmode@ofstream@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setp@streambuf@@IAEXPAD0@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?setrwbuf@stdiobuf@@QAEHHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sgetc@streambuf@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sgetn@streambuf@@QAEHPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sh_none@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sh_read@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sh_write@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?snextc@streambuf@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sputbackc@streambuf@@QAEHD@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sputc@streambuf@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sputn@streambuf@@QAEHPBDH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?stdiofile@stdiobuf@@QAEPAU_iobuf@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?stossc@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?str@istrstream@@QAEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?str@ostrstream@@QAEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?str@strstream@@QAEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?str@strstreambuf@@QAEPADXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sunk_with_stdio@ios@@0HA",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync@filebuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync@istream@@QAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync@stdiobuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync@streambuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync@strstreambuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?sync_with_stdio@ios@@SAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?tellg@istream@@QAEJXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?tellp@ostream@@QAEJXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?terminate@@YAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?text@filebuf@@2HB",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?tie@ios@@QAEPAVostream@@PAV2@@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?tie@ios@@QBEPAVostream@@XZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unbuffered@streambuf@@IAEXH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unbuffered@streambuf@@IBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?underflow@filebuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?underflow@stdiobuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?underflow@strstreambuf@@UAEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unexpected@@YAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "?unlock@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unlock@streambuf@@QAEXXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unlockbuf@ios@@QAAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unlockc@ios@@KAXXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?unsetf@ios@@QAEJJ@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?what@exception@@UBEPBDXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcrt"),
    },
    Export {
        name: "?width@ios@@QAEHH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?width@ios@@QBEHXZ",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?write@ostream@@QAEAAV1@PBCH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?write@ostream@@QAEAAV1@PBDH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?write@ostream@@QAEAAV1@PBEH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?writepad@ostream@@AAEAAV1@PBD0@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?ws@@YAAAVistream@@AAV1@@Z",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?x_curindex@ios@@0HA",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?x_lockc@ios@@0U_CRT_CRITICAL_SECTION@@A",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?x_maxbit@ios@@0JA",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?x_statebuf@ios@@0PAJA",
        kind: ExportKind::Extern,
        module: Some("msvcirt"),
    },
    Export {
        name: "?xalloc@ios@@SAHXZ",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "?xsgetn@streambuf@@UAEHPADH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "?xsputn@streambuf@@UAEHPBDH@Z",
        kind: ExportKind::Thiscall,
        module: Some("msvcirt"),
    },
    Export {
        name: "$I10_OUTPUT",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIacos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIasin",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIatan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIatan2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIcos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIcosh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIexp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIfmod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIlog",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIlog10",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIpow",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIsin",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIsinh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CIsqrt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CItan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CItanh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_CxxThrowException",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_EH_prolog",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_HUGE",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_XcptFilter",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__CxxFrameHandler",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__CxxLongjmpUnwind",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "__RTCastToVoid",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__RTDynamicCast",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__RTtypeid",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__STRINGTOLD",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__argc",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "__argv",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "__dllonexit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__doserrno",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__fpecode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__getmainargs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__initenv",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "__isascii",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__iscsym",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__iscsymf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__lconv_init",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__mb_cur_max",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___argc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___argv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___initenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___mb_cur_max",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___wargv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p___winitenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__acmdln",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__amblksiz",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__commode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__daylight",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__dstbias",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__environ",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__fmode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__iob",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__mbctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__osver",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__pctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__pgmptr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__pwctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__timezone",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__tzname",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__wcmdln",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__wenviron",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__winmajor",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__winminor",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__winver",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__p__wpgmptr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__pxcptinfoptrs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__set_app_type",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__setusermatherr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__threadhandle",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__threadid",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__toascii",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__unDName",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__wargv",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "__wgetmainargs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "__winitenv",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_abnormal_termination",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_access",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_acmdln",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdiv_m16i",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdiv_m32",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdiv_m32i",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdiv_m64",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdiv_r",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdivr_m16i",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdivr_m32",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdivr_m32i",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fdivr_m64",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fpatan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fprem",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fprem1",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adj_fptan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_adjust_fdiv",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_aexit_rtn",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_amsg_exit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_assert",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_atodbl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_atoldbl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_beep",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_beginthread",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_beginthreadex",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_c_exit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cabs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cexit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cgets",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_chdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_chdrive",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_chgsign",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_chmod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_chsize",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_clearfp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_close",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_commit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_commode",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_control87",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_controlfp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_copysign",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cputs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_creat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ctype",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_cwait",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_daylight",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_dstbias",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_dup",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_dup2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ecvt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_endthread",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_endthreadex",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_environ",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_eof",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_errno",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_except_handler2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_except_handler3",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execl",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execle",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execlp",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execlpe",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execve",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execvp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_execvpe",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_exit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_expand",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fcloseall",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fcvt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fdopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fgetchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fgetwchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_filbuf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_filelength",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_filelengthi64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fileno",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_findclose",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_findfirst",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_findfirsti64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_findnext",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_findnexti64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_finite",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_flsbuf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_flushall",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fmode",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fpclass",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fpieee_flt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fpreset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fputchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fputwchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fsopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fstat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fstati64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ftime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ftol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_fullpath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_futime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_gcvt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_get_osfhandle",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getch",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getche",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getcwd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getdcwd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getdiskfree",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getdllprocaddr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getdrive",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getdrives",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getmaxstdio",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getmbcp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getpid",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getsystime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getw",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_getws",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_global_unwind2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapadd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapchk",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapmin",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapused",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_heapwalk",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_hypot",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_initterm",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_inp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_inpd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_inpw",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_iob",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_isatty",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_isctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbalnum",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbalpha",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbgraph",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbkalnum",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbkana",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbkprint",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbkpunct",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbblead",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbprint",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbpunct",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbbtrail",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcalnum",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcalpha",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcdigit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcgraph",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbchira",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbckata",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcl0",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcl1",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcl2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbclegal",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbclower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcprint",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcpunct",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcspace",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcsymbol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbcupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbslead",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ismbstrail",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_isnan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_itoa",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_itow",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_j0",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_j1",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_jn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_kbhit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lfind",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_loaddll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_local_unwind2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_locking",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_logb",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_longjmpex",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lrotl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lrotr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lsearch",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lseek",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_lseeki64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ltoa",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ltow",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_makepath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbbtombc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbbtype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbccpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbcjistojms",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbcjmstojis",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbclen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctohira",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctokata",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctolower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctombb",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctoupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbctype",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsbtype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbscat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbschr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbscmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbscoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbscpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbscspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsdec",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsdup",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsinc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbslen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbslwr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbcat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbcmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbcnt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbcoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbcpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnbset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsncat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnccnt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsncmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsncoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsncpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnextc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsninc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsnset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbspbrk",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsrchr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsrev",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsspnp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsstr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbstok",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbstrlen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mbsupr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_memccpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_memicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mkdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mktemp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_msize",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_mtlock",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "_mtunlock",
        kind: ExportKind::Cdecl,
        module: Some("msvcirt"),
    },
    Export {
        name: "_nextafter",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_onexit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_open",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_open_osfhandle",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_osver",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_outp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_outpd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_outpw",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_pclose",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_pctype",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_pgmptr",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_pipe",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_popen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_purecall",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_putch",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_putenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_putw",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_putws",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_pwctype",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_read",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_rmdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_rmtmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_rotl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_rotr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_safe_fdiv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_safe_fdivr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_safe_fprem",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_safe_fprem1",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_scalb",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_searchenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_seh_longjmp_unwind",
        kind: ExportKind::Stdcall,
        module: Some("msvcrt"),
    },
    Export {
        name: "_set_error_mode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_seterrormode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setjmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setjmp3",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setmaxstdio",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setmbcp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setmode",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_setsystime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_sleep",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_snprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_snwprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_sopen",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnl",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnle",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnlp",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnlpe",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnve",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnvp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_spawnvpe",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_splitpath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_stat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_stati64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_statusfp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strcmpi",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strdate",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strdup",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strerror",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_stricmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_stricoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strlwr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strncoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strnicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strnicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strnset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strrev",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strtime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_strupr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_swab",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_sys_errlist",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_sys_nerr",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_tell",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_telli64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_tempnam",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_timezone",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_tolower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_toupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_tzname",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_tzset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ultoa",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ultow",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_umask",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_ungetch",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_unlink",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_unloaddll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_utime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_vsnprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_vsnwprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_waccess",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wasctime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wchdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wchmod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcmdln",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcreat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsdup",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcslwr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsncoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsnicmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsnicoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsnset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsrev",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wcsupr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wctime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wenviron",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecl",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecle",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexeclp",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexeclpe",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecve",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecvp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wexecvpe",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfdopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfindfirst",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfindfirsti64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfindnext",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfindnexti64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfreopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfsopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wfullpath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wgetcwd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wgetdcwd",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wgetenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_winmajor",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_winminor",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_winver",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wmakepath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wmkdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wmktemp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wopen",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wperror",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wpgmptr",
        kind: ExportKind::Extern,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wpopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wputenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wremove",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wrename",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_write",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wrmdir",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wsearchenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wsetlocale",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wsopen",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnl",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnle",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnlp",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnlpe",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnve",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnvp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wspawnvpe",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wsplitpath",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wstat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wstati64",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wstrdate",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wstrtime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wsystem",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wtempnam",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wtmpnam",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wtoi",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wtol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wunlink",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_wutime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_y0",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_y1",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "_yn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "abort",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "abs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "acos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "asctime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "asin",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atan2",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atexit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atof",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atoi",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "atol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "bsearch",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "calloc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ceil",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "clearerr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "clock",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "cos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "cosh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ctime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "difftime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "div",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "exit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "exp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fabs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fclose",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "feof",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ferror",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fflush",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fgetc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fgetpos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fgets",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fgetwc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fgetws",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "floor",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fmod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "fputc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fputs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fputwc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fputws",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fread",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "free",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "freopen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "frexp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "fseek",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fsetpos",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ftell",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fwprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "fwrite",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "fwscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "getc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "getchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "getenv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "gets",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "getwc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "getwchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "gmtime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "is_wctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isalnum",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isalpha",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iscntrl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isdigit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isgraph",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isleadbyte",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "islower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isprint",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ispunct",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isspace",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswalnum",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswalpha",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswascii",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswcntrl",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswctype",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswdigit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswgraph",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswlower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswprint",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswpunct",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswspace",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "iswxdigit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "isxdigit",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "labs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ldexp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ldiv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "localeconv",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "localtime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "log",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "log10",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "longjmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "malloc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "mblen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "mbstowcs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "mbtowc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "memchr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "memcmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "memcpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "memmove",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "memset",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "mktime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "modf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "perror",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "pow",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "printf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "putc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "putchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "puts",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "putwc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "putwchar",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "qsort",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "raise",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "rand",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "realloc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "remove",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "rename",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "rewind",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "scanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "setbuf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "setlocale",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "setvbuf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "signal",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "sin",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "sinh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "sprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "sqrt",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "srand",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "sscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "strcat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strchr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strcmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strcoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strcpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strcspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strerror",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strftime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strlen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strncat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strncmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strncpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strpbrk",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strrchr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strstr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strtod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strtok",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strtol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strtoul",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "strxfrm",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "swprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "swscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "system",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "tan",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "tanh",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "time",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "tmpfile",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "tmpnam",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "tolower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "toupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "towlower",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "towupper",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ungetc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "ungetwc",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vfprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vfwprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vsprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vswprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "vwprintf",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcscat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcschr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcscmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcscoll",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcscpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcscspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsftime",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcslen",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsncat",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsncmp",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsncpy",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcspbrk",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsrchr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsspn",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsstr",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcstod",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcstok",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcstol",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcstombs",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcstoul",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wcsxfrm",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wctomb",
        kind: ExportKind::Cdecl,
        module: Some("msvcrt"),
    },
    Export {
        name: "wprintf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
    Export {
        name: "wscanf",
        kind: ExportKind::Varargs,
        module: Some("msvcrt"),
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// Where the export named `name` forwards, or `None` when it is a `stub` or the spec does not
/// export it.
pub fn forward(name: &str) -> Option<Forward> {
    let export = find_export(name)?;
    export.module.map(|module| Forward {
        module,
        function: export.name,
    })
}
