/// One enumerator of a typelib enum: its IDL name and its value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Member {
    pub name: &'static str,
    pub value: u32,
}

/// Declares the constants of one `typedef enum` from `include/iads.idl`, then the member table
/// `$table` that lists them in IDL order. Enumerators without `=` take the value C gives them.
macro_rules! typelib_enum {
    ($table:ident { $($member:ident = $value:expr,)+ }) => {
        $(pub const $member: u32 = $value;)+

        pub const $table: &[Member] = &[$(Member { name: stringify!($member), value: $member }),+];
    };
}

typelib_enum!(ADS_RIGHTS_ENUM {
    ADS_RIGHT_DS_CREATE_CHILD = 0x00000001,
    ADS_RIGHT_DS_DELETE_CHILD = 0x00000002,
    ADS_RIGHT_ACTRL_DS_LIST = 0x00000004,
    ADS_RIGHT_DS_SELF = 0x00000008,
    ADS_RIGHT_DS_READ_PROP = 0x00000010,
    ADS_RIGHT_DS_WRITE_PROP = 0x00000020,
    ADS_RIGHT_DS_DELETE_TREE = 0x00000040,
    ADS_RIGHT_DS_LIST_OBJECT = 0x00000080,
    ADS_RIGHT_DS_CONTROL_ACCESS = 0x00000100,
    ADS_RIGHT_DELETE = 0x00010000,
    ADS_RIGHT_READ_CONTROL = 0x00020000,
    ADS_RIGHT_WRITE_DAC = 0x00040000,
    ADS_RIGHT_WRITE_OWNER = 0x00080000,
    ADS_RIGHT_SYNCHRONIZE = 0x00100000,
    ADS_RIGHT_ACCESS_SYSTEM_SECURITY = 0x00200000,
    ADS_RIGHT_GENERIC_ALL = 0x10000000,
    ADS_RIGHT_GENERIC_EXECUTE = 0x20000000,
    ADS_RIGHT_GENERIC_WRITE = 0x40000000,
    ADS_RIGHT_GENERIC_READ = 0x80000000,
});

typelib_enum!(ADS_AUTHENTICATION_ENUM {
    ADS_SECURE_AUTHENTICATION = 0x1,
    ADS_USE_ENCRYPTION = 0x2,
    ADS_USE_SSL = 0x2,
    ADS_READONLY_SERVER = 0x4,
    ADS_PROMPT_CREDENTIALS = 0x8,
    ADS_NO_AUTHENTICATION = 0x10,
    ADS_FAST_BIND = 0x20,
    ADS_USE_SIGNING = 0x40,
    ADS_USE_SEALING = 0x80,
    ADS_USE_DELEGATION = 0x100,
    ADS_SERVER_BIND = 0x200,
    ADS_NO_REFERRAL_CHASING = 0x400,
    ADS_AUTH_RESERVED = 0x80000000,
});

typelib_enum!(ADS_SEARCHPREF_ENUM {
    ADS_SEARCHPREF_ASYNCHRONOUS = 0,
    ADS_SEARCHPREF_DEREF_ALIASES = 1,
    ADS_SEARCHPREF_SIZE_LIMIT = 2,
    ADS_SEARCHPREF_TIME_LIMIT = 3,
    ADS_SEARCHPREF_ATTRIBTYPES_ONLY = 4,
    ADS_SEARCHPREF_SEARCH_SCOPE = 5,
    ADS_SEARCHPREF_TIMEOUT = 6,
    ADS_SEARCHPREF_PAGESIZE = 7,
    ADS_SEARCHPREF_PAGED_TIME_LIMIT = 8,
    ADS_SEARCHPREF_CHASE_REFERRALS = 9,
    ADS_SEARCHPREF_SORT_ON = 10,
    ADS_SEARCHPREF_CACHE_RESULTS = 11,
    ADS_SEARCHPREF_DIRSYNC = 12,
    ADS_SEARCHPREF_TOMBSTONE = 13,
    ADS_SEARCHPREF_VLV = 14,
    ADS_SEARCHPREF_ATTRIBUTE_QUERY = 15,
    ADS_SEARCHPREF_SECURITY_MASK = 16,
    ADS_SEARCHPREF_DIRSYNC_FLAG = 17,
    ADS_SEARCHPREF_EXTENDED_DN = 18,
});

typelib_enum!(ADS_CHASE_REFERRALS_ENUM {
    ADS_CHASE_REFERRALS_NEVER = 0,
    ADS_CHASE_REFERRALS_SUBORDINATE = 0x20,
    ADS_CHASE_REFERRALS_EXTERNAL = 0x40,
    ADS_CHASE_REFERRALS_ALWAYS = ADS_CHASE_REFERRALS_SUBORDINATE | ADS_CHASE_REFERRALS_EXTERNAL,
});

typelib_enum!(ADS_STATUSENUM {
    ADS_STATUS_S_OK = 0,
    ADS_STATUS_INVALID_SEARCHPREF = 1,
    ADS_STATUS_INVALID_SEARCHPREFVALUE = 2,
});

typelib_enum!(ADSTYPEENUM {
    ADSTYPE_INVALID = 0,
    ADSTYPE_DN_STRING = 1,
    ADSTYPE_CASE_EXACT_STRING = 2,
    ADSTYPE_CASE_IGNORE_STRING = 3,
    ADSTYPE_PRINTABLE_STRING = 4,
    ADSTYPE_NUMERIC_STRING = 5,
    ADSTYPE_BOOLEAN = 6,
    ADSTYPE_INTEGER = 7,
    ADSTYPE_OCTET_STRING = 8,
    ADSTYPE_UTC_TIME = 9,
    ADSTYPE_LARGE_INTEGER = 10,
    ADSTYPE_PROV_SPECIFIC = 11,
    ADSTYPE_OBJECT_CLASS = 12,
    ADSTYPE_CASEIGNORE_LIST = 13,
    ADSTYPE_OCTET_LIST = 14,
    ADSTYPE_PATH = 15,
    ADSTYPE_POSTALADDRESS = 16,
    ADSTYPE_TIMESTAMP = 17,
    ADSTYPE_BACKLINK = 18,
    ADSTYPE_TYPEDNAME = 19,
    ADSTYPE_HOLD = 20,
    ADSTYPE_NETADDRESS = 21,
    ADSTYPE_REPLICAPOINTER = 22,
    ADSTYPE_FAXNUMBER = 23,
    ADSTYPE_EMAIL = 24,
    ADSTYPE_NT_SECURITY_DESCRIPTOR = 25,
    ADSTYPE_UNKNOWN = 26,
    ADSTYPE_DN_WITH_BINARY = 27,
    ADSTYPE_DN_WITH_STRING = 28,
});

typelib_enum!(ADS_SCOPEENUM {
    ADS_SCOPE_BASE = 0,
    ADS_SCOPE_ONELEVEL = 1,
    ADS_SCOPE_SUBTREE = 2,
});

typelib_enum!(ADS_PASSWORD_ENCODING_ENUM {
    ADS_PASSWORD_ENCODE_REQUIRE_SSL = 0,
    ADS_PASSWORD_ENCODE_CLEAR = 1,
});

typelib_enum!(ADS_OPTION_ENUM {
    ADS_OPTION_SERVERNAME = 0,
    ADS_OPTION_REFERRALS = 1,
    ADS_OPTION_PAGE_SIZE = 2,
    ADS_OPTION_SECURITY_MASK = 3,
    ADS_OPTION_MUTUAL_AUTH_STATUS = 4,
    ADS_OPTION_QUOTA = 5,
    ADS_OPTION_PASSWORD_PORTNUMBER = 6,
    ADS_OPTION_PASSWORD_METHOD = 7,
    ADS_OPTION_ACCUMULATIVE_MODIFICATION = 8,
    ADS_OPTION_SKIP_SID_LOOKUP = 9,
});

typelib_enum!(ADS_SECURITY_INFO_ENUM {
    ADS_SECURITY_INFO_OWNER = 0x1,
    ADS_SECURITY_INFO_GROUP = 0x2,
    ADS_SECURITY_INFO_DACL = 0x4,
    ADS_SECURITY_INFO_SACL = 0x8,
});

typelib_enum!(ADS_SETTYPE_ENUM {
    ADS_SETTYPE_FULL = 1,
    ADS_SETTYPE_PROVIDER = 2,
    ADS_SETTYPE_SERVER = 3,
    ADS_SETTYPE_DN = 4,
});

typelib_enum!(ADS_FORMAT_ENUM {
    ADS_FORMAT_WINDOWS = 1,
    ADS_FORMAT_WINDOWS_NO_SERVER = 2,
    ADS_FORMAT_WINDOWS_DN = 3,
    ADS_FORMAT_WINDOWS_PARENT = 4,
    ADS_FORMAT_X500 = 5,
    ADS_FORMAT_X500_NO_SERVER = 6,
    ADS_FORMAT_X500_DN = 7,
    ADS_FORMAT_X500_PARENT = 8,
    ADS_FORMAT_SERVER = 9,
    ADS_FORMAT_PROVIDER = 10,
    ADS_FORMAT_LEAF = 11,
});

/// Returns the value of the member called `name` in `members`, or `None` if there is none.
pub fn member_value(members: &[Member], name: &str) -> Option<u32> {
    members
        .iter()
        .find(|member| member.name == name)
        .map(|member| member.value)
}

/// Returns the name of the first member of `members` with the value `value`, or `None` if no
/// member has that value. Where two members share a value, as `ADS_USE_ENCRYPTION` and
/// `ADS_USE_SSL` do, the one declared first is returned.
pub fn member_name(members: &[Member], value: u32) -> Option<&'static str> {
    members
        .iter()
        .find(|member| member.value == value)
        .map(|member| member.name)
}
