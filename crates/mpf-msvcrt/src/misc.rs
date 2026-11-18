//! Byte swapping, `rand`/`srand`, `bsearch` and `qsort` from Wine's `dlls/msvcrt/misc.c`.
//!
//! Wine keeps the `rand` seed per thread (`thread_data_t::random_seed`, initial
//! value 1). This port keeps it in an explicit `Rand` value instead of a global.

use std::cmp::Ordering;

/// `RAND_MAX` on Windows.
pub const RAND_MAX: i32 = 0x7fff;

pub fn byteswap_ushort(value: u16) -> u16 {
    value.swap_bytes()
}

pub fn byteswap_ulong(value: u32) -> u32 {
    value.swap_bytes()
}

pub fn byteswap_uint64(value: u64) -> u64 {
    value.swap_bytes()
}

/// The linear congruential generator behind MSVC's `rand`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rand {
    seed: u32,
}

impl Default for Rand {
    fn default() -> Self {
        Self { seed: 1 }
    }
}

impl Rand {
    /// `srand`: replaces the seed.
    pub fn srand(&mut self, seed: u32) {
        self.seed = seed;
    }

    /// `rand`: advances the seed and returns a value in `0..=RAND_MAX`.
    pub fn rand(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(214_013).wrapping_add(2_531_011);
        ((self.seed >> 16) & RAND_MAX as u32) as i32
    }
}

/// `bsearch`: returns the index of an element that compares equal to `key`.
///
/// `compare(key, item)` must return `Less` when `key` sorts before `item`. Wine's
/// `bsearch_s` keeps an inclusive `min..=max` range and probes `min + (max - min) / 2`.
/// This is the same probe sequence over a half-open `lo..hi` range, so the index
/// returned for unsorted input matches Wine.
pub fn bsearch<K, T>(
    key: &K,
    items: &[T],
    mut compare: impl FnMut(&K, &T) -> Ordering,
) -> Option<usize> {
    let (mut lo, mut hi) = (0, items.len());
    while lo < hi {
        let cursor = lo + (hi - 1 - lo) / 2;
        match compare(key, &items[cursor]) {
            Ordering::Equal => return Some(cursor),
            Ordering::Less => hi = cursor,
            Ordering::Greater => lo = cursor + 1,
        }
    }
    None
}

/// `qsort`: sorts `items` in place with Wine's own quicksort.
///
/// This is not the standard library's sort. Segments of eight or more elements are
/// partitioned around a median of three; smaller segments use a selection sort.
/// Elements that compare equal end up in an order fixed by that algorithm, and this
/// port reproduces it, so tie order matches Wine.
pub fn qsort<T>(items: &mut [T], mut compare: impl FnMut(&T, &T) -> Ordering) {
    if items.len() < 2 {
        return;
    }
    quick_sort(items, &mut compare);
}

fn small_sort<T, F>(items: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    for end in (2..=items.len()).rev() {
        let mut max = 0;
        for i in 1..end {
            if compare(&items[i], &items[max]) == Ordering::Greater {
                max = i;
            }
        }
        items.swap(end - 1, max);
    }
}

fn quick_sort<T, F>(items: &mut [T], compare: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut stack_lo = [0_usize; usize::BITS as usize];
    let mut stack_hi = [0_usize; usize::BITS as usize];
    stack_hi[0] = items.len() - 1;
    let mut depth = 1;
    while depth > 0 {
        depth -= 1;
        let (beg, end) = (stack_lo[depth], stack_hi[depth]);
        if end - beg < 8 {
            small_sort(&mut items[beg..=end], compare);
            continue;
        }

        let mut lo = beg;
        let mut hi = end;
        let mut med = lo + (hi - lo).div_ceil(2);
        if compare(&items[lo], &items[med]) == Ordering::Greater {
            items.swap(lo, med);
        }
        if compare(&items[lo], &items[hi]) == Ordering::Greater {
            items.swap(lo, hi);
        }
        if compare(&items[med], &items[hi]) == Ordering::Greater {
            items.swap(med, hi);
        }

        lo += 1;
        hi -= 1;
        loop {
            while lo <= hi {
                if lo != med && compare(&items[lo], &items[med]) == Ordering::Greater {
                    break;
                }
                lo += 1;
            }
            while med != hi {
                if compare(&items[hi], &items[med]) != Ordering::Greater {
                    break;
                }
                hi -= 1;
            }
            if hi < lo {
                break;
            }
            items.swap(lo, hi);
            if hi == med {
                med = lo;
            }
            lo += 1;
            hi -= 1;
        }

        while hi > beg {
            if hi != med && compare(&items[hi], &items[med]) != Ordering::Equal {
                break;
            }
            hi -= 1;
        }

        let (first, second) = if hi - beg >= end - lo {
            ((beg, hi), (lo, end))
        } else {
            ((lo, end), (beg, hi))
        };
        stack_lo[depth] = first.0;
        stack_hi[depth] = first.1;
        stack_lo[depth + 1] = second.0;
        stack_hi[depth + 1] = second.1;
        depth += 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byteswap_reverses_each_width() {
        assert_eq!(byteswap_ushort(0x1234), 0x3412);
        assert_eq!(byteswap_ulong(0x1234_5678), 0x7856_3412);
        assert_eq!(
            byteswap_uint64(0x0102_0304_0506_0708),
            0x0807_0605_0403_0201
        );
    }

    #[test]
    fn default_seed_produces_msvc_sequence() {
        let mut rand = Rand::default();
        let values: Vec<i32> = (0..5).map(|_| rand.rand()).collect();
        assert_eq!(values, [41, 18467, 6334, 26500, 19169]);
    }

    #[test]
    fn srand_restarts_the_sequence() {
        let mut rand = Rand::default();
        rand.srand(0);
        let first = rand.rand();
        rand.srand(0);
        assert_eq!(rand.rand(), first);
        assert_eq!(first, 38);
    }

    #[test]
    fn values_stay_within_rand_max() {
        let mut rand = Rand::default();
        rand.srand(u32::MAX);
        for _ in 0..1000 {
            let value = rand.rand();
            assert!((0..=RAND_MAX).contains(&value));
        }
    }

    #[test]
    fn bsearch_matches_wine_table() {
        // Wine's `bsearch` on the C build of `misc.c`: (key, items, index).
        let cases: &[(i32, &[i32], Option<usize>)] = &[
            (-1, &[], None),
            (-1, &[1, 1, 1], None),
            (5, &[-2, 1, -2, 3, 2, -4], None),
            (-3, &[-2, 2, 0, 2], None),
            (-3, &[0, -2, 0, -3, -3, -1, -3], Some(3)),
            (0, &[-3, -4, -1, -2, 0], Some(4)),
            (-1, &[4, -3, -1, -1, 0, -4], Some(2)),
            (2, &[-1, 1, 2, -4, 3, 4], Some(2)),
            (-1, &[-4, 0, 3, 1, 0, 4], None),
            (4, &[-4, 2, 4, -2, -1, 4, 4], Some(5)),
            (5, &[2, -1, 1, 1, -4, 2, 0], None),
            (3, &[-4, -3, 1, -4, 3, 1], Some(4)),
            (-4, &[4, 3, -4, -4, -2, -2], Some(2)),
            (1, &[1, 3, -4, 4, -1, 0, -1], Some(0)),
            (-3, &[1, -4, 4, 3, -1, 3, 3], None),
            (-4, &[1, 0, -3, -4, 0], None),
            (-4, &[-1, 2, 3, -2, -2, 0, 0], None),
            (-4, &[-4, -3, -1, 4, -1, -1, 1], Some(0)),
            (-5, &[1, 2, 3, 4, -2, 1, 4, -4], None),
            (-3, &[-3, -4, -3, -4, 2, -2], Some(2)),
            (4, &[2, -4, 1, -3, 3, -3, 4], Some(6)),
            (-3, &[0, -3, -4, -2, -2, 3, -3], Some(1)),
        ];
        for &(key, items, expected) in cases {
            assert_eq!(
                bsearch(&key, items, |k, v| k.cmp(v)),
                expected,
                "key {key} in {items:?}"
            );
        }
    }

    #[test]
    fn qsort_matches_wine_tie_order() {
        // Wine's `qsort` on (key, tag) pairs, comparing keys only: final tag order.
        let cases: &[(&[i32], &[usize])] = &[
            (&[2, 2, 2, 0, 1], &[3, 4, 2, 1, 0]),
            (&[1, 0, 2, 1, 2, 0], &[1, 5, 3, 0, 4, 2]),
            (&[1, 0, 0, 0, 1, 1, 0, 1], &[1, 2, 3, 6, 5, 4, 7, 0]),
            (&[], &[]),
            (&[1], &[0]),
            (&[0, 0], &[1, 0]),
            (&[1, 1, 0, 1, 0, 0, 0], &[5, 2, 4, 6, 3, 1, 0]),
            (&[1, 0, 1, 1, 1], &[1, 2, 3, 4, 0]),
            (
                &[-2, -1, 0, 0, -2, -1, 1, -2, 1, -1, -2, 0, 2, -1, 2, -2],
                &[0, 10, 15, 7, 4, 5, 13, 1, 9, 3, 11, 2, 6, 8, 14, 12],
            ),
            (
                &[
                    2, 1, 3, -2, 3, -1, 2, 1, -3, 2, 3, 0, -2, 1, -2, -2, -3, 2, 0, -1, -3, -3, 2,
                    -1, 0, -3, 0, -2,
                ],
                &[
                    20, 25, 21, 16, 8, 15, 27, 14, 3, 12, 5, 23, 19, 18, 24, 26, 11, 7, 13, 1, 0,
                    22, 9, 17, 6, 4, 10, 2,
                ],
            ),
            (
                &[0, 1, 0, 1, 0, 0, 1, 1, 0, -1],
                &[9, 8, 2, 5, 4, 0, 7, 1, 6, 3],
            ),
            (
                &[5, 1, 1, -4, -3, 3, 4, 4, 3, -3, 1, 0, 1],
                &[3, 4, 9, 11, 10, 12, 2, 1, 5, 8, 6, 7, 0],
            ),
            (
                &[1, 4, 1, 3, 4, -3, -2, -2, 1, 0, -4, -4],
                &[10, 11, 5, 6, 7, 9, 2, 8, 0, 3, 1, 4],
            ),
            (
                &[
                    2, -2, 2, -2, -1, 1, -2, -2, 1, 2, -1, -1, 1, 1, -1, 1, -2, -1, -2, -1,
                ],
                &[
                    1, 18, 3, 7, 6, 16, 17, 4, 10, 14, 19, 11, 13, 5, 15, 8, 12, 2, 0, 9,
                ],
            ),
            (
                &[
                    13, -13, 6, 4, 4, -4, 18, 7, -17, 19, -16, -19, 19, -10, 5, -17, 5, 4, 3, 5,
                    19, -12, 5,
                ],
                &[
                    11, 8, 15, 10, 1, 21, 13, 5, 18, 3, 4, 17, 16, 22, 14, 19, 2, 7, 0, 6, 20, 9,
                    12,
                ],
            ),
            (
                &[0, 0, 16, 9, 14, -5, 19, 12, 13, -5, -15, 2, 12, -2, -5, -17],
                &[15, 10, 5, 14, 9, 13, 1, 0, 11, 3, 12, 7, 8, 4, 2, 6],
            ),
            (
                &[
                    1, -2, -1, 1, 1, 1, -2, 0, -1, 0, -2, 2, 2, -1, -1, 0, -2, -1, 2, 0,
                ],
                &[
                    10, 1, 6, 16, 17, 14, 2, 8, 13, 19, 15, 9, 7, 3, 5, 4, 0, 18, 11, 12,
                ],
            ),
            (
                &[
                    -3, 3, 1, 3, -2, -3, 3, 2, 0, -1, 1, -2, -3, 3, -3, 3, 3, 0, -1, 2, -3, -1, 1,
                    2, -2,
                ],
                &[
                    0, 20, 14, 12, 5, 4, 24, 11, 21, 9, 18, 17, 8, 2, 10, 22, 23, 19, 7, 1, 13, 3,
                    16, 6, 15,
                ],
            ),
            (
                &[1, 0, 0, 1, 0, -1, 0, 0, 1, 1, 0, 0],
                &[5, 2, 10, 4, 1, 6, 11, 7, 9, 3, 0, 8],
            ),
            (
                &[
                    -1, 0, 1, 1, -1, -1, 0, -1, 0, 0, -1, -1, 1, -1, 1, 0, -1, 1, -1, 1, 0, 1, 1,
                    1, 0, 0,
                ],
                &[
                    0, 18, 16, 13, 4, 5, 11, 7, 10, 9, 8, 6, 24, 20, 1, 15, 25, 2, 14, 19, 3, 21,
                    22, 23, 12, 17,
                ],
            ),
        ];
        for &(keys, expected) in cases {
            let mut items: Vec<(i32, usize)> = keys.iter().copied().zip(0..).collect();
            qsort(&mut items, |a, b| a.0.cmp(&b.0));
            let tags: Vec<usize> = items.iter().map(|item| item.1).collect();
            assert_eq!(tags, expected, "keys {keys:?}");
        }
    }
}
