//! Oracle-derived checks for the ported directmanipulation logic.
//!
//! Rows were captured by compiling the verbatim Wine C bodies from `dlls/directmanipulation/directmanipulation.c`
//! with `gcc -fsanitize=address,undefined -g` under `target/canon/directmanipulation/` and running the harness.
//! The harness printed 101 rows: 40 logic rows (`L01`-`L40`) and 61 stub rows, one per export. ASan and
//! UBSan reported nothing for any row, so no vector is excluded. `ret` in the logic rows is the count after the
//! operation. The harness creates objects with a count of 1, and each comment gives the expected value.

use super::*;

#[test]
fn oracle_l01_to_l04_update_manager() {
    // L01 um.QI(IUnknown): S_OK, ret 2
    let mut um = UpdateManager::default();
    assert_eq!(um.query_interface(&IID_UNKNOWN), Ok(Iface::UpdateManager));
    assert_eq!(um.ref_count(), 2);
    // L02 um.QI(Manager): E_NOINTERFACE, ret 1
    let mut um = UpdateManager::default();
    assert_eq!(um.query_interface(&IID_MANAGER), Err(E_NOINTERFACE));
    assert_eq!(um.ref_count(), 1);
    // L03 um.AddRef: ret 2
    let mut um = UpdateManager::default();
    assert_eq!(um.add_ref(), 2);
    // L04 um.Release: ret 0
    let mut um = UpdateManager::default();
    assert_eq!(um.release(), 0);
}

#[test]
fn oracle_l05_to_l08_primary_content() {
    // L05 pc.QI(PrimaryContent): S_OK, ret 2
    let mut pc = PrimaryContext::default();
    assert_eq!(
        pc.query_interface(&IID_PRIMARY_CONTENT),
        Ok(Iface::PrimaryContent)
    );
    assert_eq!(pc.ref_count(), 2);
    // L06 pc.QI(Content): S_OK, ret 2
    let mut pc = PrimaryContext::default();
    assert_eq!(pc.query_interface(&IID_CONTENT), Ok(Iface::Content));
    assert_eq!(pc.ref_count(), 2);
    // L07 pc.QI(Viewport2): E_NOINTERFACE, ret 1
    let mut pc = PrimaryContext::default();
    assert_eq!(pc.query_interface(&IID_VIEWPORT2), Err(E_NOINTERFACE));
    assert_eq!(pc.ref_count(), 1);
    // L08 content.QI(IUnknown): the Content interface delegates to the primary QI, so ret 2 with the PrimaryContent pointer
    let mut pc = PrimaryContext::default();
    assert_eq!(pc.query_interface(&IID_UNKNOWN), Ok(Iface::PrimaryContent));
    assert_eq!(pc.ref_count(), 2);
}

#[test]
fn oracle_l09_to_l12_viewport() {
    // L09 vp.QI(Viewport2): S_OK, ret 2
    let mut vp = Viewport::default();
    assert_eq!(vp.query_interface(&IID_VIEWPORT2), Ok(Iface::Viewport));
    assert_eq!(vp.ref_count(), 2);
    // L10 vp.QI(Manager2): E_NOINTERFACE, ret 1
    let mut vp = Viewport::default();
    assert_eq!(vp.query_interface(&IID_MANAGER2), Err(E_NOINTERFACE));
    assert_eq!(vp.ref_count(), 1);
    // L11 vp.GetPrimaryContent(PrimaryContent): S_OK, pointer A, ret 1 (the viewport count is unchanged)
    let vp = Viewport::default();
    let pc = vp.get_primary_content(&IID_PRIMARY_CONTENT).unwrap();
    assert_eq!(pc.ref_count(), 1);
    assert_eq!(vp.ref_count(), 1);
    // L12 vp.GetPrimaryContent(Viewport): E_NOTIMPL, NULL, ret 1
    let vp = Viewport::default();
    assert_eq!(vp.get_primary_content(&IID_VIEWPORT).err(), Some(E_NOTIMPL));
    assert_eq!(vp.ref_count(), 1);
}

#[test]
fn oracle_l13_to_l19_manager_viewport_and_update_manager() {
    // L13 mgr.CreateViewport(Viewport): S_OK, ret 1
    let vp = Manager::default().create_viewport(&IID_VIEWPORT).unwrap();
    assert_eq!(vp.ref_count(), 1);
    // L14 mgr.CreateViewport(Viewport2): S_OK, ret 1
    let vp = Manager::default().create_viewport(&IID_VIEWPORT2).unwrap();
    assert_eq!(vp.ref_count(), 1);
    // L15 mgr.CreateViewport(IUnknown): E_NOTIMPL, NULL, mgr ret 1
    let mgr = Manager::default();
    assert_eq!(mgr.create_viewport(&IID_UNKNOWN).err(), Some(E_NOTIMPL));
    assert_eq!(mgr.ref_count(), 1);

    let mut mgr = Manager::default();
    // L16 mgr.GetUpdateManager(UpdateManager) first: S_OK, ret 2
    assert_eq!(
        mgr.get_update_manager(&IID_UPDATE_MANAGER)
            .map(|um| um.ref_count()),
        Ok(2)
    );
    // L17 mgr.GetUpdateManager(UpdateManager) second: S_OK, ret 3
    assert_eq!(
        mgr.get_update_manager(&IID_UPDATE_MANAGER)
            .map(|um| um.ref_count()),
        Ok(3)
    );
    // L18 mgr.GetUpdateManager(IUnknown): E_FAIL, NULL, mgr ret 1
    assert_eq!(mgr.get_update_manager(&IID_UNKNOWN).err(), Some(E_FAIL));
    assert_eq!(mgr.ref_count(), 1);
    // L19 update manager count after the manager is released: 2 (the caller still holds two references)
    assert_eq!(mgr.release(), 0);
    assert_eq!(mgr.update_manager().map(UpdateManager::ref_count), Some(2));
}

#[test]
fn oracle_l20_to_l26_create_instance() {
    // L20 CreateInstance(Manager2): S_OK, ret 1
    let (mgr, iface) = create_manager_instance(&IID_MANAGER2).unwrap();
    assert_eq!(iface, Iface::Manager);
    assert_eq!(mgr.ref_count(), 1);
    // L21 CreateInstance(Manager): S_OK, ret 1
    let (mgr, iface) = create_manager_instance(&IID_MANAGER).unwrap();
    assert_eq!(iface, Iface::Manager);
    assert_eq!(mgr.ref_count(), 1);
    // L22 CreateInstance(IUnknown): S_OK, ret 1
    let (mgr, iface) = create_manager_instance(&IID_UNKNOWN).unwrap();
    assert_eq!(iface, Iface::Manager);
    assert_eq!(mgr.ref_count(), 1);
    // L23 CreateInstance(IClassFactory): E_NOINTERFACE, NULL
    assert_eq!(
        create_manager_instance(&IID_CLASS_FACTORY).err(),
        Some(E_NOINTERFACE)
    );
    // L24 CompositorCreateInstance(IUnknown): S_OK, pointer A, ret 1
    let (cp, iface) = create_compositor_instance(&IID_UNKNOWN).unwrap();
    assert_eq!(iface, Iface::Compositor);
    assert_eq!(cp.ref_count(), 1);
    // L25 CompositorCreateInstance(FrameInfoProvider): S_OK, pointer B, ret 1
    let (cp, iface) = create_compositor_instance(&IID_FRAME_INFO_PROVIDER).unwrap();
    assert_eq!(iface, Iface::FrameInfoProvider);
    assert_eq!(cp.ref_count(), 1);
    // L26 CompositorCreateInstance(Manager): E_NOINTERFACE, NULL
    assert_eq!(
        create_compositor_instance(&IID_MANAGER).err(),
        Some(E_NOINTERFACE)
    );
}

#[test]
fn oracle_l27_to_l30_compositor_and_frame_info_provider() {
    // L27 cp.SetUpdateManager(NULL): E_INVALIDARG, ret 1
    let mut cp = Compositor::default();
    assert_eq!(cp.set_update_manager(None), E_INVALIDARG);
    assert_eq!(cp.ref_count(), 1);
    // L28 cp.SetUpdateManager(um): S_OK, update manager ret 2
    let mut cp = Compositor::default();
    let mut um = UpdateManager::default();
    assert_eq!(cp.set_update_manager(Some(&mut um)), S_OK);
    assert_eq!(um.ref_count(), 2);
    // L29 fip.QI(IUnknown): S_OK, pointer A (the Compositor interface), ret 2
    let mut cp = Compositor::default();
    assert_eq!(cp.query_interface(&IID_UNKNOWN), Ok(Iface::Compositor));
    assert_eq!(cp.ref_count(), 2);
    // L30 fip.QI(FrameInfoProvider): S_OK, pointer A (the FrameInfoProvider interface), ret 2
    let mut cp = Compositor::default();
    assert_eq!(
        cp.query_interface(&IID_FRAME_INFO_PROVIDER),
        Ok(Iface::FrameInfoProvider)
    );
    assert_eq!(cp.ref_count(), 2);
}

#[test]
fn oracle_l31_to_l40_class_factory() {
    // L31 factory.QI(IClassFactory): S_OK, pointer A (the manager factory)
    assert_eq!(
        class_factory_query_interface(Factory::Manager, &IID_CLASS_FACTORY),
        Ok(Iface::ClassFactory(Factory::Manager))
    );
    // L32 DllGetClassObject(Manager, IClassFactory): S_OK, pointer A
    assert_eq!(
        dll_get_class_object(&CLSID_MANAGER, &IID_CLASS_FACTORY),
        Ok(Iface::ClassFactory(Factory::Manager))
    );
    // L33 DllGetClassObject(Shared, IUnknown): S_OK, pointer A
    assert_eq!(
        dll_get_class_object(&CLSID_SHARED_MANAGER, &IID_UNKNOWN),
        Ok(Iface::ClassFactory(Factory::Manager))
    );
    // L34 DllGetClassObject(DComp, IClassFactory): S_OK, pointer B (the compositor factory)
    assert_eq!(
        dll_get_class_object(&CLSID_DCOMP_COMPOSITOR, &IID_CLASS_FACTORY),
        Ok(Iface::ClassFactory(Factory::Compositor))
    );
    // L35 DllGetClassObject(Manager, Manager2): E_NOINTERFACE, NULL
    assert_eq!(
        dll_get_class_object(&CLSID_MANAGER, &IID_MANAGER2).err(),
        Some(E_NOINTERFACE)
    );
    // L36 DllGetClassObject(unknown CLSID): CLASS_E_CLASSNOTAVAILABLE, NULL
    let unknown = Guid::new(0, 0, 0, [0; 8]);
    assert_eq!(
        dll_get_class_object(&unknown, &IID_UNKNOWN).err(),
        Some(CLASS_E_CLASSNOTAVAILABLE)
    );
    // L37 factory.AddRef: ret 2
    assert_eq!(class_factory_add_ref(), 2);
    // L38 factory.Release: ret 1
    assert_eq!(class_factory_release(), 1);
    // L39 factory.LockServer(TRUE): S_OK
    assert_eq!(class_factory_lock_server(true), S_OK);
    // L40 factory.QI(Manager): E_NOINTERFACE, NULL
    assert_eq!(
        class_factory_query_interface(Factory::Manager, &IID_MANAGER).err(),
        Some(E_NOINTERFACE)
    );
}

/// Stub rows: export name, then the return value the harness printed (`E_NOTIMPL` for 0x80004001, `S_OK` for 0).
const STUB_ROWS: [(&str, HResult); 61] = [
    ("update_manager_RegisterWaitHandleCallback", E_NOTIMPL),
    ("update_manager_UnregisterWaitHandleCallback", E_NOTIMPL),
    ("update_manager_Update", E_NOTIMPL),
    ("primary_SetSnapInterval", E_NOTIMPL),
    ("primary_SetSnapPoints", E_NOTIMPL),
    ("primary_SetSnapType", E_NOTIMPL),
    ("primary_SetSnapCoordinate", E_NOTIMPL),
    ("primary_SetZoomBoundaries", E_NOTIMPL),
    ("primary_SetHorizontalAlignment", E_NOTIMPL),
    ("primary_SetVerticalAlignment", E_NOTIMPL),
    ("primary_GetInertiaEndTransform", E_NOTIMPL),
    ("primary_GetCenterPoint", E_NOTIMPL),
    ("content_GetContentRect", E_NOTIMPL),
    ("content_SetContentRect", S_OK),
    ("content_GetViewport", E_NOTIMPL),
    ("content_GetTag", E_NOTIMPL),
    ("content_SetTag", E_NOTIMPL),
    ("content_GetOutputTransform", E_NOTIMPL),
    ("content_GetContentTransform", E_NOTIMPL),
    ("content_SyncContentTransform", E_NOTIMPL),
    ("viewport_Enable", E_NOTIMPL),
    ("viewport_Disable", E_NOTIMPL),
    ("viewport_SetContact", E_NOTIMPL),
    ("viewport_ReleaseContact", E_NOTIMPL),
    ("viewport_ReleaseAllContacts", E_NOTIMPL),
    ("viewport_GetStatus", E_NOTIMPL),
    ("viewport_GetTag", E_NOTIMPL),
    ("viewport_SetTag", E_NOTIMPL),
    ("viewport_GetViewportRect", E_NOTIMPL),
    ("viewport_SetViewportRect", S_OK),
    ("viewport_ZoomToRect", E_NOTIMPL),
    ("viewport_SetViewportTransform", E_NOTIMPL),
    ("viewport_SyncDisplayTransform", E_NOTIMPL),
    ("viewport_AddContent", E_NOTIMPL),
    ("viewport_RemoveContent", E_NOTIMPL),
    ("viewport_SetViewportOptions", S_OK),
    ("viewport_AddConfiguration", E_NOTIMPL),
    ("viewport_RemoveConfiguration", E_NOTIMPL),
    ("viewport_ActivateConfiguration", S_OK),
    ("viewport_SetManualGesture", E_NOTIMPL),
    ("viewport_SetChaining", E_NOTIMPL),
    ("viewport_AddEventHandler", E_NOTIMPL),
    ("viewport_RemoveEventHandler", E_NOTIMPL),
    ("viewport_SetInputMode", E_NOTIMPL),
    ("viewport_SetUpdateMode", E_NOTIMPL),
    ("viewport_Stop", E_NOTIMPL),
    ("viewport_Abandon", E_NOTIMPL),
    ("viewport_AddBehavior", E_NOTIMPL),
    ("viewport_RemoveBehavior", E_NOTIMPL),
    ("viewport_RemoveAllBehaviors", E_NOTIMPL),
    ("direct_manip_Activate", E_NOTIMPL),
    ("direct_manip_Deactivate", E_NOTIMPL),
    ("direct_manip_RegisterHitTestTarget", E_NOTIMPL),
    ("direct_manip_ProcessInput", E_NOTIMPL),
    ("direct_manip_CreateContent", E_NOTIMPL),
    ("direct_manip_CreateBehavior", E_NOTIMPL),
    ("compositor_AddContent", E_NOTIMPL),
    ("compositor_RemoveContent", E_NOTIMPL),
    ("compositor_Flush", E_NOTIMPL),
    ("compositor_AddContentWithCrossProcessChaining", E_NOTIMPL),
    ("provider_GetNextFrameInfo", E_NOTIMPL),
];

#[test]
fn oracle_stub_rows() {
    for (export, hr) in STUB_ROWS {
        assert_eq!(stub_return(export), Some(hr), "{export}");
    }
}
