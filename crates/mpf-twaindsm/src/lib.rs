//! Port of the portable logic in Wine's `dlls/twaindsm/dsm_ctrl.c` and the `DSM_Entry` routing in
//! `dlls/twaindsm/twain32_main.c`, with the export table from `dlls/twaindsm/twaindsm.spec`.
//!
//! Ported:
//! - The `DSM_Entry` routing: `DG_CONTROL/DAT_NULL` by origin `Id`, the destination lookup
//!   (`TWCC_BADDEST`), the `MSG_PROCESSEVENT`, `ENABLEDS`/`ENABLEDSUIONLY` and
//!   `REGISTER_CALLBACK` bookkeeping, and the forward to the source: [`Dsm::entry`]
//! - The `DG_CONTROL` operation table (`TWAIN_SourceManagerHandler`): [`source_manager_op`]
//! - The Source Manager state: `OpenDSM`/`CloseDSM`, `OpenDS`/`CloseDS` with the source `Id`
//!   counter, the identity getters, `UserSelect`, `GetDSMStatus`, `GetEntrypoint` and the
//!   condition code: [`Dsm`], [`Identity`], [`Device`]
//! - `TWAIN_ControlNull` (the message queue, the window posts, the application callback) and
//!   `TWAIN_ProcessEvent`: [`Dsm::entry`], [`is_control_null_msg`]
//! - Device selection by product name and the user-select filter: [`select_device`],
//!   [`selectable_identities`]
//! - The TWAIN constants and `TW_ENTRYPOINT` size used by the code above: [`codes`]
//!
//! Modelled through the [`Host`] and [`DataSource`] traits:
//! - Loading a source module: `LoadLibraryW` and `GetProcAddress("DS_Entry")` are
//!   [`Host::load_source`]. A module that fails to load or has no `DS_Entry` returns `None`.
//! - The application callback: [`Host::call_app_callback`]
//! - `RegisterWindowMessageA` and `PeekMessageW`: [`Host::register_event_message`] and
//!   [`Host::peek_event_message`]
//! - `PostMessageW`: the target windows are recorded in [`Dsm::posted_messages`] for the host
//!   to deliver.
//! - The selection dialog: [`Host::user_select`]. The dialog's listbox and `IsWindow` check
//!   are the host's job.
//!
//! Skipped:
//! - Device autodetect (`twain_autodetect`, the registry scan and the `twain_*.ds` directory
//!   walk): the device list is passed to [`Dsm::new`].
//! - `HeapAlloc` failure paths (`TWCC_LOWMEMORY`): the port has no heap to exhaust.
//! - `FreeLibrary` and the `twain_32.dll` load for version 1 sources, which are in
//!   `#ifndef WIN64` blocks and only affect the loader.
//! - The `memcpy` of the entrypoint table in `TWAIN_GetEntrypoint`: only the size check is
//!   modelled.
//! - The contents of data the port does not read (`Data::Opaque`): it is forwarded to the source
//!   unchanged.
//! - `DSM_hinstance`, `DllMain` and the `DSM_Entry` export wrapper.
//!
//! Deliberate deviations (C would crash on these inputs, the port reports an error instead):
//! - An identity operation with a NULL or mismatched `pData` returns `TWRC_FAILURE` with
//!   `TWCC_BADVALUE`. The same applies to `MSG_PROCESSEVENT` without a `TW_EVENT`, to
//!   `MSG_GET` on `DAT_STATUS` and `DAT_ENTRYPOINT` without the matching structure, and to
//!   `DAT_PARENT` with data other than a handle or NULL.
//! - `Dsm::entry` takes a non-NULL `origin`; the C code dereferences `pOrigin` on every path
//!   that reads it.

mod codes;
mod data;
mod devices;
mod dispatch;
mod dsm;
mod identity;
mod source;

pub use codes::{
    is_control_null_msg, DAT_CALLBACK, DAT_CALLBACK2, DAT_ENTRYPOINT, DAT_EVENT, DAT_IDENTITY,
    DAT_NULL, DAT_PARENT, DAT_STATUS, DAT_USERINTERFACE, DF_APP2, DF_DS2, DF_DSM2, DG_CONTROL,
    MSG_CLOSEDS, MSG_CLOSEDSM, MSG_CLOSEDSREQ, MSG_DEVICEEVENT, MSG_ENABLEDS, MSG_ENABLEDSUIONLY,
    MSG_GET, MSG_GETDEFAULT, MSG_GETFIRST, MSG_GETNEXT, MSG_NULL, MSG_OPENDS, MSG_OPENDSM,
    MSG_PROCESSEVENT, MSG_REGISTER_CALLBACK, MSG_SET, MSG_USERSELECT, MSG_XFERREADY, TWCC_BADDEST,
    TWCC_BADPROTOCOL, TWCC_BADVALUE, TWCC_NODS, TWCC_OPERATIONERROR, TWCC_SEQERROR, TWCC_SUCCESS,
    TWRC_CANCEL, TWRC_DSEVENT, TWRC_ENDOFLIST, TWRC_FAILURE, TWRC_NOTDSEVENT, TWRC_SUCCESS,
    TW_ENTRYPOINT_SIZE,
};
pub use data::{Data, Msg, TwEvent};
pub use devices::{select_device, selectable_identities, Device};
pub use dispatch::{source_manager_op, SourceManagerOp};
pub use dsm::{Dsm, Host};
pub use identity::Identity;
pub use source::{find_source, ActiveDs, Callback, DataSource};

#[cfg(test)]
mod test_host;

#[cfg(test)]
mod oracle_checks;

#[cfg(test)]
mod edge_checks;
