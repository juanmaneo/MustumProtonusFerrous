use crate::Table;

/// `offsetof(struct gasp_header, ranges)`: the version and the range count come first.
const HEADER_SIZE: usize = 4;
/// `sizeof(struct gasp_range)`: `max_ppem` followed by `flags`.
const RANGE_SIZE: usize = 4;

/// `opentype_get_gasp_flags`: the `gasp` flags for `emsize` pixels per em.
///
/// Ranges are scanned in order and the scan stops at the first range whose `max_ppem` is at least
/// `emsize`. When none matches, the flags of the last range are returned. An absent table, a
/// table shorter than its range count, and a version above 1 all give 0.
pub fn gasp_flags(gasp: Table, emsize: f32) -> u16 {
    let num_ranges = usize::from(gasp.be_word(2));
    let Some(bytes) = gasp.read_ensure(0, HEADER_SIZE + RANGE_SIZE * num_ranges) else {
        return 0;
    };
    let table = Table::new(bytes);
    if table.be_word(0) > 1 {
        return 0;
    }

    let mut flags = 0;
    for i in 0..num_ranges {
        let range = HEADER_SIZE + RANGE_SIZE * i;
        flags = table.be_word(range + 2);
        if emsize <= f32::from(table.be_word(range)) {
            break;
        }
    }
    flags
}
