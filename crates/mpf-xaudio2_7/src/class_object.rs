use crate::guid::{is_equal_guid, Guid};
use crate::hresult::{CLASS_E_CLASSNOTAVAILABLE, CLASS_E_NOAGGREGATION, E_INVALIDARG, S_OK};

pub const CLSID_XAUDIO2: Guid = Guid {
    data1: 0x5a50_8685,
    data2: 0xa254,
    data3: 0x4fba,
    data4: [0x9b, 0x82, 0x9a, 0x24, 0xb0, 0x03, 0x06, 0xaf],
};
pub const CLSID_AUDIO_VOLUME_METER: Guid = Guid {
    data1: 0xcac1_105f,
    data2: 0x619b,
    data3: 0x4d04,
    data4: [0x83, 0x1a, 0x44, 0xe1, 0xcb, 0xf1, 0x2d, 0x57],
};
pub const CLSID_AUDIO_REVERB: Guid = Guid {
    data1: 0x6a93_130e,
    data2: 0x1d53,
    data3: 0x41d1,
    data4: [0xa9, 0xcf, 0xe7, 0x58, 0x80, 0x0b, 0xb1, 0x79],
};
pub const CLSID_FX_REVERB: Guid = Guid {
    data1: 0x7d9a_ca56,
    data2: 0xcb68,
    data3: 0x4807,
    data4: [0xb6, 0x32, 0xb1, 0x37, 0x35, 0x2e, 0x85, 0x96],
};

#[derive(PartialEq, Eq)]
pub enum XapoClass {
    VolumeMeter,
    Reverb,
}

#[derive(PartialEq, Eq)]
pub enum ClassObject {
    XAudio2,
    Xapo(XapoClass),
}

/// Port of `DllGetClassObject`. The `riid` argument is not checked: the C factory constructors
/// ignore it, so the port does not take it.
pub fn class_object(rclsid: &Guid) -> Result<ClassObject, u32> {
    if is_equal_guid(rclsid, &CLSID_XAUDIO2) {
        Ok(ClassObject::XAudio2)
    } else if is_equal_guid(rclsid, &CLSID_AUDIO_VOLUME_METER) {
        Ok(ClassObject::Xapo(XapoClass::VolumeMeter))
    } else if is_equal_guid(rclsid, &CLSID_AUDIO_REVERB) {
        Ok(ClassObject::Xapo(XapoClass::Reverb))
    } else {
        Err(CLASS_E_CLASSNOTAVAILABLE)
    }
}

/// Port of the `pOuter` check shared by `XAudio2CF_CreateInstance` and `xapocf_CreateInstance`.
/// `has_outer` is true when `pOuter` is non-NULL.
pub fn aggregation_gate(has_outer: bool) -> Result<(), u32> {
    if has_outer {
        Err(CLASS_E_NOAGGREGATION)
    } else {
        Ok(())
    }
}

/// Port of the part of `XAudio2CF_CreateInstance` that runs before the engine is built.
pub fn xaudio2_create_instance_gate(has_outer: bool) -> Result<(), u32> {
    aggregation_gate(has_outer)
}

/// Port of the gate and class dispatch in `xapocf_CreateInstance` for XAUDIO2_VER 7. `clsid` is
/// the class the factory was created for. The `FXReverb` branch is compiled out for version 7,
/// so every class other than the two XAPOs is `E_INVALIDARG`.
pub fn xapo_create_instance_class(clsid: &Guid, has_outer: bool) -> Result<XapoClass, u32> {
    aggregation_gate(has_outer)?;
    if is_equal_guid(clsid, &CLSID_AUDIO_VOLUME_METER) {
        Ok(XapoClass::VolumeMeter)
    } else if is_equal_guid(clsid, &CLSID_AUDIO_REVERB) {
        Ok(XapoClass::Reverb)
    } else {
        Err(E_INVALIDARG)
    }
}

/// Port of the `FIXME` stub `XAudio2CF_LockServer`: the flag is ignored and `S_OK` returned.
pub fn xaudio2_cf_lock_server(_dolock: bool) -> u32 {
    S_OK
}

/// Port of the `FIXME` stub `xapocf_LockServer`: the flag is ignored and `S_OK` returned.
pub fn xapo_cf_lock_server(_dolock: bool) -> u32 {
    S_OK
}
