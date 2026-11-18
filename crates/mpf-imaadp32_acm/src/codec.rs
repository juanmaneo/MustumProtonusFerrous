//! The IMA ADPCM nibble codec and the per-block routines of the four convert
//! functions in `imaadp32.c`.

const IMA_STEP_TABLE: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

const IMA_INDEX_TABLE: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];

const MAX_STEP_INDEX: i32 = 88;

/// The predictor state of one channel: the last sample and the step index.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct State {
    pub(crate) sample: i32,
    pub(crate) index: i32,
}

/// The shape of one block: its channel count, its samples per block including
/// the header sample, and the byte width of one PCM sample.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Layout {
    pub(crate) channels: usize,
    pub(crate) samples: usize,
    pub(crate) sample_bytes: usize,
}

/// `process_nibble`: decodes the low four bits of `code` into `state`.
fn process_nibble(code: u8, state: &mut State) {
    let code = code & 0x0F;
    let step = IMA_STEP_TABLE[state.index as usize];
    let mut diff = step >> 3;
    if code & 1 != 0 {
        diff += step >> 2;
    }
    if code & 2 != 0 {
        diff += step >> 1;
    }
    if code & 4 != 0 {
        diff += step;
    }
    state.sample = if code & 8 != 0 {
        state.sample - diff
    } else {
        state.sample + diff
    }
    .clamp(-32768, 32767);
    state.index = (state.index + IMA_INDEX_TABLE[usize::from(code)]).clamp(0, MAX_STEP_INDEX);
}

/// `generate_nibble`: encodes `input` against the predictor in `state` and
/// returns the 4-bit code.
fn generate_nibble(input: i32, state: &mut State) -> u8 {
    let mut diff = input - state.sample;
    let mut code = 0u8;
    if diff < 0 {
        diff = -diff;
        code = 8;
    }
    let mut step = IMA_STEP_TABLE[state.index as usize];
    let mut effdiff = step >> 3;
    if diff >= step {
        code |= 4;
        diff -= step;
        effdiff += step;
    }
    step >>= 1;
    if diff >= step {
        code |= 2;
        diff -= step;
        effdiff += step;
    }
    step >>= 1;
    if diff >= step {
        code |= 1;
        effdiff += step;
    }
    state.sample = if code & 8 != 0 {
        state.sample - effdiff
    } else {
        state.sample + effdiff
    }
    .clamp(-32768, 32767);
    state.index = (state.index + IMA_INDEX_TABLE[usize::from(code)]).clamp(0, MAX_STEP_INDEX);
    code
}

/// `R16`: a little-endian signed 16-bit sample.
fn read_sample(src: &[u8]) -> i32 {
    i32::from(i16::from_le_bytes([src[0], src[1]]))
}

/// `W8_16`: writes one PCM sample of `dst.len()` bytes, 8-bit unsigned or
/// 16-bit little-endian.
fn write_sample(dst: &mut [u8], sample: i32) {
    if dst.len() == 1 {
        dst[0] = ((sample + 32768) >> 8) as u8;
    } else {
        dst.copy_from_slice(&(sample as i16).to_le_bytes());
    }
}

/// The byte that holds nibble `k` of channel `c`. Each block header is four
/// bytes per channel, and every eight nibbles of a channel take four bytes.
fn nibble_offset(channels: usize, c: usize, k: usize) -> usize {
    4 * channels * (1 + k / 8) + 4 * c + (k % 8) / 2
}

/// Decodes one block of `layout.samples` PCM frames from the ADPCM block `src`
/// into `dst`. The predictor is reset by each block's header.
pub(crate) fn decode_block(src: &[u8], dst: &mut [u8], layout: Layout) {
    let channels = layout.channels;
    let bytes = layout.sample_bytes;
    let mut states = [State::default(); 2];
    for (c, state) in states.iter_mut().enumerate().take(channels) {
        state.sample = read_sample(&src[4 * c..]);
        state.index = i32::from(src[4 * c + 2]).clamp(0, MAX_STEP_INDEX);
        write_sample(&mut dst[c * bytes..(c + 1) * bytes], state.sample);
    }
    for k in 0..layout.samples - 1 {
        for (c, state) in states.iter_mut().enumerate().take(channels) {
            let byte = src[nibble_offset(channels, c, k)];
            let code = if k % 2 == 0 { byte } else { byte >> 4 };
            process_nibble(code, state);
            let at = ((k + 1) * channels + c) * bytes;
            write_sample(&mut dst[at..at + bytes], state.sample);
        }
    }
}

/// Encodes one block from the 16-bit PCM frames `src` into the ADPCM block
/// `dst`. `states` carries the step index from one block to the next.
pub(crate) fn encode_block(src: &[u8], dst: &mut [u8], layout: Layout, states: &mut [State; 2]) {
    let channels = layout.channels;
    let step_width = if channels == 1 { 1 } else { 2 };
    for (c, state) in states.iter_mut().enumerate().take(channels) {
        state.sample = read_sample(&src[2 * c..]);
        let head = 4 * c;
        dst[head..head + 2].copy_from_slice(&(state.sample as i16).to_le_bytes());
        let step = (state.index as u16).to_le_bytes();
        dst[head + 2..head + 2 + step_width].copy_from_slice(&step[..step_width]);
    }
    for k in 0..layout.samples - 1 {
        for (c, state) in states.iter_mut().enumerate().take(channels) {
            let code = generate_nibble(read_sample(&src[2 * ((k + 1) * channels + c)..]), state);
            let at = nibble_offset(channels, c, k);
            if k % 2 == 0 {
                dst[at] = code;
            } else {
                dst[at] |= code << 4;
            }
        }
    }
}
