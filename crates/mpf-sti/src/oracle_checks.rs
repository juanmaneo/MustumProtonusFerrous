//! Oracle-derived expectations for the ported `dlls/sti` logic.
//!
//! The rows come from `target/canon/sti/harness.c`. It compiles the verbatim Wine bodies of
//! `dlls/sti/sti.c` and `dlls/sti/sti_main.c`, and `DllCanUnloadNow` from `libs/winecrt0`, with
//! `gcc -fshort-wchar -fsanitize=address,undefined -g`. It printed 63 rows (R01 to R63). No
//! ASan or UBSan report was raised and no row was excluded. The harness runs with
//! `ASAN_OPTIONS=detect_leaks=0` because some rows keep objects alive. The midl proxy-stub
//! functions are test doubles in the harness, so rows R05 to R09 check forwarding only.
//!
//! Each test quotes the printed row. The C harness prints `ppv_set`, `out_null`, `same` and
//! similar flags for pointers. The port returns no pointer, so those flags are derived from
//! the result, and the comment says how.

use super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub(crate) fn hr(v: u32) -> Hresult {
    v as Hresult
}

fn other_clsid() -> Guid {
    Guid {
        data1: 0x1234_5678,
        data2: 0,
        data3: 0,
        data4: [0, 0, 0, 0, 0, 0, 0, 1],
    }
}

pub(crate) fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

/// The state the C harness's `fake_outer` has: its counter, the result of its
/// QueryInterface, and the number of QueryInterface calls.
pub(crate) struct OuterState {
    pub(crate) refs: u32,
    pub(crate) qi_ret: Hresult,
    pub(crate) qi_calls: u32,
}

struct FakeOuter(Rc<RefCell<OuterState>>);

impl OuterUnknown for FakeOuter {
    fn query_interface(&mut self, _riid: &Guid) -> Hresult {
        let mut s = self.0.borrow_mut();
        s.qi_calls += 1;
        s.qi_ret
    }

    fn add_ref(&mut self) -> u32 {
        let mut s = self.0.borrow_mut();
        s.refs += 1;
        s.refs
    }

    fn release(&mut self) -> u32 {
        let mut s = self.0.borrow_mut();
        s.refs -= 1;
        s.refs
    }
}

/// A `pUnkOuter` with a count of 5, as in the harness, and the handle for reading its state.
pub(crate) fn outer(qi_ret: Hresult) -> (Option<Box<dyn OuterUnknown>>, Rc<RefCell<OuterState>>) {
    let state = Rc::new(RefCell::new(OuterState {
        refs: 5,
        qi_ret,
        qi_calls: 0,
    }));
    (Some(Box::new(FakeOuter(Rc::clone(&state)))), state)
}

/// The registry calls, with the return codes the harness sets and what each call saw.
#[derive(Default)]
pub(crate) struct FakeRegistry {
    pub(crate) create_ret: u32,
    pub(crate) set_ret: u32,
    pub(crate) delete_ret: u32,
    pub(crate) closes: u32,
    pub(crate) set_calls: u32,
    pub(crate) last_name: Vec<u16>,
    pub(crate) last_data: Vec<u16>,
}

impl Registry for FakeRegistry {
    fn create_key(&mut self, _path: &[u16]) -> u32 {
        self.create_ret
    }

    fn set_value(&mut self, name: &[u16], data: &[u16]) -> u32 {
        self.set_calls += 1;
        self.last_name = name.to_vec();
        self.last_data = data.to_vec();
        self.set_ret
    }

    fn delete_value(&mut self, name: &[u16]) -> u32 {
        self.last_name = name.to_vec();
        self.delete_ret
    }

    fn close_key(&mut self) {
        self.closes += 1;
    }
}

#[test]
fn oracle_dll_exports() {
    // R01 | DllCanUnloadNow | hr=0x00000001 | -
    assert_eq!(dll_can_unload_now(), hr(0x0000_0001));

    // One proxy closure serves every DllGetClassObject call below, so the proxy branch and the
    // CLSID_Sti branch are counted in one instantiation. The counter shows which branch ran.
    let proxy_calls = Cell::new(0);
    let proxy = || {
        proxy_calls.set(proxy_calls.get() + 1);
        hr(0x8004_0154)
    };

    // R02 | DllGetClassObject(CLSID_Sti, IUnknown) | hr=0x00000000 | ppv_set=1
    // ppv_set follows the result: the class factory writes *ppv only on success.
    assert_eq!(
        dll_get_class_object(&CLSID_STI, &IID_IUNKNOWN, proxy),
        hr(0x0000_0000)
    );

    // R03 | DllGetClassObject(CLSID_Sti, IClassFactory) | hr=0x00000000 | ppv_set=1
    assert_eq!(
        dll_get_class_object(&CLSID_STI, &IID_ICLASSFACTORY, proxy),
        hr(0x0000_0000)
    );

    // R04 | DllGetClassObject(CLSID_Sti, IStillImageW) | hr=0x80004002 | ppv_set=0
    assert_eq!(
        dll_get_class_object(&CLSID_STI, &IID_ISTILLIMAGEW, proxy),
        hr(0x8000_4002)
    );
    assert_eq!(proxy_calls.get(), 0, "CLSID_Sti is not proxied");

    // R05 | DllGetClassObject(other CLSID) | hr=0x80040154 | ppv_set=0
    // The proxy double returns 0x80040154 and does not write *ppv, so ppv_set=0.
    assert_eq!(
        dll_get_class_object(&other_clsid(), &IID_ICLASSFACTORY, proxy),
        hr(0x8004_0154)
    );
    assert_eq!(proxy_calls.get(), 1);

    // R06 | DllRegisterServer(proxy S_OK) | hr=0x00000000 | -
    assert_eq!(dll_register_server(|| hr(0x0000_0000)), hr(0x0000_0000));

    // R07 | DllRegisterServer(proxy E_FAIL) | hr=0x80004005 | -
    assert_eq!(dll_register_server(|| hr(0x8000_4005)), hr(0x8000_4005));

    // R08 | DllUnregisterServer(proxy E_FAIL) | hr=0x80004005 | -
    assert_eq!(dll_unregister_server(|| hr(0x8000_4005)), hr(0x8000_4005));

    // R09 | DllUnregisterServer(proxy S_OK) | hr=0x00000000 | -
    assert_eq!(dll_unregister_server(|| hr(0x0000_0000)), hr(0x0000_0000));
}

#[test]
fn oracle_stiw_no_outer_object() {
    // R10 | StiCreateInstanceA | hr=0x800300FE | -
    assert_eq!(sti_create_instance_a(), hr(0x8003_00FE));

    // R11 | StiCreateInstanceW(no outer) | hr=0x00000000 | ppSti_set=1 count=1
    let (mut obj, face) = sti_create_instance_w(None, true).unwrap();
    assert_eq!(face, Face::StillImageW);
    assert_eq!(obj.ref_count(), 1);

    // R12 | W(no outer).QI(IUnknown) | hr=0x00000000 | q_is_w=0 count=2
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_IUNKNOWN),
        Ok(Some(Face::Inner))
    );
    assert_eq!(obj.ref_count(), 2);

    // R13 | W(no outer).QI(IUnknown) then Release | hr=0x00000000 | release=1
    assert_eq!(obj.release(Face::Inner), 1);

    // R14 | W(no outer).QI(IStillImageW) | hr=0x00000000 | q_is_w=1 count=2
    let (mut obj, _) = sti_create_instance_w(None, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_ISTILLIMAGEW),
        Ok(Some(Face::StillImageW))
    );
    assert_eq!(obj.ref_count(), 2);

    // R15 | W(no outer).QI(IStillImageW) then Release | hr=0x00000000 | release=1
    assert_eq!(obj.release(Face::StillImageW), 1);

    // R16 | W(no outer).QI(IStillImageA) | hr=0x80004002 | q_null=1 count=1
    let (mut obj, _) = sti_create_instance_w(None, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_ISTILLIMAGEA),
        Err(hr(0x8000_4002))
    );
    assert_eq!(obj.ref_count(), 1);

    // R17 | W(no outer).AddRef | hr=0x00000000 | addref=2
    let (mut obj, _) = sti_create_instance_w(None, true).unwrap();
    assert_eq!(obj.add_ref(Face::StillImageW), 2);

    // R18 | W(no outer).Release after AddRef | hr=0x00000000 | release=1
    assert_eq!(obj.release(Face::StillImageW), 1);

    // R19 | W(no outer): QI(IUnknown), Release(W) | hr=0x00000000 | release_w=1
    let (mut obj, _) = sti_create_instance_w(None, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_IUNKNOWN),
        Ok(Some(Face::Inner))
    );
    assert_eq!(obj.release(Face::StillImageW), 1);

    // R20 | W(no outer): then Release(inner) to zero | hr=0x00000000 | release_inner=0
    assert_eq!(obj.release(Face::Inner), 0);

    // R21 | StiCreateInstanceW(no outer, alloc fail) | hr=0x8007000E | ppSti_set=0
    assert_eq!(
        sti_create_instance_w(None, false).map(|(_, f)| f),
        Err(hr(0x8007_000E))
    );
}

#[test]
fn oracle_stiw_with_outer_object() {
    // R22 | StiCreateInstanceW(outer) | hr=0x00000000 | ppSti_set=1 count=1 outer_refs=5
    let (o, state) = outer(S_OK);
    let (mut obj, face) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(face, Face::Inner);
    assert_eq!(obj.ref_count(), 1);
    assert_eq!(state.borrow().refs, 5);

    // R23 | inner(outer).QI(IStillImageW) | hr=0x00000000 | q_is_inner=0 count=1 outer_refs=6
    // The returned pointer is AddRef'd through the outer object, so its counter moves.
    assert_eq!(
        obj.query_interface(Face::Inner, &IID_ISTILLIMAGEW),
        Ok(Some(Face::StillImageW))
    );
    assert_eq!(obj.ref_count(), 1);
    assert_eq!(state.borrow().refs, 6);

    // R24 | W(outer).QI(IUnknown) forwarded S_OK | hr=0x00000000 | q2_set=1 outer_qi_calls=1 outer_refs=6
    // q2_set follows the result: the forwarded query succeeded, so the pointer is set.
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_IUNKNOWN),
        Ok(None)
    );
    assert_eq!(state.borrow().qi_calls, 1);
    assert_eq!(state.borrow().refs, 6);

    // R25 | W(outer).QI(IUnknown) forwarded E_NOINTERFACE | hr=0x80004002 | q2_null=1 outer_qi_calls=1 outer_refs=6
    let (o, state) = outer(hr(0x8000_4002));
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::Inner, &IID_ISTILLIMAGEW),
        Ok(Some(Face::StillImageW))
    );
    assert_eq!(
        obj.query_interface(Face::StillImageW, &IID_IUNKNOWN),
        Err(hr(0x8000_4002))
    );
    assert_eq!(state.borrow().qi_calls, 1);
    assert_eq!(state.borrow().refs, 6);

    // R26 | W(outer).AddRef forwarded | hr=0x00000000 | addref=7 outer_refs=7
    let (o, state) = outer(S_OK);
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(
        obj.query_interface(Face::Inner, &IID_ISTILLIMAGEW),
        Ok(Some(Face::StillImageW))
    );
    assert_eq!(obj.add_ref(Face::StillImageW), 7);
    assert_eq!(state.borrow().refs, 7);

    // R27 | W(outer).Release forwarded | hr=0x00000000 | release=6 outer_refs=6
    assert_eq!(obj.release(Face::StillImageW), 6);
    assert_eq!(state.borrow().refs, 6);

    // R28 | inner(outer).Release to zero | hr=0x00000000 | release=0 outer_refs=5
    let (o, state) = outer(S_OK);
    let (mut obj, _) = sti_create_instance_w(o, true).unwrap();
    assert_eq!(obj.release(Face::Inner), 0);
    assert_eq!(state.borrow().refs, 5);
}

#[test]
fn oracle_class_factory_create_instance() {
    // R29 | cf.CreateInstance(outer, IStillImageW) | hr=0x80040110 | out_null=1 outer_refs=5
    let (o, state) = outer(S_OK);
    assert_eq!(
        class_factory_create_instance(&IID_ISTILLIMAGEW, o, true).map(|(_, f)| f),
        Err(hr(0x8004_0110))
    );
    assert_eq!(state.borrow().refs, 5);

    // R30 | cf.CreateInstance(NULL, IUnknown) | hr=0x00000000 | out_set=1 count=1
    let (mut obj, face) = class_factory_create_instance(&IID_IUNKNOWN, None, true).unwrap();
    assert_eq!(face, Face::Inner);
    assert_eq!(obj.ref_count(), 1);

    // R31 | cf.CreateInstance(NULL, IStillImageW) | hr=0x00000000 | out_set=1 count=1
    let (mut obj2, face) = class_factory_create_instance(&IID_ISTILLIMAGEW, None, true).unwrap();
    assert_eq!(face, Face::StillImageW);
    assert_eq!(obj2.ref_count(), 1);
    assert_eq!(obj2.release(Face::StillImageW), 0);
    assert_eq!(obj.release(Face::Inner), 0);

    // R32 | cf.CreateInstance(NULL, IStillImageA) | hr=0x800300FE | out_null=1
    assert_eq!(
        class_factory_create_instance(&IID_ISTILLIMAGEA, None, true).map(|(_, f)| f),
        Err(hr(0x8003_00FE))
    );

    // R33 | cf.CreateInstance(NULL, IClassFactory) | hr=0x80004002 | out_null=1
    assert_eq!(
        class_factory_create_instance(&IID_ICLASSFACTORY, None, true).map(|(_, f)| f),
        Err(hr(0x8000_4002))
    );

    // R34 | cf.CreateInstance(NULL, IUnknown, alloc fail) | hr=0x8007000E | out_null=1
    assert_eq!(
        class_factory_create_instance(&IID_IUNKNOWN, None, false).map(|(_, f)| f),
        Err(hr(0x8007_000E))
    );

    // R35 | cf.CreateInstance(outer, IUnknown) | hr=0x00000000 | out_set=1 count=1 outer_refs=5
    let (o, state) = outer(S_OK);
    let (obj, face) = class_factory_create_instance(&IID_IUNKNOWN, o, true).unwrap();
    assert_eq!(face, Face::Inner);
    assert_eq!(obj.ref_count(), 1);
    assert_eq!(state.borrow().refs, 5);
}

#[test]
fn oracle_class_factory_vtable() {
    // R36 | cf.QueryInterface(IUnknown) | hr=0x00000000 | same=1
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);

    // R37 | cf.QueryInterface(IClassFactory) | hr=0x00000000 | same=1
    assert_eq!(class_factory_query_interface(&IID_ICLASSFACTORY), S_OK);

    // R38 | cf.QueryInterface(IStillImageW) | hr=0x80004002 | ppv_null=1
    assert_eq!(
        class_factory_query_interface(&IID_ISTILLIMAGEW),
        hr(0x8000_4002)
    );

    // R39 | cf.AddRef | hr=0x00000000 | count=2
    assert_eq!(class_factory_add_ref(), 2);

    // R40 | cf.Release | hr=0x00000000 | count=1
    assert_eq!(class_factory_release(), 1);

    // R41 | cf.LockServer(TRUE) | hr=0x00000000 | -
    assert_eq!(class_factory_lock_server(true), S_OK);

    // R42 | cf.LockServer(FALSE) | hr=0x00000000 | -
    assert_eq!(class_factory_lock_server(false), S_OK);
}

#[test]
fn oracle_not_implemented_methods() {
    // R43 | GetDeviceList | hr=0x80004001 | -
    assert_eq!(get_device_list(), E_NOTIMPL);
    // R44 | GetDeviceInfo | hr=0x80004001 | -
    assert_eq!(get_device_info(), E_NOTIMPL);
    // R45 | CreateDevice | hr=0x80004001 | -
    assert_eq!(create_device(), E_NOTIMPL);
    // R46 | GetDeviceValue | hr=0x80004001 | -
    assert_eq!(get_device_value(), E_NOTIMPL);
    // R47 | SetDeviceValue | hr=0x80004001 | -
    assert_eq!(set_device_value(), E_NOTIMPL);
    // R48 | GetSTILaunchInformation | hr=0x80004001 | -
    assert_eq!(get_sti_launch_information(), E_NOTIMPL);
    // R49 | EnableHwNotifications | hr=0x80004001 | -
    assert_eq!(enable_hw_notifications(), E_NOTIMPL);
    // R50 | GetHwNotificationState | hr=0x80004001 | -
    assert_eq!(get_hw_notification_state(), E_NOTIMPL);
    // R51 | RefreshDeviceBus | hr=0x80004001 | -
    assert_eq!(refresh_device_bus(), E_NOTIMPL);
    // R52 | LaunchApplicationForDevice | hr=0x80004001 | -
    assert_eq!(launch_application_for_device(), E_NOTIMPL);
    // R53 | SetupDeviceParameters | hr=0x80004001 | -
    assert_eq!(setup_device_parameters(), E_NOTIMPL);
    // R54 | WriteToErrorLog | hr=0x80004001 | -
    assert_eq!(write_to_error_log(), E_NOTIMPL);
}

#[test]
fn oracle_register_launch_application() {
    // R55 | RegisterLaunchApplication(create fail 5) | hr=0x80070005 | closes=0
    let mut reg = FakeRegistry {
        create_ret: 5,
        ..Default::default()
    };
    let hr55 = register_launch_application(&mut reg, &wide("app.exe"), &wide("C:\\x.exe"), true);
    assert_eq!(hr55, hr(0x8007_0005));
    assert_eq!(reg.closes, 0);

    // R56 | RegisterLaunchApplication(set fail 2) | hr=0x80070002 | closes=1 name=app.exe
    let mut reg = FakeRegistry {
        set_ret: 2,
        ..Default::default()
    };
    let hr56 = register_launch_application(&mut reg, &wide("app.exe"), &wide("C:\\x.exe"), true);
    assert_eq!(hr56, hr(0x8007_0002));
    assert_eq!(reg.closes, 1);
    assert_eq!(reg.last_name, wide("app.exe"));

    // R57 | RegisterLaunchApplication(ok) | hr=0x00000000 | closes=1 name=app.exe cb=72 data=C:\x.exe /StiDevice:%1 /StiEvent:%2
    let mut reg = FakeRegistry::default();
    let hr57 = register_launch_application(&mut reg, &wide("app.exe"), &wide("C:\\x.exe"), true);
    assert_eq!(hr57, S_OK);
    assert_eq!(reg.closes, 1);
    assert_eq!(reg.last_name, wide("app.exe"));
    let mut data57 = wide("C:\\x.exe /StiDevice:%1 /StiEvent:%2");
    data57.push(0);
    assert_eq!(reg.last_data, data57);
    assert_eq!(reg.last_data.len() * 2, 72);

    // R58 | RegisterLaunchApplication(alloc fail) | hr=0x8007000E | closes=1 set_called=0
    let mut reg = FakeRegistry::default();
    let hr58 = register_launch_application(&mut reg, &wide("app.exe"), &wide("C:\\x.exe"), false);
    assert_eq!(hr58, hr(0x8007_000E));
    assert_eq!(reg.closes, 1);
    assert_eq!(reg.set_calls, 0);

    // R59 | RegisterLaunchApplication(embedded NUL) | hr=0x00000000 | closes=1 cb=72 data=C:\a.exe /StiDevice:%1 /StiEvent:%2
    // The command line ends at its first NUL, so the "junk" after it is not written.
    let mut reg = FakeRegistry::default();
    let mut cmd59 = wide("C:\\a.exe");
    cmd59.push(0);
    cmd59.extend(wide("junk"));
    let hr59 = register_launch_application(&mut reg, &wide("app.exe"), &cmd59, true);
    assert_eq!(hr59, S_OK);
    assert_eq!(reg.closes, 1);
    let mut data59 = wide("C:\\a.exe /StiDevice:%1 /StiEvent:%2");
    data59.push(0);
    assert_eq!(reg.last_data, data59);
    assert_eq!(reg.last_data.len() * 2, 72);
}

#[test]
fn oracle_unregister_launch_application() {
    // R60 | UnregisterLaunchApplication(create fail 5) | hr=0x80070005 | closes=0
    let mut reg = FakeRegistry {
        create_ret: 5,
        ..Default::default()
    };
    assert_eq!(
        unregister_launch_application(&mut reg, &wide("app.exe")),
        hr(0x8007_0005)
    );
    assert_eq!(reg.closes, 0);

    // R61 | UnregisterLaunchApplication(delete fail 2) | hr=0x80070002 | closes=1 name=app.exe
    let mut reg = FakeRegistry {
        delete_ret: 2,
        ..Default::default()
    };
    assert_eq!(
        unregister_launch_application(&mut reg, &wide("app.exe")),
        hr(0x8007_0002)
    );
    assert_eq!(reg.closes, 1);
    assert_eq!(reg.last_name, wide("app.exe"));

    // R62 | UnregisterLaunchApplication(ok) | hr=0x00000000 | closes=1 name=app.exe
    let mut reg = FakeRegistry::default();
    assert_eq!(
        unregister_launch_application(&mut reg, &wide("app.exe")),
        S_OK
    );
    assert_eq!(reg.closes, 1);
    assert_eq!(reg.last_name, wide("app.exe"));

    // R63 | UnregisterLaunchApplication(create 0x80000001) | hr=0x80000001 | closes=0
    // A code that is negative as an HRESULT passes through HRESULT_FROM_WIN32 unchanged.
    let mut reg = FakeRegistry {
        create_ret: 0x8000_0001,
        ..Default::default()
    };
    assert_eq!(
        unregister_launch_application(&mut reg, &wide("app.exe")),
        hr(0x8000_0001)
    );
    assert_eq!(reg.closes, 0);
}
