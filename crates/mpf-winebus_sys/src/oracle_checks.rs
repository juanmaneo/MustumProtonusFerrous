//! Checks the port against rows recorded from the verbatim Wine C code
//! (`target/canon/winebus.sys/harness.c`, built with ASan and UBSan). Each row is
//! `(name, case, expected output)`. A case starts with `L` (report layout and
//! writes), `R` (report sync and drop), `E` (effect usage) or `T` (hat table), as
//! in the harness.
//!
//! Excluded: vectors that the sanitizers flag in C. These are 16-bit and 32-bit
//! axis stores at offsets that are not a multiple of their size, and hat writes
//! whose index is past the last hat byte. `edge_checks.rs` covers the rest.

use super::*;

fn run_case(line: &str) -> String {
    let mut tokens = line.split_whitespace();
    let op = tokens.next().unwrap();
    match op {
        "L" => run_layout(tokens),
        "R" => run_sync(tokens),
        "E" => run_effect(tokens),
        _ => run_table(tokens),
    }
}

fn nums(text: &str) -> Vec<i64> {
    text.split(':').map(|part| part.parse().unwrap()).collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    text.as_bytes()
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn buffer(report: &mut Option<Vec<u8>>) -> &mut [u8] {
    report
        .as_mut()
        .expect("e allocates the report before writes")
}

fn run_layout<'a>(ops: impl Iterator<Item = &'a str>) -> String {
    let usages: Vec<u16> = (0..64u16).map(|i| 0x30 + i).collect();
    let mut layout = HidDeviceLayout {
        bit_size: 8,
        ..Default::default()
    };
    let mut report: Option<Vec<u8>> = None;
    let mut items = Vec::new();
    for op in ops {
        let (kind, arg) = op.split_at(1);
        let ok = match kind {
            "b" => layout.add_button_count(arg.parse().unwrap()),
            "h" => layout.add_hatswitch_count(arg.parse().unwrap()),
            "a" | "r" => {
                let (size, count) = arg.split_once('x').unwrap();
                layout.add_axis_count(
                    kind == "r",
                    size.parse().unwrap(),
                    count.parse().unwrap(),
                    1,
                    &usages,
                )
            }
            "e" => {
                report = Some(vec![0; (layout.bit_size as usize).div_ceil(8)]);
                true
            }
            "X" => {
                let v = nums(arg);
                layout.set_abs_axis(buffer(&mut report), v[0] as u32, v[1] as i32)
            }
            "Y" => {
                let v = nums(arg);
                layout.set_rel_axis(buffer(&mut report), v[0] as u32, v[1] as i32)
            }
            "B" => {
                let v = nums(arg);
                layout.set_button(buffer(&mut report), v[0] as u32, v[1] != 0)
            }
            "x" => {
                let v = nums(arg);
                layout.set_hatswitch_x(buffer(&mut report), v[0] as u32, v[1] as i32)
            }
            "y" => {
                let v = nums(arg);
                layout.set_hatswitch_y(buffer(&mut report), v[0] as u32, v[1] as i32)
            }
            "m" => {
                let v = nums(arg);
                layout.move_hatswitch(buffer(&mut report), v[0] as u32, v[1] as i32, v[2] as i32)
            }
            _ => false,
        };
        items.push(if ok { "ok" } else { "no" });
    }

    let axes: Vec<String> = (0..usize::from(layout.abs_axis_count + layout.rel_axis_count))
        .map(|i| format!("{}:{}", layout.axis_byte_offsets[i], layout.axis_sizes[i]))
        .collect();
    let us: Vec<String> = (0..usize::from(layout.abs_axis_count))
        .map(|i| layout.abs_axis_usages[i].usage.to_string())
        .collect();
    let rep = match &report {
        Some(buf) => hex(buf),
        None => "-".to_string(),
    };
    format!(
        "{} | bit={} abs={} rel={} hat={}/{} btn={}/{} axes={} us={} rep={}",
        items.join(" "),
        layout.bit_size,
        layout.abs_axis_count,
        layout.rel_axis_count,
        layout.hatswitch_start,
        layout.hatswitch_count,
        layout.button_start,
        layout.button_count,
        axes.join(","),
        us.join(","),
        rep
    )
}

fn run_sync<'a>(mut ops: impl Iterator<Item = &'a str>) -> String {
    let mut report = unhex(ops.next().unwrap());
    let mut last = unhex(ops.next().unwrap());
    let mut dropped = false;
    let mut items = Vec::new();
    for op in ops {
        if op == "d" {
            drop_report(&mut dropped);
            items.push("drop");
        } else {
            items.push(if sync_report(&mut report, &mut last, &mut dropped) {
                "ok"
            } else {
                "no"
            });
        }
    }
    format!(
        "{} | rep={} last={}",
        items.join(" "),
        hex(&report),
        hex(&last)
    )
}

fn run_effect<'a>(ops: impl Iterator<Item = &'a str>) -> String {
    let items: Vec<String> = ops
        .map(|tok| {
            let usage = u16::from_str_radix(tok, 16).unwrap();
            match ff_effect_type_from_usage(usage) {
                None => "unsupported".to_string(),
                Some(effect) => match effect.waveform {
                    Some(waveform) => format!("T{:x}:{:x}", effect.effect_type, waveform),
                    None => format!("T{:x}", effect.effect_type),
                },
            }
        })
        .collect();
    items.join(" ")
}

fn run_table<'a>(ops: impl Iterator<Item = &'a str>) -> String {
    let items: Vec<String> = ops
        .map(|tok| {
            let (kind, arg) = tok.split_at(1);
            let v = nums(arg);
            if kind == "d" {
                let (x, y) = hatswitch_decompose(v[0] as u8, v[1] as u32);
                format!("{x}:{y}")
            } else {
                let mut value = v[3] as u8;
                hatswitch_compose(v[0] as i32, v[1] as i32, &mut value, v[2] as u32);
                value.to_string()
            }
        })
        .collect();
    items.join(" ")
}
#[rustfmt::skip]
const ROWS: &[(&str, &str, &str)] = &[
    ("l_axes_then_buttons", "L a8x2 b8 end", "ok ok ok | bit=32 abs=2 rel=0 hat=0/0 btn=3/8 axes=1:8,2:8 us=48,49 rep=00000000"),
    ("l_axis_size_rejected", "L a12x1 end", "no ok | bit=8 abs=0 rel=0 hat=0/0 btn=0/0 axes= us= rep=00"),
    ("l_axis_count_limit", "L a8x255 end", "no ok | bit=8 abs=0 rel=0 hat=0/0 btn=0/0 axes= us= rep=00"),
    ("l_abs_usage_limit", "L a8x32 a8x1 end", "ok no ok | bit=264 abs=32 rel=0 hat=0/0 btn=0/0 axes=1:8,2:8,3:8,4:8,5:8,6:8,7:8,8:8,9:8,10:8,11:8,12:8,13:8,14:8,15:8,16:8,17:8,18:8,19:8,20:8,21:8,22:8,23:8,24:8,25:8,26:8,27:8,28:8,29:8,30:8,31:8,32:8 us=48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79 rep=000000000000000000000000000000000000000000000000000000000000000000"),
    ("l_rel_then_abs_rejected", "L r8x1 a8x1 end", "ok no ok | bit=16 abs=0 rel=1 hat=0/0 btn=0/0 axes=1:8 us= rep=0000"),
    ("l_axis_after_buttons_rejected", "L b4 a8x1 end", "ok no ok | bit=12 abs=0 rel=0 hat=0/0 btn=1/4 axes= us= rep=0000"),
    ("l_buttons_continue_unaligned", "L b3 b3 b1 end", "ok ok ok ok | bit=15 abs=0 rel=0 hat=0/0 btn=1/7 axes= us= rep=0000"),
    ("l_hat_zero_pairs", "L h0 h2 h1 end", "ok ok ok ok | bit=24 abs=0 rel=0 hat=1/3 btn=0/0 axes= us= rep=000000"),
    ("l_hat_after_buttons_rejected", "L b1 h1 end", "ok no ok | bit=9 abs=0 rel=0 hat=0/0 btn=1/1 axes= us= rep=0000"),
    ("l_unknown_op_without_report", "L z a8x1", "no ok | bit=16 abs=1 rel=0 hat=0/0 btn=0/0 axes=1:8 us=48 rep=-"),
    ("w_axes_le_mixed", "L a8x1 a16x2 end X0:9 X1:-2 X2:258", "ok ok ok ok ok ok | bit=48 abs=3 rel=0 hat=0/0 btn=0/0 axes=1:8,2:16,4:16 us=48,48,49 rep=0009feff0201"),
    ("w_axis8_truncates_a32", "L a8x3 a32x1 end X0:300 X3:-1", "ok ok ok ok ok | bit=64 abs=4 rel=0 hat=0/0 btn=0/0 axes=1:8,2:8,3:8,4:32 us=48,49,50,48 rep=002c0000ffffffff"),
    ("w_rel_axis_writes", "L a8x1 r16x1 end Y0:-3 Y1:7 X0:9", "ok ok ok ok no ok | bit=32 abs=1 rel=1 hat=0/0 btn=0/0 axes=1:8,2:16 us=48 rep=0009fdff"),
    ("w_button_bits", "L a8x1 b10 end B0:1 B9:1 B9:0 B10:1", "ok ok ok ok ok ok no | bit=26 abs=1 rel=0 hat=0/0 btn=2/10 axes=1:8 us=48 rep=00000100"),
    ("w_hat_pairs", "L h2 end x0:1 y0:-1 x1:-1 y1:1", "ok ok ok ok ok ok | bit=16 abs=0 rel=0 hat=1/2 btn=0/0 axes= us= rep=0062"),
    ("w_hat_index_equal_count", "L h1 end x1:1 x2:1", "ok ok ok no | bit=16 abs=0 rel=0 hat=1/1 btn=0/0 axes= us= rep=0030"),
    ("w_hat_move", "L h1 end x0:1 m0:1:-1 m0:-1:-1", "ok ok ok ok ok | bit=16 abs=0 rel=0 hat=1/1 btn=0/0 axes= us= rep=0001"),
    ("w_axis_index_bounds", "L a8x1 end X1:5 Y0:1", "ok ok no no | bit=16 abs=1 rel=0 hat=0/0 btn=0/0 axes=1:8 us=48 rep=0000"),
    ("r_sync_copies_report", "R 0102 0304 s", "ok | rep=0102 last=0102"),
    ("r_drop_then_sync", "R 0102 0304 d s", "drop no | rep=0304 last=0304"),
    ("r_second_sync", "R 0102 0304 d s s", "drop no ok | rep=0304 last=0304"),
    ("e_waveforms", "E 30 31 32 33 34", "T51:58 T51:5a T51:59 T51:5b T51:5c"),
    ("e_other_types", "E 26 27 28 40 41 42 43 0 35 ff", "T52 T57 T5d T53 T55 T56 T54 unsupported unsupported unsupported"),
    ("t_decompose_nibbles", "T d0:0 d1:0 d2:0 d3:0 d4:0 d5:0 d6:0 d7:0 d8:0 d9:0 d15:0 d255:0 d16:1 d34:1 d136:1 d255:1", "0:0 0:-1 1:-1 1:0 1:1 0:1 -1:1 -1:0 -1:-1 0:0 0:0 0:0 0:-1 1:-1 -1:-1 0:0"),
    ("t_compose_both_parities", "T c0:-1:0:0 c1:-1:0:0 c-1:-1:0:255 c-1:0:0:240 c0:0:0:15 c0:1:0:0 c1:1:0:0 c-1:1:0:0 c-5:-7:0:0 c3:0:0:0 c0:-1:1:0 c1:-1:1:255 c-1:-1:1:15 c0:0:1:240 c1:0:1:0 c0:1:1:0 c-1:1:1:0 c-1:0:1:0 c2:2:1:0 c2:-2:1:0", "1 2 248 247 0 5 4 6 8 3 16 47 143 0 48 80 96 112 64 32"),
];
#[test]
fn rows_match_recorded_c_output() {
    for (name, case, expected) in ROWS {
        assert_eq!(run_case(case), *expected, "row {name}");
    }
}
