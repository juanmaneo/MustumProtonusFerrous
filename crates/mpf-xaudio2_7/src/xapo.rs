use crate::hresult::failed;

/// Port of `XAPOFX_CalcInputFrames`: the frame count is ignored and 0 is returned.
pub fn calc_input_frames(_output_frames: u32) -> u32 {
    0
}

/// Port of `XAPOFX_CalcOutputFrames`: the frame count is ignored and 0 is returned.
pub fn calc_output_frames(_input_frames: u32) -> u32 {
    0
}

/// Port of the return value of `XAPO_GetRegistrationProperties`. A failed HRESULT from the
/// wrapped XAPO is returned as is. On success the C code returns 0 and hands out the properties
/// pointer, which is not modelled.
pub fn get_registration_properties_status(hr: u32) -> u32 {
    if failed(hr) {
        hr
    } else {
        0
    }
}
