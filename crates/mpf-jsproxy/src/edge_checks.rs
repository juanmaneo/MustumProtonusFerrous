//! Edge checks for the cases the Wine oracle rows do not reach.

use super::*;

const AF_INET6: i32 = 23;

fn units(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn names_stop_at_the_first_nul() {
    assert_eq!(
        dispid_for_name(&units("dns_resolve\0x")),
        Ok(DISPID_GLOBAL_DNSRESOLVE)
    );
    assert_eq!(
        dispid_for_name(&units("\0dns_resolve")),
        Err(DISP_E_UNKNOWNNAME)
    );
    let name = units("dns_resolve\0x");
    assert_eq!(
        invoke_ex(DISPID_GLOBAL_DNSRESOLVE, &[InvokeArg::Bstr(&name)]),
        Ok(&name[..11])
    );
}

#[test]
fn invoke_checks_the_count_before_the_type() {
    let name = units("dns_resolve");
    let bstr = InvokeArg::Bstr(&name);
    assert_eq!(
        invoke_ex(DISPID_GLOBAL_DNSRESOLVE, &[]),
        Err(DISP_E_BADPARAMCOUNT)
    );
    assert_eq!(
        invoke_ex(DISPID_GLOBAL_DNSRESOLVE, &[bstr, bstr]),
        Err(DISP_E_BADPARAMCOUNT)
    );
    assert_eq!(
        invoke_ex(
            DISPID_GLOBAL_DNSRESOLVE,
            &[InvokeArg::Other, InvokeArg::Other]
        ),
        Err(DISP_E_BADPARAMCOUNT)
    );
    assert_eq!(
        invoke_ex(DISPID_GLOBAL_DNSRESOLVE, &[InvokeArg::Other, bstr]),
        Err(DISP_E_BADPARAMCOUNT)
    );
}

#[test]
fn other_ids_are_not_members_whatever_the_arguments() {
    let name = units("dns_resolve");
    assert_eq!(
        invoke_ex(0, &[InvokeArg::Bstr(&name)]),
        Err(DISP_E_MEMBERNOTFOUND)
    );
    assert_eq!(
        invoke_ex(DISPID_GLOBAL_DNSRESOLVE + 1, &[]),
        Err(DISP_E_MEMBERNOTFOUND)
    );
}

#[test]
fn a_lookup_without_an_ipv4_entry_has_no_text() {
    assert_eq!(first_ipv4_text(Some(&[])), None);
    let v6 = [AddrEntry {
        family: AF_INET6,
        s_addr: 0,
    }];
    assert_eq!(first_ipv4_text(Some(&v6)), None);
    assert_eq!(first_ipv4_text(None), None);
}

#[test]
fn the_first_ipv4_entry_is_the_one_formatted() {
    let list = [
        AddrEntry {
            family: AF_INET,
            s_addr: u32::from_be(0x0A00_0001),
        },
        AddrEntry {
            family: AF_INET,
            s_addr: u32::from_be(0x0B00_0002),
        },
    ];
    assert_eq!(first_ipv4_text(Some(&list)).as_deref(), Some("10.0.0.1"));
}

#[test]
fn deinitialize_drops_a_script_and_is_idempotent() {
    let mut state = Some(units("script"));
    deinitialize_script(&mut state);
    assert_eq!(state, None);
    deinitialize_script(&mut state);
    assert_eq!(state, None);
}

#[test]
fn a_file_load_supplies_the_script_when_no_buffer_is_given() {
    let mut state = Some(units("old"));
    let outcome = initialize_script(&mut state, None, || Some(units("file")));
    assert_eq!(outcome, InitOutcome::Loaded);
    assert_eq!(state, Some(units("file")));
}

#[test]
fn a_buffer_script_replaces_the_previous_one_without_a_file_load() {
    let text = units("hello");
    let buffer = ScriptBuffer {
        struct_size: AUTO_PROXY_SCRIPT_BUFFER_SIZE,
        script: Some(&text),
        script_size: 5,
    };
    let mut state = Some(units("old"));
    let outcome = initialize_script(&mut state, Some(buffer), || None);
    assert_eq!(outcome, InitOutcome::Loaded);
    assert_eq!(state, Some(text));
}

#[cfg(target_pointer_width = "64")]
#[test]
fn the_buffer_size_is_the_win64_layout() {
    assert_eq!(AUTO_PROXY_SCRIPT_BUFFER_SIZE, 24);
}

#[cfg(target_pointer_width = "32")]
#[test]
fn the_buffer_size_is_the_win32_layout() {
    assert_eq!(AUTO_PROXY_SCRIPT_BUFFER_SIZE, 12);
}
