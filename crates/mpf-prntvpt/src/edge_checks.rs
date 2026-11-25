//! Edge-case tests for the ported prntvpt logic.
//!
//! These cover boundaries, every branch and the flag combinations. Expectations the
//! oracle harness did not print are marked `non-oracle`. They follow from the verbatim
//! Wine code but were not run under gcc.

use super::*;
use std::fmt::Debug;

fn metrics() -> DeviceMetrics {
    DeviceMetrics {
        logpixels_x: 300,
        logpixels_y: 300,
        physical_width: 2480,
        physical_height: 3508,
        horz_res: 2464,
        vert_res: 3484,
        physical_offset_x: 16,
        physical_offset_y: 24,
    }
}

fn ticket_with_size(width: i32, height: i32) -> Ticket {
    Ticket {
        page: Page {
            media: Media {
                size: Size { width, height },
                ..Media::default()
            },
            ..Page::default()
        },
        ..Ticket::default()
    }
}

/// Exercises the derived `Default`, `Clone`, `PartialEq` and `Debug` of `T`.
fn exercise_derives<T: Debug + Clone + PartialEq + Default>() {
    exercise_value(T::default());
}

fn exercise_value<T: Debug + Clone + PartialEq>(value: T) {
    assert_eq!(value.clone(), value);
    assert!(!format!("{value:?}").is_empty());
}

#[test]
fn derived_impls_are_exercised() {
    // non-oracle: derive coverage only; nested fields are reached through the parents
    exercise_derives::<Size>();
    exercise_derives::<Media>();
    exercise_derives::<Resolution>();
    exercise_derives::<Page>();
    exercise_derives::<Document>();
    exercise_derives::<Job>();
    exercise_derives::<Ticket>();
    exercise_derives::<Devmode>();
    exercise_derives::<DeviceMetrics>();
    exercise_derives::<ImageableArea>();
    // non-oracle: PaperOption has no Default, so its derives are called on a value
    exercise_value(PaperOption {
        media: "psk:ISOA4",
        display_name: "A4",
        width: 1,
        height: 2,
    });
}

#[test]
fn media_table_round_trips() {
    // non-oracle: every psk_media entry in both directions
    let table = [
        ("psk:NorthAmericaLetter", 1),
        ("psk:NorthAmericaLegal", 5),
        ("psk:NorthAmericaExecutive", 7),
        ("psk:ISOA3", 8),
        ("psk:ISOA4", 9),
        ("psk:ISOA5", 11),
        ("psk:JISB4", 12),
        ("psk:JISB5", 13),
    ];
    for (name, paper) in table {
        assert_eq!(media_to_paper(name), paper);
        assert_eq!(paper_to_media(paper), Some(name));
    }
}

#[test]
fn media_lookups_outside_table() {
    // non-oracle: empty, differently cased and padded names fall back to A4
    assert_eq!(media_to_paper(""), DMPAPER_A4);
    assert_eq!(media_to_paper("psk:isoa4"), DMPAPER_A4);
    assert_eq!(media_to_paper("psk:ISOA4 "), DMPAPER_A4);
    // non-oracle: codes outside the table have no media name
    assert_eq!(paper_to_media(0), None);
    assert_eq!(paper_to_media(-1), None);
    assert_eq!(paper_to_media(DMPAPER_A4 + 1), None);
}

#[test]
fn node_names_match_exactly() {
    // non-oracle: psf:ParameterInit is accepted; row 05 covers psf:Feature
    assert!(is_valid_node_name("psf:ParameterInit"));
    assert!(!is_valid_node_name("psf:feature"));
    assert!(!is_valid_node_name("psf:Feature "));
    assert!(!is_valid_node_name("psk:Feature"));
    assert!(!is_valid_node_name(""));
}

#[test]
fn option_name_fallbacks() {
    // non-oracle: explicit monochrome name, and the empty name falling back
    assert_eq!(
        page_output_color_from_option("psk:Monochrome"),
        DMCOLOR_MONOCHROME
    );
    assert_eq!(page_output_color_from_option(""), DMCOLOR_MONOCHROME);
    // non-oracle: portrait is the explicit name; unknown names fall back to portrait
    assert_eq!(
        page_orientation_from_option("psk:Portrait"),
        DMORIENT_PORTRAIT
    );
    assert_eq!(
        page_orientation_from_option("psk:Unknown"),
        DMORIENT_PORTRAIT
    );
    // non-oracle: uncollated is the explicit name; unknown names fall back to uncollated
    assert_eq!(
        document_collate_from_option("psk:Uncollated"),
        DMCOLLATE_FALSE
    );
    assert_eq!(document_collate_from_option("psk:Unknown"), DMCOLLATE_FALSE);
}

#[test]
fn scaling_from_option_branches() {
    // non-oracle: no psf:Option, parameter present: the parameter is used
    assert_eq!(page_scaling_from_option(None, Some(150)), 150);
    // non-oracle: no option and no parameter gives 100
    assert_eq!(page_scaling_from_option(None, None), 100);
    // non-oracle: psk:None ignores the parameter, as Wine returns before reading it
    assert_eq!(page_scaling_from_option(Some("psk:None"), Some(150)), 100);
    // non-oracle: an unknown option takes the parameter
    assert_eq!(page_scaling_from_option(Some("psk:Unknown"), Some(42)), 42);
    // non-oracle: negative parameters are kept; only a final 0 becomes 100
    assert_eq!(
        page_scaling_from_option(Some("psk:CustomSquare"), Some(-5)),
        -5
    );
    // non-oracle: psk:CustomSquare without its ParameterInit
    assert_eq!(
        page_scaling_from_option(Some("psk:CustomSquare"), None),
        100
    );
}

#[test]
fn option_writer_fallbacks() {
    // non-oracle: any colour other than DMCOLOR_COLOR is written as monochrome
    assert_eq!(page_output_color_option(0), "psk:Monochrome");
    // non-oracle: any orientation other than portrait is written as landscape
    assert_eq!(page_orientation_option(0), "psk:Landscape");
    // non-oracle: any collate flag other than DMCOLLATE_TRUE is written as uncollated
    assert_eq!(document_collate_option(2), "psk:Uncollated");
    // non-oracle: only 100 is psk:None; other scales, including 0 and negatives, are custom
    assert_eq!(page_scaling_option(100), "psk:None");
    assert_eq!(page_scaling_option(0), "psk:CustomSquare");
    assert_eq!(page_scaling_option(-5), "psk:CustomSquare");
}

#[test]
fn mul_div_zero_divisor() {
    // non-oracle: c == 0 for any operands; row 19 covers 7 * 25400 / 0
    assert_eq!(mul_div(5, 5, 0), -1);
    assert_eq!(mul_div(0, 0, 0), -1);
}

#[test]
fn mul_div_rounds_half_away_from_zero() {
    // non-oracle: half-way quotients for all four sign combinations
    assert_eq!(mul_div(1, 1, 2), 1);
    assert_eq!(mul_div(-1, 1, 2), -1);
    assert_eq!(mul_div(1, -1, 2), -1);
    assert_eq!(mul_div(-1, -1, 2), 1);
    // non-oracle: a negative divisor negates both operands first
    assert_eq!(mul_div(3, 1, -2), -2);
    assert_eq!(mul_div(3, -1, -2), 2);
    // non-oracle: zero product
    assert_eq!(mul_div(0, -5, 3), 0);
    // non-oracle: below and above one half
    assert_eq!(mul_div(1, 1, 3), 0);
    assert_eq!(mul_div(2, 1, 3), 1);
}

#[test]
fn mul_div_range_boundaries() {
    // non-oracle: i32::MAX and -i32::MAX are in range
    assert_eq!(mul_div(i32::MAX, 1, 1), i32::MAX);
    assert_eq!(mul_div(-i32::MAX, 1, 1), -i32::MAX);
    // non-oracle: i32::MIN is out of range, since the C check is ret < -2147483647
    assert_eq!(mul_div(i32::MIN, 1, 1), -1);
    // non-oracle: 2 * MAX / 2 is exactly MAX, so it stays in range
    assert_eq!(mul_div(i32::MAX, 2, 2), i32::MAX);
    // non-oracle: 2 * MAX / 1 and -2 * MAX / 1 overflow
    assert_eq!(mul_div(i32::MAX, 2, 1), -1);
    assert_eq!(mul_div(i32::MAX, -2, 1), -1);
}

#[test]
fn mul_div_int_min_inputs() {
    // non-oracle: the C code negates INT_MIN, which is undefined; the i64 port gives the exact quotient
    assert_eq!(mul_div(i32::MIN, -1, i32::MIN), -1);
    // non-oracle: 1 / INT_MIN is about -4.7e-10 and rounds to 0
    assert_eq!(mul_div(1, 1, i32::MIN), 0);
}

#[test]
fn pixels_to_mm_edges() {
    // non-oracle: zero pixels are zero millimetres
    assert_eq!(pixels_to_mm_x1000(0, 300), 0);
    // non-oracle: one inch is 25400 thousandths of a millimetre at any dpi
    assert_eq!(pixels_to_mm_x1000(600, 600), 25400);
    assert_eq!(pixels_to_mm_x1000(96, 96), 25400);
    // non-oracle: a negative dpi negates the divisor
    assert_eq!(pixels_to_mm_x1000(300, -300), -25400);
}

#[test]
fn imageable_area_orientation_branches() {
    // non-oracle: only portrait keeps the device axes; any other value takes the landscape branch
    let landscape = imageable_area(&metrics(), DMORIENT_LANDSCAPE);
    assert_eq!(imageable_area(&metrics(), 0), landscape);
    assert_eq!(imageable_area(&metrics(), 3), landscape);
}

#[test]
fn imageable_area_zero_dpi() {
    // non-oracle: zero logical pixels per inch makes every scaled value -1, MulDiv's error result
    assert_eq!(
        imageable_area(&DeviceMetrics::default(), DMORIENT_PORTRAIT),
        ImageableArea {
            imageable_size_width: -1,
            imageable_size_height: -1,
            origin_width: -1,
            origin_height: -1,
            extent_width: -1,
            extent_height: -1,
        }
    );
}

#[test]
fn paper_size_options_edges() {
    // non-oracle: empty input gives no options
    assert_eq!(paper_size_options(&[], &[], &[]), Vec::new());
    // non-oracle: only unknown paper codes give no options
    assert_eq!(paper_size_options(&[260], &[(1, 2)], &["x"]), Vec::new());
    // non-oracle: the lists are zipped to the shortest; the second entry is dropped
    assert_eq!(
        paper_size_options(&[1, 9], &[(2159, 2794)], &["Letter", "A4"]),
        vec![PaperOption {
            media: "psk:NorthAmericaLetter",
            display_name: "Letter",
            width: 215900,
            height: 279400,
        }]
    );
    // non-oracle: the 100x scale wraps; C leaves this undefined, and oracle row 24 is excluded
    let options = paper_size_options(&[9], &[(30_000_000, 2970)], &["A4"]);
    assert_eq!(options[0].width, -1_294_967_296);
    assert_eq!(options[0].height, 297_000);
}

#[test]
fn selection_namespaces_filter() {
    // non-oracle: an empty attribute list gives an empty string
    assert_eq!(selection_namespaces(&[]), "");
    // non-oracle: a bare xmlns attribute
    assert_eq!(
        selection_namespaces(&[("xmlns", "urn:x")]),
        "xmlns=\"urn:x\" "
    );
    // non-oracle: the match is exact and case-sensitive; only "xmlns:" with a colon counts
    assert_eq!(
        selection_namespaces(&[("xmlnsfoo", "v"), ("XMLNS", "v"), ("xmlns:", "v")]),
        "xmlns:=\"v\" "
    );
    // non-oracle: attribute order is kept
    assert_eq!(
        selection_namespaces(&[("xmlns:b", "2"), ("xmlns:a", "1")]),
        "xmlns:b=\"2\" xmlns:a=\"1\" "
    );
}

#[test]
fn devmode_to_ticket_no_flags_leaves_ticket_alone() {
    // non-oracle: with no DM_* flags set, every field of the ticket is left unchanged
    let dm = Devmode {
        fields: 0,
        orientation: 2,
        paper_size: 9,
        paper_length: 1,
        paper_width: 1,
        scale: 50,
        copies: 3,
        default_source: 4,
        print_quality: 300,
        color: 1,
        y_resolution: 200,
        collate: 1,
        log_pixels: 96,
    };
    let before = Ticket {
        job: Job {
            copies: 9,
            input_bin: 9,
        },
        document: Document { collate: 9 },
        page: Page {
            media: Media {
                paper: 9,
                size: Size {
                    width: 9,
                    height: 9,
                },
            },
            resolution: Resolution { x: 9, y: 9 },
            orientation: 9,
            scaling: 9,
            color: 9,
        },
    };
    let mut ticket = before;
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(ticket, before);
}

#[test]
fn devmode_to_ticket_every_flag() {
    // non-oracle: all flags set; DM_LOGPIXELS comes last, so it sets both resolution axes
    let dm = Devmode {
        fields: DM_ORIENTATION
            | DM_PAPERSIZE
            | DM_PAPERLENGTH
            | DM_PAPERWIDTH
            | DM_SCALE
            | DM_COPIES
            | DM_DEFAULTSOURCE
            | DM_COLOR
            | DM_PRINTQUALITY
            | DM_YRESOLUTION
            | DM_COLLATE
            | DM_LOGPIXELS,
        orientation: 2,
        paper_size: 9,
        paper_length: 297,
        paper_width: 210,
        scale: 90,
        copies: 3,
        default_source: 7,
        print_quality: 600,
        color: 2,
        y_resolution: 300,
        collate: 1,
        log_pixels: 72,
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(
        ticket,
        Ticket {
            job: Job {
                copies: 3,
                input_bin: 7,
            },
            document: Document { collate: 1 },
            page: Page {
                media: Media {
                    paper: 9,
                    size: Size {
                        width: 21000,
                        height: 29700,
                    },
                },
                resolution: Resolution { x: 72, y: 72 },
                orientation: 2,
                scaling: 90,
                color: 2,
            },
        }
    );
}

#[test]
fn devmode_resolution_flags_alone() {
    // non-oracle: DM_YRESOLUTION alone sets only y
    let dm = Devmode {
        fields: DM_YRESOLUTION,
        y_resolution: 300,
        ..Devmode::default()
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(ticket.page.resolution, Resolution { x: 0, y: 300 });

    // non-oracle: DM_PRINTQUALITY alone sets both axes
    let dm = Devmode {
        fields: DM_PRINTQUALITY,
        print_quality: 600,
        ..Devmode::default()
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(ticket.page.resolution, Resolution { x: 600, y: 600 });
}

#[test]
fn devmode_paper_size_extremes() {
    // non-oracle: the 16-bit paper sizes widen before the *100, so both extremes fit
    let dm = Devmode {
        fields: DM_PAPERWIDTH | DM_PAPERLENGTH,
        paper_width: i16::MAX,
        paper_length: i16::MIN,
        ..Devmode::default()
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(
        ticket.page.media.size,
        Size {
            width: 3_276_700,
            height: -3_276_800,
        }
    );
}

#[test]
fn ticket_to_devmode_defaults_and_truncation() {
    // non-oracle: a default ticket sets the eleven mapped flags and leaves values and dmLogPixels zero
    assert_eq!(
        ticket_to_devmode(&Ticket::default()),
        Devmode {
            fields: 0xAF1F,
            ..Devmode::default()
        }
    );
    // non-oracle: division by 100 truncates toward zero, so -150 gives -1 and 99 gives 0
    let dm = ticket_to_devmode(&ticket_with_size(-150, 99));
    assert_eq!((dm.paper_width, dm.paper_length), (-1, 0));
    // non-oracle: copies are truncated to 16 bits, so 40000 wraps to -25536
    let ticket = Ticket {
        job: Job {
            copies: 40_000,
            ..Job::default()
        },
        ..Ticket::default()
    };
    assert_eq!(ticket_to_devmode(&ticket).copies, -25_536);
}

#[test]
fn ticket_devmode_round_trip() {
    // non-oracle: every field of a ticket with 100-multiple sizes survives ticket -> devmode -> ticket
    let ticket = Ticket {
        job: Job {
            copies: 2,
            input_bin: DMBIN_AUTO,
        },
        document: Document {
            collate: DMCOLLATE_TRUE,
        },
        page: Page {
            media: Media {
                paper: DMPAPER_A4,
                size: Size {
                    width: 21000,
                    height: 29700,
                },
            },
            resolution: Resolution { x: 600, y: 300 },
            orientation: DMORIENT_PORTRAIT,
            scaling: 100,
            color: DMCOLOR_MONOCHROME,
        },
    };
    let mut back = Ticket::default();
    devmode_to_ticket(&ticket_to_devmode(&ticket), &mut back);
    assert_eq!(back, ticket);
}
