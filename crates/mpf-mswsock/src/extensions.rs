/// A `GUID`, the key that `SIO_GET_EXTENSION_FUNCTION_POINTER` looks a function up by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

/// `WSAID_ACCEPTEX` from `include/mswsock.h`.
pub const WSAID_ACCEPTEX: Guid = Guid {
    data1: 0xb536_7df1,
    data2: 0xcbac,
    data3: 0x11cf,
    data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
};

/// `WSAID_GETACCEPTEXSOCKADDRS` from `include/mswsock.h`.
pub const WSAID_GETACCEPTEXSOCKADDRS: Guid = Guid {
    data1: 0xb536_7df2,
    data2: 0xcbac,
    data3: 0x11cf,
    data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
};

/// `WSAID_TRANSMITFILE` from `include/mswsock.h`.
pub const WSAID_TRANSMITFILE: Guid = Guid {
    data1: 0xb536_7df0,
    data2: 0xcbac,
    data3: 0x11cf,
    data4: [0x95, 0xca, 0x00, 0x80, 0x5f, 0x48, 0xa1, 0x92],
};

/// The statics that `dlls/mswsock/mswsock.c` keeps for the extension pointers: `acceptex_fn`,
/// `acceptexsockaddrs_fn`, `transmitfile_fn` and `initialised`. `F` is a resolved pointer.
/// `Default` is the initial state, where every pointer is NULL and nothing is initialised.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extensions<F> {
    initialised: bool,
    acceptex: Option<F>,
    acceptexsockaddrs: Option<F>,
    transmitfile: Option<F>,
}

impl<F> Default for Extensions<F> {
    fn default() -> Self {
        Self {
            initialised: false,
            acceptex: None,
            acceptexsockaddrs: None,
            transmitfile: None,
        }
    }
}

impl<F: Copy> Extensions<F> {
    /// Whether the pointers have been looked up, which `AcceptEx` and `TransmitFile` do once.
    pub fn initialised(&self) -> bool {
        self.initialised
    }

    /// `AcceptEx`. The first call looks the pointers up through `ioctl` with `acceptor`. Then it
    /// calls the `AcceptEx` pointer through `call`, or returns FALSE when that pointer is empty.
    pub fn accept_ex<S: Copy>(
        &mut self,
        acceptor: S,
        ioctl: impl FnMut(S, &Guid) -> Option<F>,
        call: impl FnOnce(F) -> bool,
    ) -> bool {
        if !self.initialised {
            self.get_fn_pointers(acceptor, ioctl);
        }
        match self.acceptex {
            Some(f) => call(f),
            None => false,
        }
    }

    /// `GetAcceptExSockaddrs`. It calls the pointer through `call` when it is set. It never looks
    /// the pointers up, so it does not set `initialised`.
    pub fn get_accept_ex_sockaddrs(&self, call: impl FnOnce(F)) {
        if let Some(f) = self.acceptexsockaddrs {
            call(f);
        }
    }

    /// `TransmitFile`. It works like [`Extensions::accept_ex`], and its first call looks up with `s`.
    pub fn transmit_file<S: Copy>(
        &mut self,
        s: S,
        ioctl: impl FnMut(S, &Guid) -> Option<F>,
        call: impl FnOnce(F) -> bool,
    ) -> bool {
        if !self.initialised {
            self.get_fn_pointers(s, ioctl);
        }
        match self.transmitfile {
            Some(f) => call(f),
            None => false,
        }
    }

    /// `get_fn_pointers` and its `get_fn` helper. The lookups run in the C order: `AcceptEx`,
    /// `GetAcceptExSockaddrs`, `TransmitFile`. A failed lookup leaves its slot empty. The C code
    /// stores a NULL result only when `WSAIoctl` succeeds, and a slot starts out NULL, so `None`
    /// here covers both cases.
    fn get_fn_pointers<S: Copy>(&mut self, s: S, mut ioctl: impl FnMut(S, &Guid) -> Option<F>) {
        if let Some(f) = ioctl(s, &WSAID_ACCEPTEX) {
            self.acceptex = Some(f);
        }
        if let Some(f) = ioctl(s, &WSAID_GETACCEPTEXSOCKADDRS) {
            self.acceptexsockaddrs = Some(f);
        }
        if let Some(f) = ioctl(s, &WSAID_TRANSMITFILE) {
            self.transmitfile = Some(f);
        }
        self.initialised = true;
    }
}
