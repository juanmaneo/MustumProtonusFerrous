use crate::codec::{decode_binary, wcstoul_decimal};
use crate::error::Error;
use crate::roots::RootKey;
use crate::script::Cursor;

/// A value written by `RegSetValueExW`. `Sz` holds the text without its NUL terminator.
pub enum Value {
    Sz(Vec<u16>),
    Dword(u32),
    Binary(Vec<u8>),
}

/// The registry calls the script engine makes. Failures are Win32 error codes.
/// A `None` parent stands for a handle that failed to open.
pub trait Registry {
    type Key: Copy;

    fn root(&self, root: RootKey) -> Self::Key;
    fn create_key(&mut self, parent: Option<Self::Key>, name: &[u16]) -> Result<Self::Key, u32>;
    fn open_key(&mut self, parent: Option<Self::Key>, name: &[u16]) -> Result<Self::Key, u32>;
    fn close_key(&mut self, key: Self::Key);
    fn delete_key(&mut self, parent: Option<Self::Key>, name: &[u16]);
    fn delete_tree(&mut self, parent: Option<Self::Key>, name: &[u16]);
    /// `name` of `None` is the default value.
    fn set_value(
        &mut self,
        key: Option<Self::Key>,
        name: Option<&[u16]>,
        value: &Value,
    ) -> Result<(), u32>;
}

/// The value of an `=` entry, chosen by its type letter. An unknown type fails before the
/// value is read.
pub(crate) fn read_value(cursor: &mut Cursor<'_>, kind: u16) -> Result<Value, Error> {
    match u8::try_from(kind) {
        Ok(b's') => Ok(Value::Sz(cursor.next_word()?)),
        Ok(b'd') => Ok(Value::Dword(wcstoul_decimal(&cursor.next_word()?))),
        Ok(b'b') => Ok(Value::Binary(decode_binary(&cursor.next_word()?)?)),
        _ => Err(Error::Exception),
    }
}
