use crate::codes::*;
use crate::data::{Data, TwEvent};
use crate::devices::{select_device, selectable_identities, Device};
use crate::dispatch::{source_manager_op, SourceManagerOp};
use crate::identity::Identity;
use crate::source::{find_source, ActiveDs, Callback, DataSource};

/// The environment the Source Manager calls out to: the Win32 loader, the application's
/// callback, the message window APIs, and the user selection dialog.
pub trait Host {
    type Source: DataSource;

    /// Loads the source module for a device (`LoadLibraryW` and `GetProcAddress("DS_Entry")`).
    /// `None` means the module could not be loaded or has no entry point.
    fn load_source(&mut self, modname: &str) -> Option<Self::Source>;

    /// Calls the application callback registered with `MSG_REGISTER_CALLBACK` and returns its
    /// result. `dest_id` is the `Id` of `pDest`, or `None` when it is NULL.
    fn call_app_callback(
        &mut self,
        proc_addr: usize,
        ref_con: usize,
        origin: &Identity,
        dest_id: Option<u32>,
        msg: u16,
    ) -> u16;

    /// `RegisterWindowMessageA("WINE TWAIN_32 EVENT")`.
    fn register_event_message(&mut self) -> u32;

    /// `PeekMessageW(..., PM_NOREMOVE)` for `message` on `hwnd`: whether such a message is queued.
    fn peek_event_message(&mut self, hwnd: usize, message: u32) -> bool;

    /// Shows the selection dialog over `choices` for `parent` (0 for no parent). Returns the index
    /// into `choices` the user picked, or `None` when the dialog is cancelled.
    fn user_select(&mut self, parent: usize, choices: &[Identity]) -> Option<usize>;
}

/// The Source Manager state of `dlls/twaindsm/dsm_ctrl.c`, with `DSM_Entry` routing from
/// `dlls/twaindsm/twain32_main.c`.
///
/// The list of open sources is `sources`, with the head (the most recently opened source) at
/// index 0, as in the C linked list. Window messages the C code posts are recorded in `posted`
/// for the host to deliver.
pub struct Dsm<H: Host> {
    host: H,
    twcc: u16,
    initialized: bool,
    source_id: u32,
    current_device: usize,
    parent: usize,
    event_message: u32,
    devices: Vec<Device>,
    sources: Vec<ActiveDs<H::Source>>,
    /// `bProcessingCallback` in `TWAIN_ControlNull`: set while the application callback runs.
    pub(crate) processing_callback: bool,
    posted: Vec<usize>,
}

impl<H: Host> Dsm<H> {
    /// A Source Manager that is not initialized, with `devices` as the device list.
    pub fn new(host: H, devices: Vec<Device>) -> Self {
        Self {
            host,
            twcc: TWCC_SUCCESS,
            initialized: false,
            source_id: 0,
            current_device: 0,
            parent: 0,
            event_message: 0,
            devices,
            sources: Vec::new(),
            processing_callback: false,
            posted: Vec::new(),
        }
    }

    /// `DSM_Entry`: routes one TWAIN call.
    pub fn entry(
        &mut self,
        origin: &mut Identity,
        dest: Option<&Identity>,
        dg: u32,
        dat: u16,
        msg: u16,
        data: Data<'_>,
    ) -> u16 {
        if dg == DG_CONTROL && dat == DAT_NULL {
            let Some(index) = find_source(&self.sources, origin.id) else {
                return self.fail(TWCC_BADPROTOCOL);
            };
            return self.control_null(origin, dest, index, msg);
        }
        if let Some(dest) = dest {
            let Some(index) = find_source(&self.sources, dest.id) else {
                return self.fail(TWCC_BADDEST);
            };
            return self.forward(origin, index, dg, dat, msg, data);
        }
        if dg == DG_CONTROL {
            self.source_manager(origin, dat, msg, data)
        } else {
            self.fail(TWCC_BADPROTOCOL)
        }
    }

    pub fn condition_code(&self) -> u16 {
        self.twcc
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn parent(&self) -> usize {
        self.parent
    }

    pub fn event_message(&self) -> u32 {
        self.event_message
    }

    pub fn next_source_id(&self) -> u32 {
        self.source_id
    }

    /// Open sources in list order, head first.
    pub fn sources(&self) -> &[ActiveDs<H::Source>] {
        &self.sources
    }

    pub fn sources_mut(&mut self) -> &mut [ActiveDs<H::Source>] {
        &mut self.sources
    }

    /// Window messages posted so far, as the target window handles (0 is the thread queue).
    pub fn posted_messages(&self) -> &[usize] {
        &self.posted
    }

    /// Adds an open source at the head of the list, as `OpenDS` does.
    pub fn adopt_source(&mut self, identity: Identity, source: H::Source) {
        self.sources.insert(0, ActiveDs::new(identity, source));
    }

    fn fail(&mut self, twcc: u16) -> u16 {
        self.twcc = twcc;
        TWRC_FAILURE
    }

    fn succeed(&mut self) -> u16 {
        self.twcc = TWCC_SUCCESS;
        TWRC_SUCCESS
    }

    /// `TWAIN_ControlNull`: queues an application message and posts to the windows that might
    /// deliver it, or calls the registered application callback.
    fn control_null(
        &mut self,
        origin: &Identity,
        dest: Option<&Identity>,
        index: usize,
        msg: u16,
    ) -> u16 {
        if !is_control_null_msg(msg) {
            return self.fail(TWCC_BADPROTOCOL);
        }

        let callback = &self.sources[index].callback;
        if callback.proc_addr != 0 {
            if self.processing_callback {
                return self.fail(TWCC_BADPROTOCOL);
            }
            self.processing_callback = true;
            let rc = self.host.call_app_callback(
                callback.proc_addr,
                callback.ref_con,
                origin,
                dest.map(|dest| dest.id),
                msg,
            );
            self.processing_callback = false;
            // The C code compares the callback's TWRC with TWCC_SUCCESS. Both are 0, and the
            // comparison is kept as written.
            if rc != TWCC_SUCCESS {
                self.twcc = TWCC_BADPROTOCOL;
            }
            return rc;
        }

        let source = &mut self.sources[index];
        source.pending.push_back(msg);
        let ui_window = source.ui_window;
        let event_window = source.event_window;
        let parent = self.parent;

        // A data source may forward messages sent to the DSM parent or to the window given to
        // ENABLEDS, so try each window once.
        if parent != 0 {
            self.posted.push(parent);
        }
        if ui_window != 0 && ui_window != parent {
            self.posted.push(ui_window);
        }
        if event_window != 0 && event_window != ui_window && event_window != parent {
            self.posted.push(event_window);
        }
        self.posted.push(0);
        TWRC_SUCCESS
    }

    /// `TWAIN_ProcessEvent`: answers a `MSG_PROCESSEVENT` for the event message with the next
    /// queued TWAIN message, or `MSG_NULL`.
    fn process_event(&mut self, index: usize, event: &mut TwEvent) -> u16 {
        let result = if event.msg.message == self.event_message {
            event.tw_message = self.sources[index].pending.pop_front().unwrap_or(MSG_NULL);
            TWRC_DSEVENT
        } else {
            TWRC_NOTDSEVENT
        };

        let hwnd = event.msg.hwnd;
        if hwnd != 0 {
            let source = &mut self.sources[index];
            source.event_window = hwnd;
            if !source.pending.is_empty() && !self.host.peek_event_message(hwnd, self.event_message)
            {
                self.posted.push(hwnd);
            }
        }
        result
    }

    /// The tail of `DSM_Entry` for a destination source: `MSG_PROCESSEVENT` first, then the
    /// window and callback bookkeeping, then the forward to the source.
    fn forward(
        &mut self,
        origin: &Identity,
        index: usize,
        dg: u32,
        dat: u16,
        msg: u16,
        mut data: Data<'_>,
    ) -> u16 {
        if dg == DG_CONTROL && dat == DAT_EVENT && msg == MSG_PROCESSEVENT {
            let Data::Event(event) = &mut data else {
                return self.fail(TWCC_BADVALUE);
            };
            if self.process_event(index, event) == TWRC_DSEVENT {
                return TWRC_DSEVENT;
            }
        }

        if dg == DG_CONTROL
            && dat == DAT_USERINTERFACE
            && (msg == MSG_ENABLEDS || msg == MSG_ENABLEDSUIONLY)
        {
            if let Data::UserInterface { hparent } = &data {
                self.sources[index].ui_window = *hparent;
            }
        }

        if dg == DG_CONTROL
            && msg == MSG_REGISTER_CALLBACK
            && (dat == DAT_CALLBACK || dat == DAT_CALLBACK2)
        {
            if let Data::Callback { proc_addr, ref_con } = data {
                self.sources[index].callback = Callback { proc_addr, ref_con };
                return self.succeed();
            }
        }

        self.twcc = TWCC_SUCCESS;
        self.sources[index]
            .source
            .entry(Some(origin), dg, dat, msg, data)
    }

    /// `TWAIN_SourceManagerHandler`: the `DG_CONTROL` operations with no destination source.
    fn source_manager(&mut self, origin: &mut Identity, dat: u16, msg: u16, data: Data<'_>) -> u16 {
        let Some(op) = source_manager_op(dat, msg) else {
            return self.fail(TWCC_BADPROTOCOL);
        };
        match op {
            SourceManagerOp::CloseDs => match data {
                Data::Identity(identity) => self.close_ds(origin, identity),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::GetDefault => match data {
                Data::Identity(out) => self.identity_get_default(out),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::GetFirst => match data {
                Data::Identity(out) => self.identity_get_first(out),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::GetNext => match data {
                Data::Identity(out) => self.identity_get_next(out),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::OpenDs => match data {
                Data::Identity(identity) => self.open_ds(origin, identity),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::UserSelect => match data {
                Data::Identity(result) => self.identity_user_select(origin, result),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::CloseDsm => match parent_handle(&data) {
                Some(hwnd) => self.close_dsm(origin, hwnd),
                None => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::OpenDsm => match parent_handle(&data) {
                Some(hwnd) => self.open_dsm(origin, hwnd),
                None => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::GetEntrypoint => match data {
                Data::Entrypoint { size } if size >= TW_ENTRYPOINT_SIZE => self.succeed(),
                _ => self.fail(TWCC_BADVALUE),
            },
            SourceManagerOp::GetDsmStatus => match data {
                Data::Status(status) => {
                    *status = self.twcc;
                    self.succeed()
                }
                _ => self.fail(TWCC_BADVALUE),
            },
        }
    }

    /// `TWAIN_OpenDSM`: the first call registers the event message and sets the parent window.
    fn open_dsm(&mut self, origin: &mut Identity, hwnd: usize) -> u16 {
        let rc = if self.initialized {
            self.fail(TWCC_SEQERROR)
        } else {
            self.event_message = self.host.register_event_message();
            if (origin.supported_groups & DF_APP2) != 0 {
                origin.supported_groups |= DF_DSM2;
            }
            self.current_device = 0;
            self.initialized = true;
            self.succeed()
        };
        self.parent = hwnd;
        rc
    }

    /// `TWAIN_CloseDSM`: closes every open source, head first, and clears the list.
    fn close_dsm(&mut self, origin: &Identity, hwnd: usize) -> u16 {
        if !self.initialized {
            return self.fail(TWCC_SEQERROR);
        }
        self.initialized = false;
        for source in &mut self.sources {
            source.source.entry(
                Some(origin),
                DG_CONTROL,
                DAT_IDENTITY,
                MSG_CLOSEDS,
                Data::Parent(hwnd),
            );
        }
        self.sources.clear();
        self.parent = 0;
        self.succeed()
    }

    /// `TWAIN_OpenDS`: loads the device, assigns the source `Id`, and opens it.
    fn open_ds(&mut self, origin: &Identity, identity: &mut Identity) -> u16 {
        if !self.initialized {
            return self.fail(TWCC_SEQERROR);
        }
        if self.devices.is_empty() {
            return self.fail(TWCC_NODS);
        }
        let index = select_device(&self.devices, &identity.product_name);
        let Some(mut source) = self.host.load_source(&self.devices[index].modname) else {
            return self.fail(TWCC_OPERATIONERROR);
        };

        identity.id = self.source_id;
        self.source_id += 1;
        if source.entry(
            None,
            DG_CONTROL,
            DAT_IDENTITY,
            MSG_GET,
            Data::Identity(&mut *identity),
        ) != TWRC_SUCCESS
        {
            self.source_id -= 1;
            return self.fail(TWCC_OPERATIONERROR);
        }
        if identity.has_ds2() {
            source.entry(
                Some(origin),
                DG_CONTROL,
                DAT_ENTRYPOINT,
                MSG_SET,
                Data::Entrypoint {
                    size: TW_ENTRYPOINT_SIZE,
                },
            );
        }
        if source.entry(
            Some(origin),
            DG_CONTROL,
            DAT_IDENTITY,
            MSG_OPENDS,
            Data::Identity(&mut *identity),
        ) != TWRC_SUCCESS
        {
            self.source_id -= 1;
            return self.fail(TWCC_OPERATIONERROR);
        }

        self.sources
            .insert(0, ActiveDs::new(identity.active_copy(), source));
        self.succeed()
    }

    /// `TWAIN_CloseDS`: closes the source with `identity.Id` and removes it from the list.
    fn close_ds(&mut self, origin: &Identity, identity: &mut Identity) -> u16 {
        let Some(index) = find_source(&self.sources, identity.id) else {
            return self.fail(TWCC_NODS);
        };
        let rc = self.sources[index].source.entry(
            Some(origin),
            DG_CONTROL,
            DAT_IDENTITY,
            MSG_CLOSEDS,
            Data::Identity(identity),
        );
        self.sources.remove(index);
        self.twcc = if rc == TWRC_SUCCESS {
            TWCC_SUCCESS
        } else {
            TWCC_SEQERROR
        };
        rc
    }

    /// `TWAIN_IdentityGetDefault`: the first device.
    fn identity_get_default(&mut self, out: &mut Identity) -> u16 {
        let Some(device) = self.devices.first() else {
            return self.fail(TWCC_NODS);
        };
        *out = device.identity.clone();
        self.succeed()
    }

    /// `TWAIN_IdentityGetFirst`: the first device, and the cursor moves past it.
    fn identity_get_first(&mut self, out: &mut Identity) -> u16 {
        let Some(device) = self.devices.first() else {
            return self.fail(TWCC_NODS);
        };
        *out = device.identity.clone();
        self.current_device = 1;
        TWRC_SUCCESS
    }

    /// `TWAIN_IdentityGetNext`: the next device, or `TWRC_ENDOFLIST` past the last one.
    fn identity_get_next(&mut self, out: &mut Identity) -> u16 {
        let Some(device) = self.devices.get(self.current_device) else {
            self.twcc = TWCC_SUCCESS;
            return TWRC_ENDOFLIST;
        };
        *out = device.identity.clone();
        self.current_device += 1;
        TWRC_SUCCESS
    }

    /// `TWAIN_UserSelect`: offers the devices that share a group with the application and writes
    /// the one the user picks. The dialog itself is the host's `user_select`.
    fn identity_user_select(&mut self, origin: &Identity, result: &mut Identity) -> u16 {
        let mut choices = selectable_identities(&self.devices, origin.supported_groups);
        let parent = self.parent;
        match self.host.user_select(parent, &choices) {
            Some(index) => {
                *result = choices.swap_remove(index);
                self.succeed()
            }
            None => {
                self.twcc = TWCC_SUCCESS;
                TWRC_CANCEL
            }
        }
    }
}

/// The parent window handle carried by `DAT_PARENT` operations. A NULL `pData` is handle 0.
fn parent_handle(data: &Data<'_>) -> Option<usize> {
    match data {
        Data::Null => Some(0),
        Data::Parent(hwnd) => Some(*hwnd),
        _ => None,
    }
}
