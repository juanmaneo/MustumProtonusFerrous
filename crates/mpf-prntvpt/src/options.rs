use crate::{
    DMCOLLATE_FALSE, DMCOLLATE_TRUE, DMCOLOR_COLOR, DMCOLOR_MONOCHROME, DMORIENT_LANDSCAPE,
    DMORIENT_PORTRAIT,
};

/// `is_valid_node_name`: the two `psf:` node names the PrintTicket reader accepts.
pub fn is_valid_node_name(name: &str) -> bool {
    matches!(name, "psf:ParameterInit" | "psf:Feature")
}

/// The colour mode for a `psk:PageOutputColor` option name, as `read_PageOutputColor`
/// maps it. Unknown names give monochrome (Wine logs a FIXME, which is not ported).
pub fn page_output_color_from_option(name: &str) -> i32 {
    match name {
        "psk:Color" => DMCOLOR_COLOR,
        _ => DMCOLOR_MONOCHROME,
    }
}

/// The orientation for a `psk:PageOrientation` option name. Unknown names give portrait.
pub fn page_orientation_from_option(name: &str) -> i32 {
    match name {
        "psk:Landscape" => DMORIENT_LANDSCAPE,
        _ => DMORIENT_PORTRAIT,
    }
}

/// The collate flag for a `psk:DocumentCollate` option name. Unknown names give uncollated.
pub fn document_collate_from_option(name: &str) -> i32 {
    match name {
        "psk:Collated" => DMCOLLATE_TRUE,
        _ => DMCOLLATE_FALSE,
    }
}

/// `read_PageScaling`. `option` is the name of the `psf:Option`, or `None` when the
/// feature has none. `param` is the `psk:PageScalingScale` value, or `None` when that
/// ParameterInit is missing or not an integer. `psk:None` gives 100 and ignores `param`.
/// Any other option takes `param` (0 when absent). A final 0 becomes 100, but negative
/// values are kept.
pub fn page_scaling_from_option(option: Option<&str>, param: Option<i32>) -> i32 {
    let scaling = match option {
        Some("psk:None") => 100,
        _ => param.unwrap_or(0),
    };
    if scaling == 0 {
        100
    } else {
        scaling
    }
}

/// The option name `write_PageOutputColor` writes for a colour mode.
pub fn page_output_color_option(color: i32) -> &'static str {
    if color == DMCOLOR_COLOR {
        "psk:Color"
    } else {
        "psk:Monochrome"
    }
}

/// The option name `write_PageOrientation` writes for an orientation.
pub fn page_orientation_option(orientation: i32) -> &'static str {
    if orientation == DMORIENT_PORTRAIT {
        "psk:Portrait"
    } else {
        "psk:Landscape"
    }
}

/// The option name `write_DocumentCollate` writes for a collate flag.
pub fn document_collate_option(collate: i32) -> &'static str {
    if collate == DMCOLLATE_TRUE {
        "psk:Collated"
    } else {
        "psk:Uncollated"
    }
}

/// The option name `write_PageScaling` writes. Only 100 maps to `psk:None`; any other
/// scale is `psk:CustomSquare`, whose `psk:Scale` property is DOM output and not ported.
pub fn page_scaling_option(scaling: i32) -> &'static str {
    if scaling == 100 {
        "psk:None"
    } else {
        "psk:CustomSquare"
    }
}
