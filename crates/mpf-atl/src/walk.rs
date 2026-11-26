use crate::error::Error;
use crate::registry::{read_value, Registry};
use crate::roots::find_root;
use crate::script::{Cursor, EQUALS, MINUS};
use crate::text::eq_fold_str;

/// The `key_type` of `do_process_key`.
enum Kind {
    Normal,
    NoRemove,
    Val,
    ForceRemove,
    Delete,
}

fn classify(word: &[u16]) -> Kind {
    if eq_fold_str(word, "NoRemove") {
        Kind::NoRemove
    } else if eq_fold_str(word, "ForceRemove") {
        Kind::ForceRemove
    } else if eq_fold_str(word, "val") {
        Kind::Val
    } else if eq_fold_str(word, "Delete") {
        Kind::Delete
    } else {
        Kind::Normal
    }
}

/// `do_process_root_key`: runs each `root { ... }` block in `script`.
pub(crate) fn process_root_keys<R: Registry>(
    reg: &mut R,
    script: &[u16],
    register: bool,
) -> Result<(), Error> {
    let mut cursor = Cursor::new(script);
    let mut word = cursor.next_word()?;
    while !cursor.at_end() {
        let root = find_root(&word).ok_or(Error::Exception)?;
        word = cursor.next_word()?;
        if !eq_fold_str(&word, "{") {
            return Err(Error::Exception);
        }
        let key = reg.root(root);
        process_key(reg, &mut cursor, Some(key), register)?;
        word = cursor.next_word()?;
    }
    Ok(())
}

/// `do_process_key`: reads entries up to the closing `}`. The opening `{` is already consumed.
fn process_key<R: Registry>(
    reg: &mut R,
    cursor: &mut Cursor<'_>,
    parent: Option<R::Key>,
    register: bool,
) -> Result<(), Error> {
    let mut word = block_word(cursor)?;
    while !eq_fold_str(&word, "}") {
        let kind = classify(&word);
        let name = match kind {
            Kind::Normal => word,
            _ => cursor.next_word()?,
        };
        let mut opened = None;
        let result = process_entry(reg, cursor, parent, &kind, &name, register, &mut opened);
        if let Some(key) = opened {
            reg.close_key(key);
        }
        result?;
        word = block_word(cursor)?;
    }
    Ok(())
}

/// One entry of a key block: a value, a deletion, or a key with an optional value and
/// nested block. `opened` receives the key handle, which the caller closes.
fn process_entry<R: Registry>(
    reg: &mut R,
    cursor: &mut Cursor<'_>,
    parent: Option<R::Key>,
    kind: &Kind,
    name: &[u16],
    register: bool,
    opened: &mut Option<R::Key>,
) -> Result<(), Error> {
    if matches!(kind, Kind::Delete) {
        if register {
            reg.delete_tree(parent, name);
        }
        return Ok(());
    }
    let is_val = matches!(kind, Kind::Val);
    if !is_val {
        if register {
            if matches!(kind, Kind::ForceRemove) {
                reg.delete_tree(parent, name);
            }
            *opened = Some(reg.create_key(parent, name).map_err(Error::Win32)?);
        } else {
            *opened = reg.open_key(parent, name).ok();
        }
    }
    let value_key = if is_val { parent } else { *opened };

    if cursor.eat(EQUALS) {
        let type_word = cursor.next_word()?;
        if type_word.len() != 1 {
            return Err(Error::Exception);
        }
        if register {
            let value = read_value(cursor, type_word[0])?;
            let value_name = match kind {
                Kind::Val if !name.is_empty() => Some(name),
                _ => None,
            };
            reg.set_value(value_key, value_name, &value)
                .map_err(Error::Win32)?;
        } else {
            cursor.eat(MINUS);
            cursor.next_word()?;
        }
    } else if is_val {
        return Err(Error::Exception);
    }

    if !is_val && cursor.eat_block_open() {
        process_key(reg, cursor, *opened, register)?;
    }
    if !register && matches!(kind, Kind::Normal | Kind::ForceRemove) {
        reg.delete_key(parent, name);
    }
    Ok(())
}

/// The next word inside a block. Running out of input there is an `Exception`; the C
/// loop instead spins on empty words.
fn block_word(cursor: &mut Cursor<'_>) -> Result<Vec<u16>, Error> {
    if cursor.at_end() {
        return Err(Error::Exception);
    }
    cursor.next_word()
}
