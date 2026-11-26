//! The `IAsyncInfo` state machine from Wine's `dlls/coremessaging/async.c`.
//!
//! The COM plumbing is not modelled. A handler is a [`Handler`] marker instead of a
//! pointer, and invoking it is reported through return values instead of a call.

use crate::{
    failed, Hresult, E_ILLEGAL_DELEGATE_ASSIGNMENT, E_ILLEGAL_METHOD_CALL, E_ILLEGAL_STATE_CHANGE,
    STATUS_PENDING, S_OK,
};

/// `AsyncStatus`, plus Wine's private `Closed` value (`#define Closed 4`), which marks
/// an operation that has been closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsyncStatus {
    Started = 0,
    Completed = 1,
    Canceled = 2,
    Error = 3,
    Closed = 4,
}

/// What an async callback produced: its `HRESULT`, and the `VT_BOOL` value it wrote
/// (`None` leaves the `PROPVARIANT` empty).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackResult {
    pub hr: Hresult,
    pub value: Option<bool>,
}

/// `async_operation_callback` without the invoker and parameter objects.
pub type Callback = fn(called_async: bool) -> CallbackResult;

/// The completed handler slot. `HANDLER_NOT_SET` is `None` in [`AsyncInfo`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Handler {
    Null,
    Set,
}

/// Outcome of one run of `async_info_complete`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Completion {
    /// The callback is still pending, so `Start` submits the work to the threadpool.
    Pending,
    /// The callback finished. `invoked` is the status the completed handler was
    /// invoked with, or `None` if no handler was set.
    Done { invoked: Option<AsyncStatus> },
}

/// State behind one `IAsyncInfo` (`struct async_info`).
pub struct AsyncInfo {
    callback: Callback,
    status: AsyncStatus,
    handler: Option<Handler>,
    value: Option<bool>,
    hr: Hresult,
}

impl AsyncInfo {
    /// `async_info_create`: status `Started`, handler not set.
    pub fn new(callback: Callback) -> Self {
        Self {
            callback,
            status: AsyncStatus::Started,
            handler: None,
            value: None,
            hr: S_OK,
        }
    }

    /// `async_impl_Start`: runs the callback synchronously. On `Pending`, the caller
    /// must later run `complete(true)` on the threadpool.
    pub fn start(&mut self) -> Completion {
        self.complete(false)
    }

    /// `async_info_complete`: runs the callback and records its result.
    pub fn complete(&mut self, called_async: bool) -> Completion {
        let CallbackResult { hr, value } = (self.callback)(called_async);
        // Only the synchronous call can leave the operation pending. A threadpool call
        // that returns STATUS_PENDING completes with that value as its hr.
        if !called_async && hr == STATUS_PENDING {
            return Completion::Pending;
        }
        if self.status != AsyncStatus::Closed {
            self.status = if failed(hr) {
                AsyncStatus::Error
            } else {
                AsyncStatus::Completed
            };
        }
        self.value = value;
        self.hr = hr;
        let invoked = match self.handler {
            Some(Handler::Set) => {
                self.handler = Some(Handler::Null);
                Some(self.status)
            }
            _ => None,
        };
        Completion::Done { invoked }
    }

    /// `async_impl_put_Completed`. If the operation has already finished, the handler is
    /// invoked at once and its status is returned.
    pub fn put_completed(&mut self, handler: Handler) -> Result<Option<AsyncStatus>, Hresult> {
        if self.status == AsyncStatus::Closed {
            return Err(E_ILLEGAL_METHOD_CALL);
        }
        if self.handler.is_some() {
            return Err(E_ILLEGAL_DELEGATE_ASSIGNMENT);
        }
        self.handler = Some(handler);
        if handler == Handler::Set && self.status != AsyncStatus::Started {
            self.handler = Some(Handler::Null);
            return Ok(Some(self.status));
        }
        Ok(None)
    }

    /// `async_impl_get_Completed`. Returns whether a handler is set; the handler pointer
    /// itself is not modelled.
    pub fn get_completed(&self) -> (Hresult, bool) {
        (self.open_hr(), self.handler == Some(Handler::Set))
    }

    /// `async_impl_get_Result` as seen by `GetResults`: the result is available only once
    /// completed or errored, and `results` is the boolean value (0 when there is none).
    pub fn get_results(&self) -> (Hresult, bool) {
        match self.status {
            AsyncStatus::Completed | AsyncStatus::Error => (self.hr, self.value.unwrap_or(false)),
            _ => (E_ILLEGAL_METHOD_CALL, false),
        }
    }

    /// `async_info_get_Id`: every operation reports id 1.
    pub fn get_id(&self) -> (Hresult, u32) {
        (self.open_hr(), 1)
    }

    /// `async_info_get_Status`: the status is returned even when the call fails.
    pub fn get_status(&self) -> (Hresult, AsyncStatus) {
        (self.open_hr(), self.status)
    }

    /// `async_info_get_ErrorCode`: a closed operation fails both the call and the out value.
    pub fn get_error_code(&self) -> (Hresult, Hresult) {
        if self.status == AsyncStatus::Closed {
            (E_ILLEGAL_METHOD_CALL, E_ILLEGAL_METHOD_CALL)
        } else {
            (S_OK, self.hr)
        }
    }

    /// `async_info_Cancel`: only a started operation becomes `Canceled`.
    pub fn cancel(&mut self) -> Hresult {
        let hr = self.open_hr();
        if self.status == AsyncStatus::Started {
            self.status = AsyncStatus::Canceled;
        }
        hr
    }

    /// `async_info_Close`: a started operation cannot be closed. Closing twice succeeds.
    pub fn close(&mut self) -> Hresult {
        match self.status {
            AsyncStatus::Started => E_ILLEGAL_STATE_CHANGE,
            AsyncStatus::Closed => S_OK,
            _ => {
                self.status = AsyncStatus::Closed;
                S_OK
            }
        }
    }

    fn open_hr(&self) -> Hresult {
        if self.status == AsyncStatus::Closed {
            E_ILLEGAL_METHOD_CALL
        } else {
            S_OK
        }
    }
}
