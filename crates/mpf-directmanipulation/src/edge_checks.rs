//! Edge-case checks for the ported directmanipulation logic.
//!
//! The GUID strings come from `include/directmanipulation.idl` and `include/unknwn.idl`. The remaining
//! expectations were derived from reading the Wine source and are not oracle output. Each one is marked
//! "not oracle-derived" in its comment.

use super::*;

const IDL_GUIDS: [(Guid, &str); 15] = [
    (IID_UNKNOWN, "00000000-0000-0000-c000-000000000046"),
    (IID_CLASS_FACTORY, "00000001-0000-0000-c000-000000000046"),
    (IID_UPDATE_MANAGER, "b0ae62fd-be34-46e7-9caa-d361facbb9cc"),
    (IID_MANAGER, "fbf5d3b4-70c7-4163-9322-5a6f660d6fbc"),
    (IID_MANAGER2, "fa1005e9-3d16-484c-bfc9-62b61e56ec4e"),
    (IID_CONTENT, "b89962cb-3d89-442b-bb58-5098fa0f9f16"),
    (IID_PRIMARY_CONTENT, "c12851e4-1698-4625-b9b1-7ca3ec18630b"),
    (IID_COMPOSITOR, "537a0825-0387-4efa-b62f-71eb1f085a7e"),
    (IID_COMPOSITOR2, "d38c7822-f1cb-43cb-b4b9-ac0c767a412e"),
    (IID_VIEWPORT, "28b85a3d-60a0-48bd-9ba1-5ce8d9ea3a6d"),
    (IID_VIEWPORT2, "923ccaac-61e1-4385-b726-017af189882a"),
    (
        IID_FRAME_INFO_PROVIDER,
        "fb759dba-6f4c-4c01-874e-19c8a05907f9",
    ),
    (CLSID_MANAGER, "54e211b6-3650-4f75-8334-fa359598e1c5"),
    (CLSID_SHARED_MANAGER, "99793286-77cc-4b57-96db-3b354f6f9fb5"),
    (
        CLSID_DCOMP_COMPOSITOR,
        "79dea627-a08a-43ac-8ef5-6900b9299126",
    ),
];

#[test]
fn guid_display_matches_idl_strings() {
    // Not oracle-derived: compared with the IDL strings.
    for (guid, text) in IDL_GUIDS {
        assert_eq!(guid.to_string(), text);
    }
}

#[test]
fn classify_known_and_unknown_iids() {
    // Not oracle-derived: every IID in the table classifies to its own variant.
    let expected = [
        (IID_UNKNOWN, Iid::Unknown),
        (IID_CLASS_FACTORY, Iid::ClassFactory),
        (IID_UPDATE_MANAGER, Iid::UpdateManager),
        (IID_MANAGER, Iid::Manager),
        (IID_MANAGER2, Iid::Manager2),
        (IID_CONTENT, Iid::Content),
        (IID_PRIMARY_CONTENT, Iid::PrimaryContent),
        (IID_COMPOSITOR, Iid::Compositor),
        (IID_COMPOSITOR2, Iid::Compositor2),
        (IID_VIEWPORT, Iid::Viewport),
        (IID_VIEWPORT2, Iid::Viewport2),
        (IID_FRAME_INFO_PROVIDER, Iid::FrameInfoProvider),
    ];
    for (guid, iid) in expected {
        assert_eq!(classify_iid(&guid), Some(iid));
    }
    // Not oracle-derived: an all-zero GUID is not a known IID.
    assert_eq!(classify_iid(&Guid::new(0, 0, 0, [0; 8])), None);
}

#[test]
fn classify_known_and_unknown_clsids() {
    // Not oracle-derived: the shared manager CLSID is a separate value from the manager CLSID.
    assert_eq!(classify_clsid(&CLSID_MANAGER), Some(Clsid::Manager));
    assert_eq!(classify_clsid(&CLSID_SHARED_MANAGER), Some(Clsid::Shared));
    assert_eq!(
        classify_clsid(&CLSID_DCOMP_COMPOSITOR),
        Some(Clsid::DCompCompositor)
    );
    assert_eq!(classify_clsid(&IID_MANAGER), None);
}

#[test]
fn succeeded_uses_the_severity_bit() {
    // Not oracle-derived: SUCCEEDED() is true exactly when the top bit is clear.
    assert!(succeeded(S_OK));
    assert!(succeeded(0x7FFF_FFFF));
    assert!(!succeeded(0x8000_0000));
    assert!(!succeeded(E_NOTIMPL));
}

#[test]
fn ref_count_starts_at_one_and_moves_both_ways() {
    // Not oracle-derived: the default count and the arithmetic the harness reads through `ret`.
    let mut refs = RefCount::default();
    assert_eq!(refs.count(), 1);
    assert_eq!(refs.add_ref(), 2);
    assert_eq!(refs.release(), 1);
    assert_eq!(refs.release(), 0);
}

#[test]
fn query_interface_rejects_interfaces_the_object_does_not_have() {
    // Not oracle-derived: each object refuses IIDs that belong to another object.
    assert_eq!(query_interface(Object::Viewport, &IID_MANAGER), None);
    assert_eq!(query_interface(Object::UpdateManager, &IID_CONTENT), None);
    assert_eq!(
        query_interface(Object::PrimaryContext, &IID_VIEWPORT2),
        None
    );
    assert_eq!(query_interface(Object::Compositor, &IID_CONTENT), None);
    assert_eq!(
        query_interface(Object::Manager, &IID_FRAME_INFO_PROVIDER),
        None
    );
    assert_eq!(
        query_interface(Object::Manager, &Guid::new(1, 2, 3, [4; 8])),
        None
    );
    assert_eq!(
        query_interface(Object::Compositor, &IID_FRAME_INFO_PROVIDER),
        Some(Iface::FrameInfoProvider)
    );
    assert_eq!(
        query_interface(Object::PrimaryContext, &IID_CONTENT),
        Some(Iface::Content)
    );
}

#[test]
fn query_counted_leaves_the_count_alone_on_failure() {
    // Not oracle-derived: a failed query must not add a reference.
    let mut refs = RefCount::default();
    assert_eq!(
        query_counted(Object::Manager, &IID_VIEWPORT, &mut refs),
        Err(E_NOINTERFACE)
    );
    assert_eq!(refs.count(), 1);
    assert_eq!(
        query_counted(Object::Manager, &IID_MANAGER, &mut refs),
        Ok(Iface::Manager)
    );
    assert_eq!(refs.count(), 2);
}

#[test]
fn object_add_ref_and_release_without_queries() {
    // Not oracle-derived: AddRef and Release on each object type, outside the harness rows.
    let mut mgr = Manager::default();
    assert_eq!(mgr.add_ref(), 2);
    assert_eq!(mgr.release(), 1);
    let mut vp = Viewport::default();
    assert_eq!(vp.add_ref(), 2);
    assert_eq!(vp.release(), 1);
    let mut pc = PrimaryContext::default();
    assert_eq!(pc.add_ref(), 2);
    assert_eq!(pc.release(), 1);
    let mut cp = Compositor::default();
    assert_eq!(cp.add_ref(), 2);
    assert_eq!(cp.release(None), 1);
}

#[test]
fn manager_release_to_zero_without_update_manager() {
    // Not oracle-derived: a manager that never created an update manager releases nothing else.
    let mut mgr = Manager::default();
    assert!(mgr.update_manager().is_none());
    assert_eq!(mgr.release(), 0);
    assert!(mgr.update_manager().is_none());
}

#[test]
fn manager_release_above_zero_keeps_the_update_manager_count() {
    // Not oracle-derived: only the release that reaches zero drops the manager's reference.
    let mut mgr = Manager::default();
    assert_eq!(
        mgr.get_update_manager(&IID_UPDATE_MANAGER)
            .map(|um| um.ref_count()),
        Ok(2)
    );
    mgr.add_ref();
    assert_eq!(mgr.release(), 1);
    assert_eq!(mgr.update_manager().map(UpdateManager::ref_count), Some(2));
}

#[test]
fn create_viewport_accepts_viewport_iids_only() {
    // Not oracle-derived: Content and Compositor IIDs are not viewports, so they get E_NOTIMPL.
    let mgr = Manager::default();
    assert_eq!(mgr.create_viewport(&IID_CONTENT).err(), Some(E_NOTIMPL));
    assert_eq!(
        mgr.create_viewport(&IID_VIEWPORT2).map(|vp| vp.ref_count()),
        Ok(1)
    );
}

#[test]
fn get_primary_content_is_only_for_primary_content_iid() {
    // Not oracle-derived: the Content IID is not accepted by GetPrimaryContent.
    let vp = Viewport::default();
    assert_eq!(vp.get_primary_content(&IID_CONTENT).err(), Some(E_NOTIMPL));
    assert_eq!(
        vp.get_primary_content(&IID_PRIMARY_CONTENT)
            .map(|pc| pc.ref_count()),
        Ok(1)
    );
}

#[test]
fn get_update_manager_rejects_other_iids_without_creating_one() {
    // Not oracle-derived: a rejected IID must not create the update manager.
    let mut mgr = Manager::default();
    assert_eq!(mgr.get_update_manager(&IID_MANAGER).err(), Some(E_FAIL));
    assert!(mgr.update_manager().is_none());
}

#[test]
fn compositor_set_update_manager_overwrite_leaks_the_previous_reference() {
    // Not oracle-derived: Wine overwrites `This->manager` without releasing it, so the first
    // manager keeps its reference (bug-compatible behaviour from the source).
    let mut cp = Compositor::default();
    let mut first = UpdateManager::default();
    let mut second = UpdateManager::default();
    assert_eq!(cp.set_update_manager(Some(&mut first)), S_OK);
    assert_eq!(cp.set_update_manager(Some(&mut second)), S_OK);
    assert_eq!(first.ref_count(), 2);
    assert_eq!(second.ref_count(), 2);
    assert_eq!(cp.release(Some(&mut second)), 0);
    assert_eq!(second.ref_count(), 1);
    assert_eq!(first.ref_count(), 2);
}

#[test]
fn compositor_release_paths() {
    // Not oracle-derived: the manager is released only when the count reaches zero and a reference was taken.
    let mut um = UpdateManager::default();
    let mut cp = Compositor::default();
    cp.add_ref();
    assert_eq!(cp.set_update_manager(Some(&mut um)), S_OK);
    assert_eq!(cp.release(Some(&mut um)), 1);
    assert_eq!(um.ref_count(), 2);
    assert_eq!(cp.release(Some(&mut um)), 0);
    assert_eq!(um.ref_count(), 1);

    let mut um = UpdateManager::default();
    let mut cp = Compositor::default();
    assert_eq!(cp.release(Some(&mut um)), 0);
    assert_eq!(um.ref_count(), 1);

    let mut um = UpdateManager::default();
    let mut cp = Compositor::default();
    assert_eq!(cp.set_update_manager(Some(&mut um)), S_OK);
    assert_eq!(cp.release(None), 0);
    assert_eq!(um.ref_count(), 2);
}

#[test]
fn class_factory_query_interface_accepts_unknown_and_class_factory_only() {
    // Not oracle-derived: the factory answers IUnknown and IClassFactory, nothing else.
    assert_eq!(
        class_factory_query_interface(Factory::Compositor, &IID_UNKNOWN),
        Ok(Iface::ClassFactory(Factory::Compositor))
    );
    assert_eq!(
        class_factory_query_interface(Factory::Compositor, &IID_COMPOSITOR),
        Err(E_NOINTERFACE)
    );
}

#[test]
fn dll_get_class_object_routes_each_clsid() {
    // Not oracle-derived: the shared manager CLSID uses the manager factory, and the compositor factory
    // answers IClassFactory too.
    assert_eq!(
        dll_get_class_object(&CLSID_SHARED_MANAGER, &IID_CLASS_FACTORY),
        Ok(Iface::ClassFactory(Factory::Manager))
    );
    assert_eq!(
        dll_get_class_object(&CLSID_DCOMP_COMPOSITOR, &IID_UNKNOWN),
        Ok(Iface::ClassFactory(Factory::Compositor))
    );
    assert_eq!(
        dll_get_class_object(&CLSID_DCOMP_COMPOSITOR, &IID_MANAGER).err(),
        Some(E_NOINTERFACE)
    );
}

#[test]
fn stub_return_rejects_unknown_exports() {
    // Not oracle-derived: a name outside the 61 stub exports has no modelled return value.
    assert_eq!(stub_return("direct_manip_Unknown"), None);
    assert_eq!(stub_return(""), None);
}
