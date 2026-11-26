/// One `Default` element already recorded for the package: the C's `struct default_type`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefaultType<'a> {
    pub ext: &'a [u16],
    pub content_type: &'a [u16],
}

/// What `opc_package_add_content_type` does for one part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentTypeAction {
    /// A `Default` for the extension already has this content type.
    Nothing,
    /// No `Default` covers the extension, so one is added.
    AddDefault,
    /// The part has no extension, or the extension maps to another content type,
    /// so an `Override` is added.
    AddOverride,
}

/// `opc_package_add_content_type`, decision only.
///
/// `defaults` are the `Default` elements in insertion order. Overrides are not
/// part of the list, since the C skips them. `ext` is `None` when the part name
/// has no extension. The first default whose extension matches decides, as the
/// C loop breaks there. Extensions compare with `wcsicmp`, which the port folds
/// for ASCII only; content types compare exactly.
pub fn classify_content_type(
    defaults: &[DefaultType<'_>],
    ext: Option<&[u16]>,
    content_type: &[u16],
) -> ContentTypeAction {
    let Some(ext) = ext else {
        return ContentTypeAction::AddOverride;
    };
    match defaults
        .iter()
        .find(|default| eq_ignore_ascii_case(default.ext, ext))
    {
        Some(default) if default.content_type == content_type => ContentTypeAction::Nothing,
        Some(_) => ContentTypeAction::AddOverride,
        None => ContentTypeAction::AddDefault,
    }
}

fn eq_ignore_ascii_case(left: &[u16], right: &[u16]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(&a, &b)| fold_ascii(a) == fold_ascii(b))
}

fn fold_ascii(unit: u16) -> u16 {
    match u8::try_from(unit) {
        Ok(byte) => u16::from(byte.to_ascii_lowercase()),
        Err(_) => unit,
    }
}
