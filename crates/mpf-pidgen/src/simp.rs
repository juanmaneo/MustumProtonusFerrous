/// `PIDGenSimpA` from `dlls/pidgen/main.c`. The C body only logs its arguments and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn pid_gen_simp_a(
    _str: Option<&[u8]>,
    _p1: i32,
    _p2: i32,
    _p3: i32,
    _p4: i32,
    _p5: i32,
    _p6: i32,
    _p7: i32,
    _p8: i32,
) -> i32 {
    0
}

/// `PIDGenSimpW` from `dlls/pidgen/main.c`. The C body only logs its arguments and returns 0.
#[allow(clippy::too_many_arguments)]
pub fn pid_gen_simp_w(
    _str: Option<&[u16]>,
    _p1: i32,
    _p2: i32,
    _p3: i32,
    _p4: i32,
    _p5: i32,
    _p6: i32,
    _p7: i32,
    _p8: i32,
) -> i32 {
    0
}
