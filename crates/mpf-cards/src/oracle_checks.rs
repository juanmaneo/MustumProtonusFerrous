//! Replays the commands of the Wine oracle (`target/canon/cards`) against this crate.
//! The vectors are the verbatim output of the deterministic logic in `cdtDrawExt`
//! (argument checks, MODEFLAG_DONT_ROUND_CORNERS handling, and the mode state machine)
//! compiled with `gcc -fsanitize=address,undefined -g` and run clean under both.

use super::*;

/// One oracle row: input arguments and expected output.
struct Vector {
    name: &'static str,
    card: i32,
    mode: i32,
    dx: i32,
    dy: i32,
    card_width: i32,
    card_height: i32,
    expected: Result<DrawArgs, DrawError>,
}

const VECTORS: &[Vector] = &[
    // Mode 0 (FACEUP), card 0, dx=card_width, dy=card_height → round_corners=true
    Vector {
        name: "faceup card0 default",
        card: 0,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Mode 1 (FACEDOWN), card 0
    Vector {
        name: "facedown card0",
        card: 0,
        mode: 1,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Mode 2 (HILITE), card 0 → raster_op = NOTSRCCOPY
    Vector {
        name: "hilite card0",
        card: 0,
        mode: 2,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: NOTSRCCOPY,
            round_corners: true,
        }),
    },
    // Mode 3 (GHOST), card 0 → card = CARD_FREE_MASK, erase=true, raster=SRCAND
    Vector {
        name: "ghost card0",
        card: 0,
        mode: 3,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_FREE_MASK,
            erase_flag: true,
            draw_flag: true,
            raster_op: SRCAND,
            round_corners: true,
        }),
    },
    // Mode 4 (REMOVE), card 0 → erase=true, draw=false
    Vector {
        name: "remove card0",
        card: 0,
        mode: 4,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: true,
            draw_flag: false,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Mode 5 (INVISIBLEGHOST), card 0 → card = CARD_FREE_MASK, raster=SRCAND
    Vector {
        name: "invisibleghost card0",
        card: 0,
        mode: 5,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_FREE_MASK,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCAND,
            round_corners: true,
        }),
    },
    // Mode 6 (DECKX), card 0 → card = CARD_BACK_THE_X
    Vector {
        name: "deckx card0",
        card: 0,
        mode: 6,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_BACK_THE_X,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Mode 7 (DECKO), card 0 → card = CARD_BACK_THE_O
    Vector {
        name: "decko card0",
        card: 0,
        mode: 7,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_BACK_THE_O,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card 51 (last face), mode 0
    Vector {
        name: "faceup card51",
        card: 51,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 51,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card 52 (ghost mask), mode 0
    Vector {
        name: "faceup card52",
        card: 52,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 52,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card 53 (first back), mode 0
    Vector {
        name: "faceup card53",
        card: 53,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 53,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card 67 (THE_X), mode 0
    Vector {
        name: "faceup card67",
        card: 67,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 67,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card 68 (THE_O), mode 0
    Vector {
        name: "faceup card68",
        card: 68,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 68,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // Card -1 → InvalidCard
    Vector {
        name: "card -1",
        card: -1,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Err(DrawError::InvalidCard),
    },
    // Card 69 → InvalidCard
    Vector {
        name: "card 69",
        card: 69,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Err(DrawError::InvalidCard),
    },
    // Mode 8 → InvalidMode
    Vector {
        name: "mode 8",
        card: 0,
        mode: 8,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Err(DrawError::InvalidMode),
    },
    // Mode -1 → InvalidMode
    Vector {
        name: "mode -1",
        card: 0,
        mode: -1,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Err(DrawError::InvalidMode),
    },
    // dx != card_width → round_corners = false
    Vector {
        name: "dx mismatch",
        card: 0,
        mode: 0,
        dx: 70,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // dy != card_height → round_corners = false
    Vector {
        name: "dy mismatch",
        card: 0,
        mode: 0,
        dx: 71,
        dy: 95,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // card_width mismatch → round_corners = false
    Vector {
        name: "card_width mismatch",
        card: 0,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 70,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // card_height mismatch → round_corners = false
    Vector {
        name: "card_height mismatch",
        card: 0,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 95,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000000) → round_corners = false, mode = 0
    Vector {
        name: "dont_round_corners mode0",
        card: 0,
        mode: -2147483648,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000001) → round_corners = false, mode = 1
    Vector {
        name: "dont_round_corners mode1",
        card: 0,
        mode: -2147483647,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000002) → round_corners = false, mode = 2
    Vector {
        name: "dont_round_corners mode2",
        card: 51,
        mode: -2147483646,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 51,
            erase_flag: false,
            draw_flag: true,
            raster_op: NOTSRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000003) → round_corners = false, mode = 3
    Vector {
        name: "dont_round_corners mode3",
        card: 0,
        mode: -2147483645,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_FREE_MASK,
            erase_flag: true,
            draw_flag: true,
            raster_op: SRCAND,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000004) → round_corners = false, mode = 4
    Vector {
        name: "dont_round_corners mode4",
        card: 0,
        mode: -2147483644,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: true,
            draw_flag: false,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000005) → round_corners = false, mode = 5
    Vector {
        name: "dont_round_corners mode5",
        card: 0,
        mode: -2147483643,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_FREE_MASK,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCAND,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000006) → round_corners = false, mode = 6
    Vector {
        name: "dont_round_corners mode6",
        card: 0,
        mode: -2147483642,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_BACK_THE_X,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // MODEFLAG_DONT_ROUND_CORNERS set (mode = 0x80000007) → round_corners = false, mode = 7
    Vector {
        name: "dont_round_corners mode7",
        card: 0,
        mode: -2147483641,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: CARD_BACK_THE_O,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: false,
        }),
    },
    // dx=dy=card_width=card_height → round_corners = true
    Vector {
        name: "round_corners true",
        card: 0,
        mode: 0,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: SRCCOPY,
            round_corners: true,
        }),
    },
    // dx=dy=card_width=card_height, mode=2 (HILITE) → round_corners = true
    Vector {
        name: "round_corners true hilite",
        card: 0,
        mode: 2,
        dx: 71,
        dy: 96,
        card_width: 71,
        card_height: 96,
        expected: Ok(DrawArgs {
            card: 0,
            erase_flag: false,
            draw_flag: true,
            raster_op: NOTSRCCOPY,
            round_corners: true,
        }),
    },
];

#[test]
fn oracle_vectors_match_wine() {
    for v in VECTORS {
        let result = resolve_draw_args(v.card, v.mode, v.dx, v.dy, v.card_width, v.card_height);
        assert_eq!(result, v.expected, "{}", v.name);
    }
}
