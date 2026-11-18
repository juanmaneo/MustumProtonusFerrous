//! `HidP_GetCollectionDescription`: splits a report descriptor into its
//! top-level collections and tallies the bit length of each report ID.

use crate::parser::{parse_descriptor, top_level_collections};
use crate::{CollectionDesc, DeviceDesc, ReportIdDesc, ValueCaps, HIDP_STATUS_INTERNAL_ERROR};

/// The bit length of one value: the end of its last bit, from the report start.
fn caps_bits(caps: &ValueCaps) -> u32 {
    (u32::from(caps.start_byte) * 8)
        .wrapping_add(u32::from(caps.start_bit))
        .wrapping_add(u32::from(caps.bit_size) * u32::from(caps.report_count))
}

/// Grows the length of each report ID in `values`, and records `index` as its
/// collection.
fn track(
    lengths: &mut [u32; 256],
    collection: &mut [u32; 256],
    values: &[ValueCaps],
    index: usize,
) {
    for caps in values {
        let report = usize::from(caps.report_id);
        lengths[report] = lengths[report].max(caps_bits(caps));
        collection[report] = index as u32;
    }
}

/// A report length in bytes, rounded up.
fn report_bytes(bits: u32) -> u16 {
    (bits.wrapping_add(7) / 8) as u16
}

/// `HidP_GetCollectionDescription`: the collections of `report_desc` and the
/// report IDs they use. `Err` is `HIDP_STATUS_INTERNAL_ERROR`, returned when a
/// collection does not parse.
pub fn get_collection_description(report_desc: &[u8]) -> Result<DeviceDesc, i32> {
    let bounds = top_level_collections(report_desc);
    let mut collections = Vec::with_capacity(bounds.len().saturating_sub(1));
    let mut input = [0u32; 256];
    let mut output = [0u32; 256];
    let mut feature = [0u32; 256];
    let mut collection = [0u32; 256];

    for (index, span) in bounds.windows(2).enumerate() {
        let preparsed =
            parse_descriptor(&report_desc[span[0]..span[1]]).ok_or(HIDP_STATUS_INTERNAL_ERROR)?;
        track(&mut input, &mut collection, &preparsed.input.values, index);
        track(
            &mut output,
            &mut collection,
            &preparsed.output.values,
            index,
        );
        track(
            &mut feature,
            &mut collection,
            &preparsed.feature.values,
            index,
        );
        collections.push(CollectionDesc {
            usage_page: preparsed.usage_page,
            usage: preparsed.usage,
            collection_number: (index as u8).wrapping_add(1),
            input_length: preparsed.input.byte_length,
            output_length: preparsed.output.byte_length,
            feature_length: preparsed.feature.byte_length,
            preparsed,
        });
    }

    let mut report_ids = Vec::new();
    for id in 0..256usize {
        let (input_bits, output_bits, feature_bits) = (input[id], output[id], feature[id]);
        if input_bits == 0 && output_bits == 0 && feature_bits == 0 {
            continue;
        }
        report_ids.push(ReportIdDesc {
            report_id: id as u8,
            collection_number: (collection[id] as u8).wrapping_add(1),
            input_length: report_bytes(input_bits),
            output_length: report_bytes(output_bits),
            feature_length: report_bytes(feature_bits),
        });
    }

    Ok(DeviceDesc {
        collections,
        report_ids,
    })
}
