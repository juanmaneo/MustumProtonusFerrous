/// `sizeof(LINECOUNTRYLIST)`: the header, six DWORDs.
const LIST_SIZE: u32 = 24;
/// `sizeof(LINECOUNTRYENTRY)`: eleven DWORDs.
const ENTRY_SIZE: u32 = 44;
/// `LINEERR_INIFILECORRUPT` from `include/tapi.h`, returned when the country list key does not open.
const LINEERR_INIFILECORRUPT: u32 = 0x8000_000E;
/// `LINEERR_STRUCTURETOOSMALL` from `include/tapi.h`.
const LINEERR_STRUCTURETOOSMALL: u32 = 0x8000_004D;

/// One subkey of `Software\Microsoft\Windows\CurrentVersion\Telephony\Country List`, in
/// enumeration order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountryKey<'a> {
    /// The subkey name, which is the country ID in decimal.
    pub name: &'a str,
    /// The values the C body reads, or `None` when the subkey cannot be opened and is skipped.
    pub rules: Option<CountryRules>,
}

/// The values of one country subkey. Each rule is the raw `REG_SZ` data, including the NUL.
/// A value that is missing from the registry is modelled as empty data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountryRules {
    pub country_code: u32,
    pub international_rule: Vec<u8>,
    pub long_distance_rule: Vec<u8>,
    pub country_name: Vec<u8>,
    pub same_area_rule: Vec<u8>,
}

/// `lineGetCountryA` and `lineGetCountryW` from `dlls/tapi32/line.c`. The A and W bodies differ
/// only in registry failure handling and in `atoi` against `wcstol`. For the country list keys
/// both give the same result, so one model serves both.
///
/// `list` is the caller's `LINECOUNTRYLIST` buffer, and its first DWORD is `dwTotalSize`. `keys`
/// is `None` when the country list key does not open. Entries are written at their enumeration
/// index, so a skipped subkey leaves a zeroed slot. When `country_id` is not 0, only the subkey
/// with that ID is written, into entry 0. Writes past the end of `list` are dropped.
pub fn line_get_country(list: &mut [u8], country_id: u32, keys: Option<&[CountryKey]>) -> u32 {
    let Some(keys) = keys else {
        return LINEERR_INIFILECORRUPT;
    };
    let total = read_u32(list, 0);
    if total < LIST_SIZE {
        return LINEERR_STRUCTURETOOSMALL;
    }
    let span = (total as usize).min(list.len());
    list[..span].fill(0);

    let by_id = country_id != 0;
    let mut used = LIST_SIZE;
    let mut num_countries = 0;
    let mut list_size = 0;
    let mut offset = if by_id {
        ENTRY_SIZE
    } else {
        LIST_SIZE + keys.len() as u32 * ENTRY_SIZE
    };

    for (enum_index, key) in keys.iter().enumerate() {
        let id = country_id_of(key.name);
        if by_id && id != country_id {
            continue;
        }
        let Some(rules) = &key.rules else {
            continue;
        };
        let len = (rules.international_rule.len()
            + rules.long_distance_rule.len()
            + rules.country_name.len()
            + rules.same_area_rule.len()) as u32;

        if total < offset + len {
            offset += len;
            if by_id {
                break;
            }
            continue;
        }
        num_countries += 1;
        list_size += ENTRY_SIZE;
        used += len + ENTRY_SIZE;

        let index = if by_id { 0 } else { enum_index };
        let entry = LIST_SIZE as usize + index * ENTRY_SIZE as usize;
        put_u32(list, entry, id);
        put_u32(list, entry + 4, rules.country_code);
        put_u32(list, entry + 8, 0);
        if index > 0 {
            put_u32(list, entry - ENTRY_SIZE as usize + 8, id);
        }

        put_rule(list, entry + 12, &mut offset, &rules.country_name);
        put_rule(list, entry + 20, &mut offset, &rules.same_area_rule);
        put_rule(list, entry + 28, &mut offset, &rules.long_distance_rule);
        put_rule(list, entry + 36, &mut offset, &rules.international_rule);

        if by_id {
            break;
        }
    }

    // The header lies below offset 24, which no entry or string write reaches, so the C body's
    // header writes can be done once at the end.
    put_u32(list, 0, total);
    put_u32(list, 4, offset);
    put_u32(list, 8, used);
    put_u32(list, 12, num_countries);
    put_u32(list, 16, list_size);
    put_u32(list, 20, LIST_SIZE);
    0
}

/// `wcstol(subkey_name, NULL, 10)` on a country subkey name. Leading white space and an optional
/// sign are accepted, then decimal digits. The result clamps to the range of `long`, which is
/// 32 bits on Windows, and is returned as the DWORD that the C code stores.
pub fn country_id_of(name: &str) -> u32 {
    let bytes = name.as_bytes();
    let mut i = 0;
    while bytes
        .get(i)
        .is_some_and(|b| *b == b' ' || (b'\t'..=b'\r').contains(b))
    {
        i += 1;
    }
    let negative = bytes.get(i) == Some(&b'-');
    if matches!(bytes.get(i), Some(b'+' | b'-')) {
        i += 1;
    }
    let mut magnitude: i64 = 0;
    while let Some(digit) = bytes.get(i).filter(|b| b.is_ascii_digit()) {
        magnitude = (magnitude * 10 + i64::from(digit - b'0')).min(1 << 32);
        i += 1;
    }
    let value = if negative { -magnitude } else { magnitude };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32 as u32
}

/// Stores one rule's size and offset in its entry field pair, then copies its data to `offset`.
/// `field` is the byte offset of the size DWORD inside the entry.
fn put_rule(list: &mut [u8], field: usize, offset: &mut u32, data: &[u8]) {
    put_u32(list, field, data.len() as u32);
    put_u32(list, field + 4, *offset);
    put_bytes(list, *offset as usize, data);
    *offset += data.len() as u32;
}

fn read_u32(bytes: &[u8], at: usize) -> u32 {
    let mut word = [0; 4];
    if let Some(src) = bytes.get(at..at + 4) {
        word.copy_from_slice(src);
    }
    u32::from_le_bytes(word)
}

fn put_u32(list: &mut [u8], at: usize, value: u32) {
    put_bytes(list, at, &value.to_le_bytes());
}

fn put_bytes(list: &mut [u8], at: usize, data: &[u8]) {
    let end = at.saturating_add(data.len()).min(list.len());
    if at < end {
        list[at..end].copy_from_slice(&data[..end - at]);
    }
}
