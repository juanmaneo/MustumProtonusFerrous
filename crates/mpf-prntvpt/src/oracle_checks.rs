//! Oracle-derived test vectors for the ported prntvpt logic.
//!
//! Vectors were captured by compiling the verbatim Wine C bodies from
//! `dlls/prntvpt/ticket.c` and the `MulDiv` helper from `dlls/kernelbase/main.c`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/prntvpt/`, then
//! running the harness. It prints 25 rows. Row 24 (`paper_options_overflow`) is
//! excluded: UBSan reports signed overflow in `pt[i].x * 100`, so 24 rows remain.
//! Row numbers are given per assertion.

use super::*;

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

#[test]
fn oracle_media_rows() {
    // row 01
    assert_eq!(media_to_paper("psk:NorthAmericaLetter"), 1);
    // row 02: unknown name falls back to A4
    assert_eq!(media_to_paper("psk:Bogus"), 9);
    // row 03
    assert_eq!(paper_to_media(5), Some("psk:NorthAmericaLegal"));
    // row 04
    assert_eq!(paper_to_media(260), None);
}

#[test]
fn oracle_option_rows() {
    // row 05
    assert!(is_valid_node_name("psf:Feature"));
    // row 06
    assert_eq!(page_output_color_from_option("psk:Color"), DMCOLOR_COLOR);
    // row 07: unknown name falls back to monochrome
    assert_eq!(page_output_color_from_option("psk:Foo"), DMCOLOR_MONOCHROME);
    // row 08
    assert_eq!(
        page_orientation_from_option("psk:Landscape"),
        DMORIENT_LANDSCAPE
    );
    // row 09: option writers, colour then orientation then collate
    assert_eq!(page_output_color_option(DMCOLOR_COLOR), "psk:Color");
    assert_eq!(
        page_output_color_option(DMCOLOR_MONOCHROME),
        "psk:Monochrome"
    );
    assert_eq!(page_orientation_option(DMORIENT_PORTRAIT), "psk:Portrait");
    assert_eq!(page_orientation_option(DMORIENT_LANDSCAPE), "psk:Landscape");
    assert_eq!(document_collate_option(DMCOLLATE_TRUE), "psk:Collated");
    assert_eq!(document_collate_option(DMCOLLATE_FALSE), "psk:Uncollated");
    // row 10
    assert_eq!(document_collate_from_option("psk:Collated"), DMCOLLATE_TRUE);
    // row 11: psk:None gives 100
    assert_eq!(page_scaling_from_option(Some("psk:None"), None), 100);
    // row 12: psk:CustomSquare takes the parameter
    assert_eq!(
        page_scaling_from_option(Some("psk:CustomSquare"), Some(150)),
        150
    );
    // row 13: a zero parameter becomes 100
    assert_eq!(
        page_scaling_from_option(Some("psk:CustomSquare"), Some(0)),
        100
    );
    // row 14
    assert_eq!(page_scaling_option(150), "psk:CustomSquare");
}

#[test]
fn oracle_devmode_rows() {
    // row 15: DM_PAPERWIDTH | DM_PRINTQUALITY | DM_YRESOLUTION; y ends at 300
    let dm = Devmode {
        fields: DM_PAPERWIDTH | DM_PRINTQUALITY | DM_YRESOLUTION,
        paper_width: 210,
        print_quality: 600,
        y_resolution: 300,
        ..Devmode::default()
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(
        ticket,
        Ticket {
            page: Page {
                media: Media {
                    paper: 0,
                    size: Size {
                        width: 21000,
                        height: 0,
                    },
                },
                resolution: Resolution { x: 600, y: 300 },
                ..Page::default()
            },
            ..Ticket::default()
        }
    );

    // row 16: DM_LOGPIXELS overrides both resolution axes
    let dm = Devmode {
        fields: DM_PRINTQUALITY | DM_YRESOLUTION | DM_LOGPIXELS,
        log_pixels: 96,
        ..Devmode::default()
    };
    let mut ticket = Ticket::default();
    devmode_to_ticket(&dm, &mut ticket);
    assert_eq!(
        ticket,
        Ticket {
            page: Page {
                resolution: Resolution { x: 96, y: 96 },
                ..Page::default()
            },
            ..Ticket::default()
        }
    );

    // row 17: a 5000000 width divides to 50000 and wraps to -15536 as a SHORT
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
                    width: 5_000_000,
                    height: 297_000,
                },
            },
            resolution: Resolution { x: 600, y: 600 },
            orientation: DMORIENT_LANDSCAPE,
            scaling: 100,
            color: DMCOLOR_COLOR,
        },
    };
    assert_eq!(
        ticket_to_devmode(&ticket),
        Devmode {
            fields: 0xAF1F,
            orientation: 2,
            paper_size: 9,
            paper_length: 2970,
            paper_width: -15536,
            scale: 100,
            copies: 2,
            default_source: 7,
            print_quality: 600,
            color: 2,
            y_resolution: 600,
            collate: 1,
            log_pixels: 0,
        }
    );
}

#[test]
fn oracle_caps_rows() {
    // row 18
    assert_eq!(pixels_to_mm_x1000(300, 300), 25400);
    // row 19: zero dpi
    assert_eq!(pixels_to_mm_x1000(7, 0), -1);
    // row 20: 25400 / 3 = 8466.67 rounds up
    assert_eq!(pixels_to_mm_x1000(1, 3), 8467);
    // row 21
    assert_eq!(
        imageable_area(&metrics(), DMORIENT_PORTRAIT),
        ImageableArea {
            imageable_size_width: 209973,
            imageable_size_height: 297011,
            origin_width: 1355,
            origin_height: 2032,
            extent_width: 208619,
            extent_height: 294979,
        }
    );
    // row 22
    assert_eq!(
        imageable_area(&metrics(), DMORIENT_LANDSCAPE),
        ImageableArea {
            imageable_size_width: 297011,
            imageable_size_height: 209973,
            origin_width: 2032,
            origin_height: 1355,
            extent_width: 294979,
            extent_height: 208619,
        }
    );
    // row 23: the unknown paper code 260 is skipped
    let papers: [u16; 3] = [9, 260, 1];
    let sizes = [(2100, 2970), (1, 2), (2159, 2794)];
    let names = ["A4 Sheet", "Unknown", "Letter"];
    assert_eq!(
        paper_size_options(&papers, &sizes, &names),
        vec![
            PaperOption {
                media: "psk:ISOA4",
                display_name: "A4 Sheet",
                width: 210000,
                height: 297000,
            },
            PaperOption {
                media: "psk:NorthAmericaLetter",
                display_name: "Letter",
                width: 215900,
                height: 279400,
            },
        ]
    );
}

#[test]
fn oracle_namespace_row() {
    // row 25: the plain attribute "name" is not a namespace and is dropped
    assert_eq!(
        selection_namespaces(&[
            ("xmlns:psf", "http://a"),
            ("name", "x"),
            ("xmlns", "http://d"),
            ("xmlns:xsi", "http://x"),
        ]),
        "xmlns:psf=\"http://a\" xmlns=\"http://d\" xmlns:xsi=\"http://x\" "
    );
}
