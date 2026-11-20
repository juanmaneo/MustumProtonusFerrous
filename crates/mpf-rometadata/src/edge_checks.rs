//! Direct checks of the branches that the oracle rows do not reach: boundary values, the `None`
//! outcomes of each reader, the wrapping arithmetic, and the asserts that the port keeps from C.

use crate::coded::{bit_width, coded_index_size, coded_value_as_token, index_size};
use crate::enumeration::TokenEnum;
use crate::heap::{
    blob_at, decode_compressed_uint, guid_offset, span_in_bounds, string_at, trim_string_heap,
};
use crate::pe::{rva_to_offset, Section};
use crate::token_from_rid;
use crate::typepath::split_type_name;

#[test]
fn bit_width_boundaries() {
    assert_eq!(bit_width(0), 32);
    assert_eq!(bit_width(1), 1);
    assert_eq!(bit_width(2), 1);
    assert_eq!(bit_width(3), 2);
    assert_eq!(bit_width(4), 2);
    assert_eq!(bit_width(5), 3);
    assert_eq!(bit_width(u32::MAX), 32);
}

#[test]
fn index_size_boundary() {
    assert_eq!(index_size(65535), 2);
    assert_eq!(index_size(65536), 4);
}

#[test]
fn coded_index_size_boundaries() {
    assert_eq!(coded_index_size(&[Some(32767), None]), 2);
    assert_eq!(coded_index_size(&[Some(32768), None]), 4);
    assert_eq!(coded_index_size(&[None, None]), 2);
    assert_eq!(coded_index_size(&[Some(1), Some(40000), None, None]), 4);
}

#[test]
#[should_panic(expected = "at least one table")]
fn coded_index_size_rejects_empty_list() {
    coded_index_size(&[]);
}

#[test]
fn coded_value_decodes_tag_and_row() {
    assert_eq!(coded_value_as_token(6, &[0, 1]), Some(3));
    assert_eq!(coded_value_as_token(0, &[45]), Some(0x2d00_0000));
    assert_eq!(coded_value_as_token(3, &[0, 1, 2]), None);
    assert_eq!(coded_value_as_token(1, &[0]), None);
}

#[test]
#[should_panic(expected = "at least one table")]
fn coded_value_rejects_empty_list() {
    coded_value_as_token(0, &[]);
}

#[test]
fn compressed_uint_lengths() {
    assert_eq!(decode_compressed_uint(&[0x7f]), Some((127, 1)));
    assert_eq!(decode_compressed_uint(&[0x80, 0x01]), Some((1, 2)));
    assert_eq!(decode_compressed_uint(&[0xbf, 0xff]), Some((16383, 2)));
    assert_eq!(decode_compressed_uint(&[0xc0, 0, 0, 5]), Some((5, 4)));
    assert_eq!(
        decode_compressed_uint(&[0xdf, 0xff, 0xff, 0xff]),
        Some((0x1fff_ffff, 4))
    );
}

#[test]
fn compressed_uint_rejects_truncated_and_invalid() {
    assert_eq!(decode_compressed_uint(&[0xe0, 0, 0, 0]), None);
    assert_eq!(decode_compressed_uint(&[0xff]), None);
    assert_eq!(decode_compressed_uint(&[]), None);
    assert_eq!(decode_compressed_uint(&[0x80]), None);
    assert_eq!(decode_compressed_uint(&[0xc0, 0, 0]), None);
}

#[test]
fn blob_at_reads_prefix_and_tail() {
    assert_eq!(blob_at(b"\x01\x02", 0), Some((1, &b"\x02"[..])));
    assert_eq!(blob_at(b"\x01\x02", 2), None);
    assert_eq!(blob_at(b"\x01\x02", 3), None);
    assert_eq!(blob_at(b"\x01", 0), None);
}

#[test]
fn string_at_reads_to_end_of_heap() {
    assert_eq!(string_at(b"ab\0", 1), Some(&b"b\0"[..]));
    assert_eq!(string_at(b"ab", 2), None);
}

#[test]
fn guid_offset_is_one_based() {
    assert_eq!(guid_offset(1, 32), Some(0));
    assert_eq!(guid_offset(2, 32), Some(16));
    assert_eq!(guid_offset(3, 32), None);
    assert_eq!(guid_offset(0, 32), None);
}

#[test]
fn trim_string_heap_keeps_one_nul() {
    assert_eq!(trim_string_heap(&[]), Some(&[][..]));
    assert_eq!(trim_string_heap(&[1]), None);
    assert_eq!(trim_string_heap(&[0]), Some(&[0][..]));
    assert_eq!(trim_string_heap(&[0, 0, 0, 0]), Some(&[0][..]));
    assert_eq!(trim_string_heap(&[0, 0x61]), Some(&[0, 0x61][..]));
}

#[test]
fn span_in_bounds_edges() {
    assert!(span_in_bounds(16, 0, 16));
    assert!(span_in_bounds(16, 15, 1));
    assert!(!span_in_bounds(16, 16, 0));
    assert!(!span_in_bounds(16, 8, 9));
    assert!(!span_in_bounds(0, 0, 0));
    assert!(!span_in_bounds(u64::MAX, 4, u64::MAX));
}

#[test]
fn rva_to_offset_section_edges() {
    assert_eq!(rva_to_offset(&[], 0), None);
    let sections = [Section {
        virtual_address: 0x1000,
        virtual_size: 0x100,
        raw_pointer: 0x400,
    }];
    assert_eq!(rva_to_offset(&sections, 0x1000), Some(0x400));
    assert_eq!(rva_to_offset(&sections, 0x10ff), Some(0x4ff));
    assert_eq!(rva_to_offset(&sections, 0x1100), None);
    assert_eq!(rva_to_offset(&sections, 0xfff), None);
}

#[test]
fn rva_to_offset_wraps_in_32_bits() {
    let wrapped_end = [Section {
        virtual_address: u32::MAX - 9,
        virtual_size: 20,
        raw_pointer: 0,
    }];
    assert_eq!(rva_to_offset(&wrapped_end, u32::MAX - 9), None);

    let wrapped_offset = [Section {
        virtual_address: 100,
        virtual_size: 50,
        raw_pointer: u32::MAX - 9,
    }];
    assert_eq!(rva_to_offset(&wrapped_offset, 120), Some(10));
}

#[test]
fn split_type_name_at_last_dot() {
    let plain = split_type_name(b"Name");
    assert_eq!((plain.namespace, plain.type_name), (&b""[..], &b"Name"[..]));
    let nested = split_type_name(b"A.B.C");
    assert_eq!(
        (nested.namespace, nested.type_name),
        (&b"A.B"[..], &b"C"[..])
    );
    let trailing = split_type_name(b"A.");
    assert_eq!(
        (trailing.namespace, trailing.type_name),
        (&b"A"[..], &b""[..])
    );
    let leading = split_type_name(b".X");
    assert_eq!(
        (leading.namespace, leading.type_name),
        (&b""[..], &b"X"[..])
    );
}

#[test]
fn type_path_equals_compares_both_parts() {
    let ab = split_type_name(b"A.B");
    assert!(ab.equals(&split_type_name(b"A.B")));
    assert!(!ab.equals(&split_type_name(b"A.C")));
    assert!(!ab.equals(&split_type_name(b"AB")));
}

#[test]
fn range_count_and_wrap() {
    assert_eq!(TokenEnum::range(0x0200_0000, 1, 3).count(), 3);
    assert_eq!(TokenEnum::range(0, 0, 5).count(), 6);
    assert_eq!(TokenEnum::list(vec![1, 2, 3]).count(), 3);
}

#[test]
fn range_get_entries_advances_across_calls() {
    let mut henum = TokenEnum::range(0x0200_0000, 1, 3);
    let mut two = [0; 2];
    assert_eq!(henum.get_entries(&mut two), 2);
    assert_eq!(two, [0x0200_0001, 0x0200_0002]);
    assert_eq!(henum.position(), 3);

    let mut one = [0; 1];
    assert_eq!(henum.get_entries(&mut one), 1);
    assert_eq!(one, [0x0200_0003]);
    assert_eq!(henum.position(), 4);

    assert_eq!(henum.get_entries(&mut []), 0);
}

#[test]
fn range_reset_moves_cursor_from_start() {
    let mut henum = TokenEnum::range(0x0200_0000, 1, 3);
    henum.reset(1);
    assert_eq!(henum.position(), 2);

    let mut wrapped = TokenEnum::range(0, 5, 6);
    wrapped.reset(u32::MAX);
    assert_eq!(wrapped.position(), 4);
}

#[test]
fn range_get_entries_wraps_at_max() {
    let mut henum = TokenEnum::range(0, u32::MAX, u32::MAX);
    let mut buf = [0; 2];
    assert_eq!(henum.get_entries(&mut buf), 2);
    assert_eq!(buf, [u32::MAX, 0]);
    assert_eq!(henum.position(), 1);
}

#[test]
fn list_reset_and_end_of_list() {
    let mut henum = TokenEnum::list(vec![0xa, 0xb, 0xc]);
    henum.reset(1);
    let mut buf = [0; 2];
    assert_eq!(henum.get_entries(&mut buf), 2);
    assert_eq!(buf, [0xb, 0xc]);
    assert_eq!(henum.position(), 3);
    assert_eq!(henum.get_entries(&mut buf), 0);

    let mut past_end = TokenEnum::list(vec![0xa, 0xb, 0xc]);
    past_end.reset(7);
    assert_eq!(past_end.get_entries(&mut buf), 0);
    assert_eq!(past_end.position(), 7);
}

#[test]
fn list_get_entries_with_empty_buffer() {
    let mut henum = TokenEnum::list(vec![1]);
    assert_eq!(henum.get_entries(&mut []), 0);
    assert_eq!(henum.position(), 0);
}

#[test]
#[should_panic(expected = "starts at or before its end")]
fn range_rejects_start_after_end() {
    TokenEnum::range(0, 5, 4);
}

#[test]
#[should_panic(expected = "is not empty")]
fn list_rejects_empty() {
    TokenEnum::list(Vec::new());
}

#[test]
fn token_from_rid_sets_table_tag() {
    assert_eq!(token_from_rid(0x10, 0x0200_0000), 0x0200_0010);
}
