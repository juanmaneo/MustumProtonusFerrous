use crate::identity::Identity;

/// The `pData` argument of `DSM_Entry`, typed by the operation that uses it.
///
/// `Null` is a NULL pointer. `Opaque` is data the port does not inspect (it is only forwarded to
/// a data source). Any other variant where the operation expects a different one is rejected
/// with `TWCC_BADVALUE`, except for the optional ENABLEDS and REGISTER_CALLBACK arguments, which
/// are forwarded unchanged when they do not match.
pub enum Data<'a> {
    Null,
    Identity(&'a mut Identity),
    Status(&'a mut u16),
    Entrypoint { size: u32 },
    Event(&'a mut TwEvent),
    UserInterface { hparent: usize },
    Callback { proc_addr: usize, ref_con: usize },
    Parent(usize),
    Opaque,
}

/// The `MSG` structure a `TW_EVENT` points at: the window message and its target window.
pub struct Msg {
    pub message: u32,
    pub hwnd: usize,
}

/// The `TW_EVENT` structure: the event message and the TWAIN message written back for it.
pub struct TwEvent {
    pub msg: Msg,
    pub tw_message: u16,
}
