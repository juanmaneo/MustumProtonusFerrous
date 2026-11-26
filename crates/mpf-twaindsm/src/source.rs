use crate::data::Data;
use crate::identity::Identity;
use std::collections::VecDeque;

/// A loaded data source (`activeDS::dsEntry`): receives the operations the Source Manager
/// forwards to it. `origin` is `None` only for the `MSG_GET` issued while opening a source.
pub trait DataSource {
    fn entry(
        &mut self,
        origin: Option<&Identity>,
        dg: u32,
        dat: u16,
        msg: u16,
        data: Data<'_>,
    ) -> u16;
}

/// The application callback registered with `MSG_REGISTER_CALLBACK`. `proc_addr == 0` means
/// no callback is registered.
#[derive(Debug, Default)]
pub struct Callback {
    pub proc_addr: usize,
    pub ref_con: usize,
}

/// An open data source (`activeDS`): its identity, the loaded source, the TWAIN messages queued
/// for the application, and the windows the Source Manager posts to.
pub struct ActiveDs<S> {
    pub identity: Identity,
    pub source: S,
    pub pending: VecDeque<u16>,
    pub ui_window: usize,
    pub event_window: usize,
    pub callback: Callback,
}

impl<S> ActiveDs<S> {
    pub fn new(identity: Identity, source: S) -> Self {
        Self {
            identity,
            source,
            pending: VecDeque::new(),
            ui_window: 0,
            event_window: 0,
            callback: Callback::default(),
        }
    }
}

/// Index of the first source in list order whose `Id` matches. The list head is index 0.
pub fn find_source<S>(sources: &[ActiveDs<S>], id: u32) -> Option<usize> {
    sources.iter().position(|source| source.identity.id == id)
}
