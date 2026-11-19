use crate::Table;

/// `sizeof(struct vdmx_header)`.
const HEADER_SIZE: usize = 6;
/// `sizeof(struct vdmx_ratio)`.
const RATIO_SIZE: usize = 4;
/// `sizeof(struct vdmx_group)`. The C header check counts this many bytes for each group.
const GROUP_SIZE: usize = 10;
/// `offsetof(struct vdmx_group, entries)`: the record count, then `startsz` and `endsz`.
const GROUP_ENTRIES: usize = 4;
/// `sizeof(struct vdmx_vtable)`: `yPelHeight`, `yMax` and `yMin`.
const ENTRY_SIZE: usize = 6;
/// The device ratio that `find_vdmx_group` matches, 1:1.
const DEVICE_RATIO: u8 = 1;

/// `find_vdmx_group`: the offset of the group for the 1:1 device ratio, or 0 when no ratio record
/// matches. A ratio matches when its `bCharSet` is set and either all three ratios are 0, or
/// `xRatio` is 1 and the `y` range contains 1.
fn find_group(vdmx: Table) -> usize {
    let num_ratios = usize::from(vdmx.be_word(4));
    for i in 0..num_ratios {
        let ratio = HEADER_SIZE + RATIO_SIZE * i;
        let char_set = vdmx.byte(ratio);
        let x_ratio = vdmx.byte(ratio + 1);
        let y_start = vdmx.byte(ratio + 2);
        let y_end = vdmx.byte(ratio + 3);
        if char_set == 0 {
            continue;
        }
        if (x_ratio == 0 && y_start == 0 && y_end == 0)
            || (x_ratio == DEVICE_RATIO && y_start <= DEVICE_RATIO && y_end >= DEVICE_RATIO)
        {
            return usize::from(vdmx.be_word(HEADER_SIZE + RATIO_SIZE * num_ratios + 2 * i));
        }
    }
    0
}

/// `opentype_get_vdmx_size`: the `(ascent, descent)` of the VDMX record for `emsize`.
///
/// Returns `None` when the table is absent or truncated, when no group matches the 1:1 device
/// ratio, when `emsize` is outside the group's `startsz..endsz` range, and when the group has no
/// record for `emsize`. Records are scanned in order and the scan stops at the first record above
/// `emsize`, because the C code does not interpolate. `descent` is the negated `yMin`, stored as
/// a `u16` as the C code does.
pub fn vdmx_size(vdmx: Table, emsize: i32) -> Option<(u16, u16)> {
    let num_ratios = usize::from(vdmx.be_word(4));
    let num_recs = usize::from(vdmx.be_word(2));
    vdmx.read_ensure(
        0,
        HEADER_SIZE + RATIO_SIZE * num_ratios + GROUP_SIZE * num_recs,
    )?;

    let group_offset = find_group(vdmx);
    if group_offset == 0 {
        return None;
    }

    let group_recs = usize::from(vdmx.be_word(group_offset));
    let group_bytes = vdmx.read_ensure(group_offset, GROUP_ENTRIES + ENTRY_SIZE * group_recs)?;
    let group = Table::new(group_bytes);
    if emsize < i32::from(group.byte(2)) || emsize >= i32::from(group.byte(3)) {
        return None;
    }

    for i in 0..group_recs {
        let entry = GROUP_ENTRIES + ENTRY_SIZE * i;
        let ppem = i32::from(group.be_word(entry));
        if ppem > emsize {
            return None;
        }
        if ppem == emsize {
            let ascent = group.be_word(entry + 2);
            let descent = (-i32::from(group.be_word(entry + 4) as i16)) as u16;
            return Some((ascent, descent));
        }
    }
    None
}
