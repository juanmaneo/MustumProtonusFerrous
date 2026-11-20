use crate::geometry::{check_normalized_rect, NormalizedRect};
use crate::{
    HResult, E_INVALIDARG, MF_E_BUFFERTOOSMALL, MF_E_INVALIDREQUEST, MF_E_INVALIDSTREAMNUMBER,
};

/// `MAX_MIXER_INPUT_STREAMS`: the reference stream and up to 15 input streams.
pub const MAX_INPUT_STREAMS: usize = 16;

/// `struct input_stream`, without the sample attributes and media type.
#[derive(Clone, Copy, Debug, PartialEq)]
struct InputStream {
    id: u32,
    zorder: u32,
    rect: NormalizedRect,
}

/// The reference stream and the input streams of `struct video_mixer`.
///
/// `inputs` is sorted by stream id, which is what the C `bsearch` needs. `input_ids` keeps the
/// ids in insertion order, which is the order `GetStreamIDs` reports them in. The reference
/// stream, id `0`, is present from the start.
#[derive(Clone, Debug, PartialEq)]
pub struct InputStreams {
    inputs: Vec<InputStream>,
    input_ids: Vec<u32>,
}

impl Default for InputStreams {
    fn default() -> Self {
        Self {
            inputs: vec![InputStream {
                id: 0,
                zorder: 0,
                rect: NormalizedRect::FULL,
            }],
            input_ids: vec![0],
        }
    }
}

impl InputStreams {
    fn index_of(&self, id: u32) -> Result<usize, HResult> {
        self.inputs
            .binary_search_by_key(&id, |input| input.id)
            .map_err(|_| MF_E_INVALIDSTREAMNUMBER)
    }

    /// `video_mixer_transform_AddInputStreams`: adds the streams in `ids`, each on top of the
    /// streams before it. The batch is rejected as a whole, with `E_INVALIDARG`, if it does not
    /// fit, repeats an id, or repeats an id that is already present.
    pub fn add(&mut self, ids: &[u32]) -> Result<(), HResult> {
        if ids.len() > MAX_INPUT_STREAMS - self.inputs.len() {
            return Err(E_INVALIDARG);
        }

        let mut sorted = ids.to_vec();
        sorted.sort_unstable();
        if sorted.windows(2).any(|pair| pair[0] == pair[1])
            || ids.iter().any(|&id| self.index_of(id).is_ok())
        {
            return Err(E_INVALIDARG);
        }

        let first_zorder = self.inputs.len() as u32;
        for (offset, &id) in ids.iter().enumerate() {
            let at = self.inputs.partition_point(|input| input.id < id);
            self.inputs.insert(
                at,
                InputStream {
                    id,
                    zorder: first_zorder + offset as u32,
                    rect: NormalizedRect::FULL,
                },
            );
        }
        self.input_ids.extend_from_slice(ids);
        Ok(())
    }

    /// `video_mixer_transform_DeleteInputStream`. The reference stream, id `0`, cannot be
    /// deleted.
    ///
    /// The C body removes the entry at the stream's index in the sorted `inputs` from the
    /// insertion-ordered `input_ids` as well. The two orders differ once ids are added out of
    /// order. After `add(&[5, 3])`, deleting `3` leaves `[0, 3]`, so `5` is dropped and `3` is
    /// kept. The port keeps this as it is in C. It looks like a Wine bug.
    pub fn delete(&mut self, id: u32) -> Result<(), HResult> {
        if id == 0 {
            return Err(MF_E_INVALIDSTREAMNUMBER);
        }

        let index = self.index_of(id)?;
        self.inputs.remove(index);
        self.input_ids.remove(index);
        Ok(())
    }

    /// `video_mixer_control_SetStreamZOrder`. The reference stream keeps zorder `0`, and no
    /// other stream can take it.
    pub fn set_zorder(&mut self, id: u32, zorder: u32) -> Result<(), HResult> {
        if id == 0 && zorder != 0 {
            return Err(E_INVALIDARG);
        }
        if zorder as usize >= self.inputs.len() {
            return Err(E_INVALIDARG);
        }

        let index = self.index_of(id)?;
        if id != 0 && zorder == 0 {
            return Err(MF_E_INVALIDREQUEST);
        }
        self.inputs[index].zorder = zorder;
        Ok(())
    }

    /// `video_mixer_control_GetStreamZOrder`.
    pub fn get_zorder(&self, id: u32) -> Result<u32, HResult> {
        Ok(self.inputs[self.index_of(id)?].zorder)
    }

    /// `video_mixer_control_SetStreamOutputRect`. The rectangle is checked before the stream is
    /// looked up.
    pub fn set_output_rect(&mut self, id: u32, rect: &NormalizedRect) -> Result<(), HResult> {
        check_normalized_rect(rect)?;

        let index = self.index_of(id)?;
        self.inputs[index].rect = *rect;
        Ok(())
    }

    /// `video_mixer_control_GetStreamOutputRect`.
    pub fn get_output_rect(&self, id: u32) -> Result<NormalizedRect, HResult> {
        Ok(self.inputs[self.index_of(id)?].rect)
    }

    /// `video_mixer_transform_GetStreamIDs`: the input ids in insertion order. The reply is
    /// `MF_E_BUFFERTOOSMALL` when the list does not fit in `input_size`, or when `output_size`
    /// is `0`.
    pub fn stream_ids(&self, input_size: u32, output_size: u32) -> Result<Vec<u32>, HResult> {
        if self.input_ids.len() > input_size as usize || output_size == 0 {
            return Err(MF_E_BUFFERTOOSMALL);
        }
        Ok(self.input_ids.clone())
    }
}
