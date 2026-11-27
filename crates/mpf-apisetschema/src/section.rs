//! `output_apiset_section` from `tools/winebuild/spec32.c`: the `.apiset` section for ApiSetSchema version 6.

use crate::hash::HASH_FACTOR;
use crate::schema::Schema;

/// The section header: `0x1c` bytes, seven dwords.
const HEADER_SIZE: usize = 0x1c;
/// One name entry: six dwords.
const NAME_SIZE: usize = 0x18;
/// One value entry: five dwords.
const VALUE_SIZE: usize = 0x14;

impl Schema {
    /// The section bytes, little-endian. Offsets are relative to the start of the section.
    pub fn section(&self) -> Vec<u8> {
        let count = self.entries.len();
        let value_start = HEADER_SIZE + count * NAME_SIZE;
        let strings_start = self.entries.iter().fold(value_start, |pos, entry| {
            pos + VALUE_SIZE * entry.values.len().max(1)
        });
        let pool = self.pool.bytes();
        let hash_pos = strings_start + ((pool.len() * 2 + 3) & !3);
        let size = hash_pos + count * 8;

        let mut out = Vec::with_capacity(size);
        put_dword(&mut out, 6);
        put_dword(&mut out, size);
        put_dword(&mut out, 0);
        put_dword(&mut out, count);
        put_dword(&mut out, HEADER_SIZE);
        put_dword(&mut out, hash_pos);
        put_dword(&mut out, HASH_FACTOR as usize);

        let mut value_pos = value_start;
        for entry in &self.entries {
            let slots = entry.values.len().max(1);
            put_dword(&mut out, 1);
            put_dword(&mut out, strings_start + entry.name_off * 2);
            put_dword(&mut out, entry.name_len * 2);
            put_dword(&mut out, entry.hash_len * 2);
            put_dword(&mut out, value_pos);
            put_dword(&mut out, slots);
            value_pos += VALUE_SIZE * slots;
        }

        for entry in &self.entries {
            if entry.values.is_empty() {
                out.extend_from_slice(&[0; VALUE_SIZE]);
            }
            for value in &entry.values {
                put_dword(&mut out, 0);
                if value.name_off != 0 {
                    put_dword(&mut out, strings_start + value.name_off * 2);
                    put_dword(&mut out, value.name_len * 2);
                } else {
                    put_dword(&mut out, 0);
                    put_dword(&mut out, 0);
                }
                put_dword(&mut out, strings_start + value.val_off * 2);
                put_dword(&mut out, value.val_len * 2);
            }
        }

        for &byte in pool {
            // `put_word` takes an `unsigned short`, so a `char` from 0x80 up is sign-extended to 16 bits.
            out.extend_from_slice(&(byte as i8 as u16).to_le_bytes());
        }
        out.resize(hash_pos, 0);

        // Hashes are unique, because `parse_line` rejects a name whose hash is taken, so the sort order is fixed.
        let mut hashes: Vec<(u32, usize)> = self
            .entries
            .iter()
            .enumerate()
            .map(|(index, entry)| (entry.hash, index))
            .collect();
        hashes.sort_by_key(|&(hash, _)| hash);
        for (hash, index) in hashes {
            put_dword(&mut out, hash as usize);
            put_dword(&mut out, index);
        }
        out
    }
}

fn put_dword(out: &mut Vec<u8>, value: usize) {
    out.extend_from_slice(&(value as u32).to_le_bytes());
}
