use crate::path::ObjectPath;
use crate::{
    BACKSLASH, COLON, COMMA, DOT, EQ, WBEMPATH_GET_NAMESPACE_ONLY, WBEMPATH_GET_ORIGINAL,
    WBEMPATH_GET_RELATIVE_ONLY, WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY, WBEMPATH_GET_SERVER_TOO,
};

/// The text GetText builds for `flags`, with the length it reports. The length
/// is the text length, except that a dotted key list counts its '.' even when
/// the text has none (an implied first key), as path.c does.
pub(crate) fn build_path(path: &ObjectPath, flags: i32) -> (Vec<u16>, usize) {
    match flags {
        0 => append_class(namespace_text(path, false), path),
        WBEMPATH_GET_RELATIVE_ONLY => {
            if class_name(path).is_empty() {
                (Vec::new(), 0)
            } else {
                class_and_keys(path)
            }
        }
        WBEMPATH_GET_SERVER_TOO => {
            let mut text = server_text(path);
            let server_len = text.len();
            let (rest, len) = append_class(namespace_text(path, true), path);
            text.extend(rest);
            (text, server_len + len)
        }
        WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY => {
            let mut text = server_text(path);
            text.extend(namespace_text(path, true));
            exact(text)
        }
        WBEMPATH_GET_NAMESPACE_ONLY => exact(namespace_text(path, false)),
        WBEMPATH_GET_ORIGINAL => exact(path.text.clone()),
        _ => (Vec::new(), 0),
    }
}

fn exact(text: Vec<u16>) -> (Vec<u16>, usize) {
    let len = text.len();
    (text, len)
}

fn class_name(path: &ObjectPath) -> &[u16] {
    path.class.as_deref().unwrap_or_default()
}

fn namespace_text(path: &ObjectPath, leading: bool) -> Vec<u16> {
    let mut text = Vec::new();
    for (i, namespace) in path.namespaces.iter().enumerate() {
        if i > 0 || leading {
            text.push(BACKSLASH);
        }
        text.extend_from_slice(namespace);
    }
    text
}

fn server_text(path: &ObjectPath) -> Vec<u16> {
    let mut text = vec![BACKSLASH, BACKSLASH];
    match path.server.as_deref() {
        Some(server) if !server.is_empty() => text.extend_from_slice(server),
        _ => text.push(DOT),
    }
    text
}

/// Appends ':' and the class and keys, when the class is not empty. Keys
/// without a class are not written.
fn append_class(mut text: Vec<u16>, path: &ObjectPath) -> (Vec<u16>, usize) {
    if class_name(path).is_empty() {
        return exact(text);
    }
    let (class, class_len) = class_and_keys(path);
    let len = text.len() + 1 + class_len;
    text.push(COLON);
    text.extend(class);
    (text, len)
}

fn class_and_keys(path: &ObjectPath) -> (Vec<u16>, usize) {
    let class = class_name(path);
    let (keys, keys_len) = key_part(path);
    let mut text = class.to_vec();
    text.extend(keys);
    (text, class.len() + keys_len)
}

/// The '.' and the key list. The first key's name is empty for an implied key,
/// and then the list starts with '=' and gets no '.'; the reported length
/// still counts the '.'.
fn key_part(path: &ObjectPath) -> (Vec<u16>, usize) {
    if path.keys.is_empty() {
        return (Vec::new(), 0);
    }
    let keylist = keylist_text(path);
    let mut text = Vec::with_capacity(keylist.len() + 1);
    if keylist.first() != Some(&EQ) {
        text.push(DOT);
    }
    let len = keylist.len() + 1;
    text.extend(keylist);
    (text, len)
}

fn keylist_text(path: &ObjectPath) -> Vec<u16> {
    let mut text = Vec::new();
    for (i, key) in path.keys.iter().enumerate() {
        if i > 0 {
            text.push(COMMA);
        }
        text.extend_from_slice(&key.name);
        text.push(EQ);
        text.extend_from_slice(&key.value);
    }
    text
}
