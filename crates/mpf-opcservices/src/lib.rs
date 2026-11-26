//! Port of the portable logic in Wine's `dlls/opcservices/` (`uri.c`, `package.c`
//! and `opc_private.h`).
//!
//! Ported:
//! - `opc_part_uri_get_rels_uri`, string core: [`rels_part_path`]
//! - `opc_package_add_content_type`, decision only: [`classify_content_type`],
//!   [`ContentTypeAction`], [`DefaultType`]
//! - `opc_array_reserve`, capacity policy only: [`array_reserve_capacity`]
//!
//! Skipped:
//! - `opc_part_uri_get_rels_uri` `IUri` calls (`IUri_GetPath`,
//!   `IUri_GetPropertyLength`, `CreateUri`) and its `malloc`/`free`.
//! - `opc_array_reserve` `realloc`: the port returns the capacity to request.
//! - `uri.c` `IUri` vtable methods, `opc_part_uri_create`, `opc_root_uri_create`
//!   and `opc_source_uri_create`: COM objects, refcounts and `IUri` comparison.
//! - `package.c` COM vtables, part and relationship enumerators, list
//!   plumbing, `opc_strdupW`, and the `IXmlWriter` output of `[Content_Types].xml`
//!   and `.rels` parts: COM and allocation.
//! - `compress.c`: zip writer over `IStream`, zlib deflate, `RtlComputeCrc32`
//!   and `FileTimeToDosDateTime`.
//! - `factory.c`: class factory, `DllGetClassObject` and the file-backed `IStream`.

mod content_type;
mod rels_uri;

pub use content_type::{classify_content_type, ContentTypeAction, DefaultType};
pub use rels_uri::rels_part_path;

/// `opc_array_reserve` capacity policy: the capacity the C code requests so that
/// `count` elements of `size` bytes fit, starting from `capacity`.
///
/// Returns `None` where the C returns FALSE: `count` above `usize::MAX / size`,
/// or a zero `size` that the C divides by. Realloc failure is not modelled.
/// As in the C, the minimum of 4 is not clamped to the maximum, so for
/// `size > usize::MAX / 4` the C's `new_capacity * size` wraps; the requested
/// capacity is still reported here.
pub fn array_reserve_capacity(capacity: usize, count: usize, size: usize) -> Option<usize> {
    if count <= capacity {
        return Some(capacity);
    }
    let max_capacity = usize::MAX.checked_div(size)?;
    if count > max_capacity {
        return None;
    }
    let mut new_capacity = capacity.max(4);
    while new_capacity < count && new_capacity <= max_capacity / 2 {
        new_capacity *= 2;
    }
    if new_capacity < count {
        new_capacity = max_capacity;
    }
    Some(new_capacity)
}

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
