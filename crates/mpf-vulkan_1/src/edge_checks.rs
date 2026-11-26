//! Edge-case tests for the ported vulkan-1 logic.
//!
//! Not every expectation is oracle-derived; the ones that aren't say so in a comment.

use super::*;

#[test]
fn constants_are_pinned() {
    // Not oracle-derived: values from include/winternl.h and include/winnt.h.
    assert_eq!(DLL_PROCESS_ATTACH, 1);
    assert_eq!(EXCEPTION_WINE_STUB, 0x8000_0100);
    assert_eq!(EXCEPTION_NONCONTINUABLE, 0x01);
}

#[test]
fn dll_main_no_effects_off_attach() {
    // Not oracle-derived beyond the rows in oracle_checks: detach, thread attach and thread detach.
    for reason in [0, 2, 3] {
        assert_eq!(dll_main(reason), (true, DllMainEffects::default()));
    }
}

#[test]
fn dll_main_attach_makes_both_calls() {
    assert_eq!(
        dll_main(DLL_PROCESS_ATTACH),
        (
            true,
            DllMainEffects {
                disable_thread_library_calls: true,
                get_dpi_for_system: true,
            }
        )
    );
}

#[test]
fn export_lookup_is_case_sensitive_and_exact() {
    // Not oracle-derived: lookup semantics of the port's own table helpers.
    assert_eq!(find_export("vkcreatedevice"), None);
    assert_eq!(find_export(""), None);
    assert_eq!(export_kind("vkCreateDevice "), None);
    assert_eq!(forward_target("vkNotAnExport"), None);
    assert_eq!(stub_exception("vkNotAnExport"), None);
    assert_eq!(
        forward_target("vkCreateDevice"),
        Some("winevulkan.vkCreateDevice")
    );
}

#[test]
fn first_and_last_exports() {
    // Not oracle-derived: spec order, first and last lines.
    assert_eq!(EXPORTS[0].name, "vkAcquireNextImage2KHR");
    assert_eq!(EXPORTS[0].args, "ptr ptr ptr");
    assert_eq!(EXPORTS[EXPORTS.len() - 1].name, "vkWaitSemaphores");
    assert_eq!(EXPORTS[EXPORTS.len() - 1].args, "ptr ptr int64");
}

#[test]
fn export_names_are_unique_and_forwards_match_names() {
    // Not oracle-derived: every forward target is `winevulkan.<name>`, and no name repeats.
    for (i, e) in EXPORTS.iter().enumerate() {
        assert!(EXPORTS[..i].iter().all(|o| o.name != e.name), "{}", e.name);
        if let ExportKind::Forward(target) = e.kind {
            assert_eq!(target.strip_prefix("winevulkan."), Some(e.name));
        } else {
            assert_eq!(e.args, "");
        }
    }
}

#[test]
fn custom_stub_record() {
    // Not oracle-derived: the generic record builder keeps the names it is given.
    let e = wine_spec_unimplemented_stub("x.dll", "f");
    assert_eq!(e.module, "x.dll");
    assert_eq!(e.function, "f");
    assert_eq!(e.number_parameters, 2);
}
