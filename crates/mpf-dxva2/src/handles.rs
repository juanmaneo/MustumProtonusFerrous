use crate::{DXVA2_E_NEW_VIDEO_DEVICE, DXVA2_E_NOT_INITIALIZED, E_HANDLE, E_INVALIDARG, HRESULT};

/// An `HANDLE` from the device manager. Zero is `NULL`, and handle `n` is slot `n - 1`.
pub type Handle = usize;

/// One entry of `struct device_handle`. The C `flags` word holds the two bits `open` and
/// `invalid`. The `IDirect3DStateBlock9` pointer is not modelled.
#[derive(Default)]
struct Slot {
    open: bool,
    invalid: bool,
}

/// The device handle table of `struct device_manager`: the device token, whether a device
/// has been reset in, and the handle slots.
pub struct DeviceHandles {
    token: u32,
    has_device: bool,
    slots: Vec<Slot>,
}

impl DeviceHandles {
    /// `DXVA2CreateDirect3DDeviceManager9`: a manager with no device and no handles.
    pub fn new(token: u32) -> Self {
        Self {
            token,
            has_device: false,
            slots: Vec::new(),
        }
    }

    /// `ResetDevice`. A token that does not match returns `E_INVALIDARG` and changes nothing.
    /// A reset after the first one marks every slot invalid. The device pointer itself is not
    /// modelled, and neither is the lock, which the C body also clears here.
    pub fn reset_device(&mut self, token: u32) -> Result<(), HRESULT> {
        if token != self.token {
            return Err(E_INVALIDARG);
        }

        if self.has_device {
            for slot in &mut self.slots {
                slot.invalid = true;
            }
        }
        self.has_device = true;

        Ok(())
    }

    /// `OpenDeviceHandle`. Without a device, returns `DXVA2_E_NOT_INITIALIZED`.
    ///
    /// The first closed slot gets its open flag set, but the C body then appends a new slot and
    /// returns its handle. The port keeps this as it is in C, so a reused slot stays open next
    /// to the new one.
    pub fn open_device_handle(&mut self) -> Result<Handle, HRESULT> {
        if !self.has_device {
            return Err(DXVA2_E_NOT_INITIALIZED);
        }

        if let Some(slot) = self.slots.iter_mut().find(|slot| !slot.open) {
            slot.open = true;
        }
        self.slots.push(Slot {
            open: true,
            invalid: false,
        });

        Ok(self.slots.len())
    }

    /// `CloseDeviceHandle`. Closing a closed or unknown handle returns `E_HANDLE`. Closing the
    /// last slot removes it from the table, but only that one slot, as the C count does.
    pub fn close_device_handle(&mut self, handle: Handle) -> Result<(), HRESULT> {
        let index = self.index(handle)?;
        if !self.slots[index].open {
            return Err(E_HANDLE);
        }

        self.slots[index] = Slot::default();
        if index + 1 == self.slots.len() {
            self.slots.pop();
        }

        Ok(())
    }

    /// The check that `TestDevice` and `GetVideoService` both make. An invalid slot returns
    /// `DXVA2_E_NEW_VIDEO_DEVICE`, and a closed slot returns `E_HANDLE`.
    pub fn check_handle(&self, handle: Handle) -> Result<(), HRESULT> {
        let slot = &self.slots[self.index(handle)?];

        if slot.invalid {
            Err(DXVA2_E_NEW_VIDEO_DEVICE)
        } else if !slot.open {
            Err(E_HANDLE)
        } else {
            Ok(())
        }
    }

    fn index(&self, handle: Handle) -> Result<usize, HRESULT> {
        if handle == 0 || handle > self.slots.len() {
            return Err(E_HANDLE);
        }

        Ok(handle - 1)
    }
}
