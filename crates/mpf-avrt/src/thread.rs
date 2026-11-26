use crate::error::AvrtError;

/// The `HANDLE` that the `AvSetMm*ThreadCharacteristics*W` stubs return on success.
pub const AVRT_STUB_HANDLE: usize = 0x1234_5678;

/// One ANSI argument after `strdupAW`. A NULL pointer stays NULL, a converted string is
/// passed on as wide characters, and a failed `malloc` is reported as `OutOfMemory`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnsiArg<'a> {
    Null,
    Converted(&'a [u16]),
    OutOfMemory,
}

impl<'a> AnsiArg<'a> {
    /// The `if (x && !(xW = strdupAW(x)))` check of the ANSI wrappers.
    fn wide(self) -> Result<Option<&'a [u16]>, AvrtError> {
        match self {
            AnsiArg::Null => Ok(None),
            AnsiArg::Converted(wide) => Ok(Some(wide)),
            AnsiArg::OutOfMemory => Err(AvrtError::OutOfMemory),
        }
    }
}

/// `AvSetMmThreadCharacteristicsW`. `has_index` says whether the `TaskIndex` pointer is
/// non-NULL; the stub only checks it and never writes through it.
pub fn av_set_mm_thread_characteristics_w(
    name: Option<&[u16]>,
    has_index: bool,
) -> Result<usize, AvrtError> {
    if name.is_none() {
        return Err(AvrtError::InvalidTaskName);
    }
    if !has_index {
        return Err(AvrtError::InvalidHandle);
    }
    Ok(AVRT_STUB_HANDLE)
}

/// `AvSetMmThreadCharacteristicsA`: converts the name, then calls the W variant.
pub fn av_set_mm_thread_characteristics_a(
    name: AnsiArg<'_>,
    has_index: bool,
) -> Result<usize, AvrtError> {
    av_set_mm_thread_characteristics_w(name.wide()?, has_index)
}

/// `AvSetMmMaxThreadCharacteristicsW`. The first task is required and the second must be absent.
pub fn av_set_mm_max_thread_characteristics_w(
    task1: Option<&[u16]>,
    task2: Option<&[u16]>,
    has_index: bool,
) -> Result<usize, AvrtError> {
    if task1.is_none() || task2.is_some() {
        return Err(AvrtError::InvalidTaskName);
    }
    if !has_index {
        return Err(AvrtError::InvalidHandle);
    }
    Ok(AVRT_STUB_HANDLE)
}

/// `AvSetMmMaxThreadCharacteristicsA`: converts task1, then task2, then calls the W variant.
pub fn av_set_mm_max_thread_characteristics_a(
    task1: AnsiArg<'_>,
    task2: AnsiArg<'_>,
    has_index: bool,
) -> Result<usize, AvrtError> {
    let task1 = task1.wide()?;
    let task2 = task2.wide()?;
    av_set_mm_max_thread_characteristics_w(task1, task2, has_index)
}
