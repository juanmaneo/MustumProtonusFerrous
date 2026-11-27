/// How `dlls/icuin/icuin.spec` declares an export.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    /// `@ cdecl`: a `__cdecl` function.
    Cdecl,
    /// `@ varargs`: a variadic function, `TYPE_VARARGS` in `tools/winebuild/build.h`.
    Varargs,
}

/// One line of the `icuin.spec` export table: `@ <kind> <name>(<args>) icu.<name>`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Export {
    pub name: &'static str,
    pub kind: ExportKind,
    /// The argument types as the spec writes them, for example `str ptr long ptr`.
    pub args: &'static str,
}

/// Where an export forwards: a function of the same name in `FORWARD_MODULE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Forward {
    pub module: &'static str,
    pub function: &'static str,
}

/// The module every export in `icuin.spec` forwards to, from the `icu.` prefix of each line.
pub const FORWARD_MODULE: &str = "icu";

/// The module name from `MODULE = icuin.dll` in `dlls/icuin/Makefile.in`.
pub const ICUIN_DLL_NAME: &str = "icuin.dll";

/// The `icuin.spec` export table, in spec order. Every entry forwards to `icu.<name>`.
pub static EXPORTS: [Export; 416] = [
    Export {
        name: "u_formatMessage",
        kind: ExportKind::Varargs,
        args: "str ptr long ptr long ptr",
    },
    Export {
        name: "u_formatMessageWithError",
        kind: ExportKind::Varargs,
        args: "str ptr long ptr long ptr ptr",
    },
    Export {
        name: "u_parseMessage",
        kind: ExportKind::Varargs,
        args: "str ptr long ptr long ptr",
    },
    Export {
        name: "u_parseMessageWithError",
        kind: ExportKind::Varargs,
        args: "str ptr long ptr long ptr ptr",
    },
    Export {
        name: "u_vformatMessage",
        kind: ExportKind::Cdecl,
        args: "str ptr long ptr long long ptr",
    },
    Export {
        name: "u_vformatMessageWithError",
        kind: ExportKind::Cdecl,
        args: "str ptr long ptr long ptr long ptr",
    },
    Export {
        name: "u_vparseMessage",
        kind: ExportKind::Cdecl,
        args: "str ptr long ptr long long ptr",
    },
    Export {
        name: "u_vparseMessageWithError",
        kind: ExportKind::Cdecl,
        args: "str ptr long ptr long long ptr ptr",
    },
    Export {
        name: "ucal_add",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "ucal_clear",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucal_clearField",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucal_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucal_countAvailable",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "ucal_equivalentTo",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_get",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_getAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucal_getAvailable",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "ucal_getCanonicalTimeZoneID",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "ucal_getDSTSavings",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_getDayOfWeekType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_getDefaultTimeZone",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_getFieldDifference",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "ucal_getGregorianChange",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_getKeywordValuesForLocale",
        kind: ExportKind::Cdecl,
        args: "str str long ptr",
    },
    Export {
        name: "ucal_getLimit",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "ucal_getLocaleByType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_getMillis",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_getNow",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "ucal_getTZDataVersion",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucal_getTimeZoneDisplayName",
        kind: ExportKind::Cdecl,
        args: "ptr long str ptr long ptr",
    },
    Export {
        name: "ucal_getTimeZoneID",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucal_getTimeZoneIDForWindowsID",
        kind: ExportKind::Cdecl,
        args: "ptr long str ptr long ptr",
    },
    Export {
        name: "ucal_getTimeZoneTransitionDate",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr",
    },
    Export {
        name: "ucal_getType",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_getWeekendTransition",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_getWindowsTimeZoneID",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "ucal_inDaylightTime",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_isSet",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucal_isWeekend",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_open",
        kind: ExportKind::Cdecl,
        args: "ptr long str long ptr",
    },
    Export {
        name: "ucal_openCountryTimeZones",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ucal_openTimeZoneIDEnumeration",
        kind: ExportKind::Cdecl,
        args: "long str ptr ptr",
    },
    Export {
        name: "ucal_openTimeZones",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucal_roll",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "ucal_set",
        kind: ExportKind::Cdecl,
        args: "ptr long long",
    },
    Export {
        name: "ucal_setAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long long",
    },
    Export {
        name: "ucal_setDate",
        kind: ExportKind::Cdecl,
        args: "ptr long long long ptr",
    },
    Export {
        name: "ucal_setDateTime",
        kind: ExportKind::Cdecl,
        args: "ptr long long long long long long ptr",
    },
    Export {
        name: "ucal_setDefaultTimeZone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucal_setGregorianChange",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_setMillis",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucal_setTimeZone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_cloneBinary",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_closeElements",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_countAvailable",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "ucol_equal",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long",
    },
    Export {
        name: "ucol_getAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucol_getAvailable",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "ucol_getBound",
        kind: ExportKind::Cdecl,
        args: "ptr long long long ptr long ptr",
    },
    Export {
        name: "ucol_getContractionsAndExpansions",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr",
    },
    Export {
        name: "ucol_getDisplayName",
        kind: ExportKind::Cdecl,
        args: "str str ptr long ptr",
    },
    Export {
        name: "ucol_getEquivalentReorderCodes",
        kind: ExportKind::Cdecl,
        args: "long ptr long ptr",
    },
    Export {
        name: "ucol_getFunctionalEquivalent",
        kind: ExportKind::Cdecl,
        args: "str long str str ptr ptr",
    },
    Export {
        name: "ucol_getKeywordValues",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ucol_getKeywordValuesForLocale",
        kind: ExportKind::Cdecl,
        args: "str str long ptr",
    },
    Export {
        name: "ucol_getKeywords",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_getLocaleByType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucol_getMaxExpansion",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucol_getMaxVariable",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_getOffset",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_getReorderCodes",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_getRules",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucol_getRulesEx",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long",
    },
    Export {
        name: "ucol_getSortKey",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long",
    },
    Export {
        name: "ucol_getStrength",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_getTailoredSet",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucol_getUCAVersion",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucol_getVariableTop",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucol_getVersion",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucol_greater",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long",
    },
    Export {
        name: "ucol_greaterOrEqual",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long",
    },
    Export {
        name: "ucol_keyHashCode",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucol_mergeSortkeys",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr long",
    },
    Export {
        name: "ucol_next",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucol_nextSortKeyPart",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "ucol_open",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ucol_openAvailableLocales",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_openBinary",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr",
    },
    Export {
        name: "ucol_openElements",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_openRules",
        kind: ExportKind::Cdecl,
        args: "ptr long long long ptr ptr",
    },
    Export {
        name: "ucol_previous",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucol_primaryOrder",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "ucol_reset",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucol_safeClone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "ucol_secondaryOrder",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "ucol_setAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "ucol_setMaxVariable",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucol_setOffset",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ucol_setReorderCodes",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_setStrength",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucol_setText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucol_strcoll",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long",
    },
    Export {
        name: "ucol_strcollIter",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "ucol_strcollUTF8",
        kind: ExportKind::Cdecl,
        args: "ptr str long str long ptr",
    },
    Export {
        name: "ucol_tertiaryOrder",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "ucsdet_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucsdet_detect",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucsdet_detectAll",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "ucsdet_enableInputFilter",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "ucsdet_getAllDetectableCharsets",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucsdet_getConfidence",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucsdet_getLanguage",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucsdet_getName",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ucsdet_getUChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ucsdet_isInputFilterEnabled",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucsdet_open",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ucsdet_setDeclaredEncoding",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr",
    },
    Export {
        name: "ucsdet_setText",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr",
    },
    Export {
        name: "udat_adoptNumberFormat",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udat_adoptNumberFormatForFields",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "udat_applyPattern",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long",
    },
    Export {
        name: "udat_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udat_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udat_countAvailable",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "udat_countSymbols",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "udat_format",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "udat_formatCalendar",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "udat_formatCalendarForFields",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "udat_formatForFields",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "udat_get2DigitYearStart",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udat_getAvailable",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "udat_getBooleanAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udat_getCalendar",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udat_getContext",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udat_getLocaleByType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udat_getNumberFormat",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udat_getNumberFormatForField",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "udat_getSymbols",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr long ptr",
    },
    Export {
        name: "udat_isLenient",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udat_open",
        kind: ExportKind::Cdecl,
        args: "long long str ptr long ptr long ptr",
    },
    Export {
        name: "udat_parse",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "udat_parseCalendar",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "udat_set2DigitYearStart",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udat_setBooleanAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "udat_setCalendar",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udat_setContext",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udat_setLenient",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "udat_setNumberFormat",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udat_setSymbols",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr long ptr",
    },
    Export {
        name: "udat_toCalendarDateField",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "udat_toPattern",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "udatpg_addPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long long ptr long ptr ptr",
    },
    Export {
        name: "udatpg_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udatpg_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udatpg_getAppendItemFormat",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udatpg_getAppendItemName",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "udatpg_getBaseSkeleton",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "udatpg_getBestPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "udatpg_getBestPatternWithOptions",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long long ptr long ptr",
    },
    Export {
        name: "udatpg_getDateTimeFormat",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udatpg_getDecimal",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udatpg_getPatternForSkeleton",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "udatpg_getSkeleton",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "udatpg_open",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "udatpg_openBaseSkeletons",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udatpg_openEmpty",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udatpg_openSkeletons",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "udatpg_replaceFieldTypes",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr long ptr",
    },
    Export {
        name: "udatpg_replaceFieldTypesWithOptions",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long long ptr long ptr",
    },
    Export {
        name: "udatpg_setAppendItemFormat",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long",
    },
    Export {
        name: "udatpg_setAppendItemName",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long",
    },
    Export {
        name: "udatpg_setDateTimeFormat",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long",
    },
    Export {
        name: "udatpg_setDecimal",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long",
    },
    Export {
        name: "udtitvfmt_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "udtitvfmt_format",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr long ptr ptr",
    },
    Export {
        name: "udtitvfmt_open",
        kind: ExportKind::Cdecl,
        args: "str ptr long ptr long ptr",
    },
    Export {
        name: "ufieldpositer_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ufieldpositer_next",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "ufieldpositer_open",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ufmt_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ufmt_getArrayItemByIndex",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "ufmt_getArrayLength",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getDate",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getDecNumChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "ufmt_getDouble",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getInt64",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getLong",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getObject",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getType",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "ufmt_getUChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "ufmt_isNumeric",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ufmt_open",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ugender_getInstance",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ugender_getListGender",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ulocdata_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ulocdata_getCLDRVersion",
        kind: ExportKind::Cdecl,
        args: "long ptr",
    },
    Export {
        name: "ulocdata_getDelimiter",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "ulocdata_getExemplarSet",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long long ptr",
    },
    Export {
        name: "ulocdata_getLocaleDisplayPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ulocdata_getLocaleSeparator",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "ulocdata_getMeasurementSystem",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ulocdata_getNoSubstitute",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ulocdata_getPaperSize",
        kind: ExportKind::Cdecl,
        args: "str ptr ptr ptr",
    },
    Export {
        name: "ulocdata_open",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "ulocdata_setNoSubstitute",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "umsg_applyPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "umsg_autoQuoteApostrophe",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "umsg_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "umsg_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "umsg_format",
        kind: ExportKind::Varargs,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "umsg_getLocale",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "umsg_open",
        kind: ExportKind::Cdecl,
        args: "ptr long str ptr ptr",
    },
    Export {
        name: "umsg_parse",
        kind: ExportKind::Varargs,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "umsg_setLocale",
        kind: ExportKind::Cdecl,
        args: "ptr str",
    },
    Export {
        name: "umsg_toPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "umsg_vformat",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long long ptr",
    },
    Export {
        name: "umsg_vparse",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "unum_applyPattern",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "unum_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "unum_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unum_countAvailable",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "unum_format",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "unum_formatDecimal",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr long ptr ptr",
    },
    Export {
        name: "unum_formatDouble",
        kind: ExportKind::Cdecl,
        args: "ptr double ptr long ptr ptr",
    },
    Export {
        name: "unum_formatDoubleCurrency",
        kind: ExportKind::Cdecl,
        args: "ptr double ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_formatDoubleForFields",
        kind: ExportKind::Cdecl,
        args: "ptr double ptr long ptr ptr",
    },
    Export {
        name: "unum_formatInt64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr",
    },
    Export {
        name: "unum_formatUFormattable",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_getAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "unum_getAvailable",
        kind: ExportKind::Cdecl,
        args: "long",
    },
    Export {
        name: "unum_getContext",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "unum_getDoubleAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "unum_getLocaleByType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "unum_getSymbol",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "unum_getTextAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "unum_open",
        kind: ExportKind::Cdecl,
        args: "long ptr long str ptr ptr",
    },
    Export {
        name: "unum_parse",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_parseDecimal",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr str long ptr",
    },
    Export {
        name: "unum_parseDouble",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_parseDoubleCurrency",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr ptr",
    },
    Export {
        name: "unum_parseInt64",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_parseToUFormattable",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "unum_setAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long long",
    },
    Export {
        name: "unum_setContext",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "unum_setDoubleAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long double",
    },
    Export {
        name: "unum_setSymbol",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "unum_setTextAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "unum_toPattern",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "unumsys_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unumsys_getDescription",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "unumsys_getName",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unumsys_getRadix",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unumsys_isAlgorithmic",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unumsys_open",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "unumsys_openAvailableNames",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "unumsys_openByName",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "uplrules_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uplrules_getKeywords",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uplrules_open",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "uplrules_openForType",
        kind: ExportKind::Cdecl,
        args: "str long ptr",
    },
    Export {
        name: "uplrules_select",
        kind: ExportKind::Cdecl,
        args: "ptr double ptr long ptr",
    },
    Export {
        name: "uregex_appendReplacement",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr ptr",
    },
    Export {
        name: "uregex_appendReplacementUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_appendTail",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_appendTailUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uregex_end",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_end64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_find",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_find64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_findNext",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_flags",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_getFindProgressCallback",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_getMatchCallback",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_getStackLimit",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_getText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_getTimeLimit",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_getUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_group",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "uregex_groupCount",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_groupNumberFromCName",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr",
    },
    Export {
        name: "uregex_groupNumberFromName",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "uregex_groupUText",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr ptr",
    },
    Export {
        name: "uregex_hasAnchoringBounds",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_hasTransparentBounds",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_hitEnd",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_lookingAt",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_lookingAt64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_matches",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_matches64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_open",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr ptr",
    },
    Export {
        name: "uregex_openC",
        kind: ExportKind::Cdecl,
        args: "str long ptr ptr",
    },
    Export {
        name: "uregex_openUText",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr",
    },
    Export {
        name: "uregex_pattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_patternUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_refreshUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_regionEnd",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_regionEnd64",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_regionStart",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_regionStart64",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_replaceAll",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "uregex_replaceAllUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_replaceFirst",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "uregex_replaceFirstUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_requireEnd",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregex_reset",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_reset64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_setFindProgressCallback",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_setMatchCallback",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr",
    },
    Export {
        name: "uregex_setRegion",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "uregex_setRegion64",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "uregex_setRegionAndStart",
        kind: ExportKind::Cdecl,
        args: "ptr long long long ptr",
    },
    Export {
        name: "uregex_setStackLimit",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_setText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "uregex_setTimeLimit",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_setUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uregex_split",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr long ptr",
    },
    Export {
        name: "uregex_splitUText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "uregex_start",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_start64",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_useAnchoringBounds",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregex_useTransparentBounds",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregion_areEqual",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregion_contains",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregion_getAvailable",
        kind: ExportKind::Cdecl,
        args: "long ptr",
    },
    Export {
        name: "uregion_getContainedRegions",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregion_getContainedRegionsOfType",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uregion_getContainingRegion",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uregion_getContainingRegionOfType",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "uregion_getNumericCode",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uregion_getPreferredValues",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uregion_getRegionCode",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uregion_getRegionFromCode",
        kind: ExportKind::Cdecl,
        args: "str ptr",
    },
    Export {
        name: "uregion_getRegionFromNumericCode",
        kind: ExportKind::Cdecl,
        args: "long ptr",
    },
    Export {
        name: "uregion_getType",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ureldatefmt_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "ureldatefmt_combineDateAndTime",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr long ptr",
    },
    Export {
        name: "ureldatefmt_format",
        kind: ExportKind::Cdecl,
        args: "ptr double long ptr long ptr",
    },
    Export {
        name: "ureldatefmt_formatNumeric",
        kind: ExportKind::Cdecl,
        args: "ptr double long ptr long ptr",
    },
    Export {
        name: "ureldatefmt_open",
        kind: ExportKind::Cdecl,
        args: "str ptr long long ptr",
    },
    Export {
        name: "usearch_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_first",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_following",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "usearch_getAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "usearch_getBreakIterator",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_getCollator",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_getMatchedLength",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_getMatchedStart",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_getMatchedText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "usearch_getOffset",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_getPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_getText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_last",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_next",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_open",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long str ptr ptr",
    },
    Export {
        name: "usearch_openFromCollator",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr ptr ptr",
    },
    Export {
        name: "usearch_preceding",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "usearch_previous",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "usearch_reset",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "usearch_setAttribute",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr",
    },
    Export {
        name: "usearch_setBreakIterator",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "usearch_setCollator",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "usearch_setOffset",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "usearch_setPattern",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "usearch_setText",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "uspoof_areConfusable",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr long ptr",
    },
    Export {
        name: "uspoof_areConfusableUTF8",
        kind: ExportKind::Cdecl,
        args: "ptr str long str long ptr",
    },
    Export {
        name: "uspoof_check",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "uspoof_check2",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr ptr",
    },
    Export {
        name: "uspoof_check2UTF8",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr ptr",
    },
    Export {
        name: "uspoof_checkUTF8",
        kind: ExportKind::Cdecl,
        args: "ptr str long ptr ptr",
    },
    Export {
        name: "uspoof_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_closeCheckResult",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_getAllowedChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getAllowedLocales",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getCheckResultChecks",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getCheckResultNumerics",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getCheckResultRestrictionLevel",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getChecks",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "uspoof_getInclusionSet",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_getRecommendedSet",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_getRestrictionLevel",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_getSkeleton",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr long ptr",
    },
    Export {
        name: "uspoof_getSkeletonUTF8",
        kind: ExportKind::Cdecl,
        args: "ptr long str long str long ptr",
    },
    Export {
        name: "uspoof_open",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_openCheckResult",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "uspoof_openFromSerialized",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr",
    },
    Export {
        name: "uspoof_openFromSource",
        kind: ExportKind::Cdecl,
        args: "str long str long ptr ptr ptr",
    },
    Export {
        name: "uspoof_serialize",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "uspoof_setAllowedChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr",
    },
    Export {
        name: "uspoof_setAllowedLocales",
        kind: ExportKind::Cdecl,
        args: "ptr str ptr",
    },
    Export {
        name: "uspoof_setChecks",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr",
    },
    Export {
        name: "uspoof_setRestrictionLevel",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
    Export {
        name: "utmscale_fromInt64",
        kind: ExportKind::Cdecl,
        args: "long long ptr",
    },
    Export {
        name: "utmscale_getTimeScaleValue",
        kind: ExportKind::Cdecl,
        args: "long long ptr",
    },
    Export {
        name: "utmscale_toInt64",
        kind: ExportKind::Cdecl,
        args: "long long ptr",
    },
    Export {
        name: "utrans_clone",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "utrans_close",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "utrans_countAvailableIDs",
        kind: ExportKind::Cdecl,
        args: "",
    },
    Export {
        name: "utrans_getSourceSet",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr ptr",
    },
    Export {
        name: "utrans_getUnicodeID",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "utrans_openIDs",
        kind: ExportKind::Cdecl,
        args: "ptr",
    },
    Export {
        name: "utrans_openInverse",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "utrans_openU",
        kind: ExportKind::Cdecl,
        args: "ptr long long ptr long ptr ptr",
    },
    Export {
        name: "utrans_register",
        kind: ExportKind::Cdecl,
        args: "ptr ptr",
    },
    Export {
        name: "utrans_setFilter",
        kind: ExportKind::Cdecl,
        args: "ptr ptr long ptr",
    },
    Export {
        name: "utrans_toRules",
        kind: ExportKind::Cdecl,
        args: "ptr long ptr long ptr",
    },
    Export {
        name: "utrans_trans",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "utrans_transIncremental",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr ptr ptr",
    },
    Export {
        name: "utrans_transIncrementalUChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long ptr ptr",
    },
    Export {
        name: "utrans_transUChars",
        kind: ExportKind::Cdecl,
        args: "ptr ptr ptr long long ptr ptr",
    },
    Export {
        name: "utrans_unregisterID",
        kind: ExportKind::Cdecl,
        args: "ptr long",
    },
];

/// The export named `name`, or `None` when the spec does not export it. Names are case-sensitive.
pub fn find_export(name: &str) -> Option<Export> {
    EXPORTS.iter().find(|export| export.name == name).copied()
}

/// Where the export named `name` forwards, or `None` when the spec does not export it.
pub fn forward(name: &str) -> Option<Forward> {
    find_export(name).map(|export| Forward {
        module: FORWARD_MODULE,
        function: export.name,
    })
}
