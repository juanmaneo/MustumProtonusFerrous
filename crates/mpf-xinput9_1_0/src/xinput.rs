/// `ERROR_SUCCESS` from `include/winerror.h`.
pub const ERROR_SUCCESS: u32 = 0;
/// `ERROR_BAD_ARGUMENTS` from `include/winerror.h`.
pub const ERROR_BAD_ARGUMENTS: u32 = 160;
/// `ERROR_DEVICE_NOT_CONNECTED` from `include/winerror.h`.
pub const ERROR_DEVICE_NOT_CONNECTED: u32 = 1167;
/// `XUSER_MAX_COUNT` from `include/xinput.h`: the number of controller slots.
pub const XUSER_MAX_COUNT: u32 = 4;

/// A `GUID`, 16 bytes. The port only copies it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid(pub [u8; 16]);

/// `GUID_NULL`, defined in `dlls/xinput9_1_0/main.c`.
pub const GUID_NULL: Guid = Guid([0; 16]);

/// `XINPUT_CAPABILITIES`, 20 bytes. The port does not read it: it is passed through to the backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities(pub [u8; 20]);

/// `XINPUT_STATE`, 16 bytes. The port does not read it: it is passed through to the backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State(pub [u8; 16]);

/// `XINPUT_VIBRATION`, 4 bytes. The port does not read it: it is passed through to the backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vibration(pub [u8; 4]);

/// `pXInputGetCapabilities`: the xinput1_4 export, where `None` is the NULL pointer.
pub type GetCapabilitiesFn = fn(u32, u32, Option<&mut Capabilities>) -> u32;
/// `pXInputGetState`: the xinput1_4 export, where `None` is the NULL pointer.
pub type GetStateFn = fn(u32, Option<&mut State>) -> u32;
/// `pXInputSetState`: the xinput1_4 export, where `None` is the NULL pointer.
pub type SetStateFn = fn(u32, Option<&mut Vibration>) -> u32;

/// The three `pXInput*` pointers that `init_xinput1_4_funcs` fills in. Each is set or NULL on its own.
#[derive(Clone, Copy, Default)]
pub struct Xinput14Funcs {
    pub get_capabilities: Option<GetCapabilitiesFn>,
    pub get_state: Option<GetStateFn>,
    pub set_state: Option<SetStateFn>,
}

/// The process state of xinput9_1_0. `loader` plays `init_xinput1_4_funcs`.
pub struct Xinput9<L> {
    loader: Option<L>,
    funcs: Xinput14Funcs,
}

impl<L: FnOnce() -> Xinput14Funcs> Xinput9<L> {
    /// A process that has not loaded xinput1_4 yet.
    pub fn new(loader: L) -> Self {
        Self {
            loader: Some(loader),
            funcs: Xinput14Funcs::default(),
        }
    }

    /// `InitOnceExecuteOnce(&init_xinput1_4_once, ...)`: runs the loader on the first call only,
    /// and every export runs it before its own argument checks.
    fn load_once(&mut self) -> Xinput14Funcs {
        if let Some(load) = self.loader.take() {
            self.funcs = load();
        }
        self.funcs
    }

    /// `XInputGetCapabilities`: forwards to the backend, without checking `index`.
    pub fn get_capabilities(
        &mut self,
        index: u32,
        flags: u32,
        capabilities: Option<&mut Capabilities>,
    ) -> u32 {
        match self.load_once().get_capabilities {
            Some(get) => get(index, flags, capabilities),
            None => ERROR_DEVICE_NOT_CONNECTED,
        }
    }

    /// `XInputGetDSoundAudioDeviceGuids`: a bad `index` or a NULL GUID pointer fails before the
    /// backend is called. If the backend `XInputGetState` fails, its error is returned and the
    /// GUIDs are left alone. On success both GUIDs are set to `GUID_NULL`.
    pub fn get_dsound_audio_device_guids(
        &mut self,
        index: u32,
        render_guid: Option<&mut Guid>,
        capture_guid: Option<&mut Guid>,
    ) -> u32 {
        let funcs = self.load_once();
        let (render_guid, capture_guid) = match (render_guid, capture_guid) {
            (Some(render), Some(capture)) if index < XUSER_MAX_COUNT => (render, capture),
            _ => return ERROR_BAD_ARGUMENTS,
        };
        let Some(get_state) = funcs.get_state else {
            return ERROR_DEVICE_NOT_CONNECTED;
        };
        let mut state = State([0; 16]);
        let ret = get_state(index, Some(&mut state));
        if ret != ERROR_SUCCESS {
            return ret;
        }
        // Docs say this function returns no results on Win8 and later.
        *render_guid = GUID_NULL;
        *capture_guid = GUID_NULL;
        ERROR_SUCCESS
    }

    /// `XInputGetState`: forwards to the backend, without checking `index`.
    pub fn get_state(&mut self, index: u32, state: Option<&mut State>) -> u32 {
        match self.load_once().get_state {
            Some(get) => get(index, state),
            None => ERROR_DEVICE_NOT_CONNECTED,
        }
    }

    /// `XInputSetState`: forwards to the backend, without checking `index`.
    pub fn set_state(&mut self, index: u32, vibration: Option<&mut Vibration>) -> u32 {
        match self.load_once().set_state {
            Some(set) => set(index, vibration),
            None => ERROR_DEVICE_NOT_CONNECTED,
        }
    }
}
