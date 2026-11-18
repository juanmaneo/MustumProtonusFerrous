//! Edge cases of the dispatch that the Wine oracle vectors do not reach.

use super::*;

const CLASSES: [(ComClass, Guid); 4] = [
    (ComClass::UpdateSession, CLSID_UPDATE_SESSION),
    (ComClass::AutomaticUpdates, CLSID_AUTOMATIC_UPDATES),
    (ComClass::SystemInformation, CLSID_SYSTEM_INFORMATION),
    (ComClass::UpdateInstaller, CLSID_UPDATE_INSTALLER),
];

const OBJECTS: [(ComObject, Guid); 6] = [
    (ComObject::UpdateSession, IID_IUPDATE_SESSION),
    (ComObject::UpdateSearcher, IID_IUPDATE_SEARCHER),
    (ComObject::UpdateDownloader, IID_IUPDATE_DOWNLOADER),
    (ComObject::UpdateInstaller, IID_IUPDATE_INSTALLER),
    (ComObject::AutomaticUpdates, IID_IAUTOMATIC_UPDATES),
    (ComObject::SystemInformation, IID_ISYSTEM_INFORMATION),
];

/// The GUID with the last byte of `data4` changed, so it differs from `iid` in one bit.
fn near_miss(iid: &Guid) -> Guid {
    let mut out = *iid;
    out.data4[7] ^= 1;
    out
}

#[test]
fn every_class_is_found_by_its_clsid() {
    for (class, clsid) in CLASSES {
        assert_eq!(class_for_clsid(&clsid), Some(class));
        assert_eq!(dll_get_class_object(&clsid, &IID_IUNKNOWN), S_OK);
    }
}

#[test]
fn each_class_creates_its_own_object() {
    assert_eq!(ComClass::UpdateSession.object(), ComObject::UpdateSession);
    assert_eq!(
        ComClass::AutomaticUpdates.object(),
        ComObject::AutomaticUpdates
    );
    assert_eq!(
        ComClass::SystemInformation.object(),
        ComObject::SystemInformation
    );
    assert_eq!(
        ComClass::UpdateInstaller.object(),
        ComObject::UpdateInstaller
    );
}

#[test]
fn near_miss_clsids_are_not_registered() {
    for (_, clsid) in CLASSES {
        let miss = near_miss(&clsid);
        assert_eq!(class_for_clsid(&miss), None);
        assert_eq!(
            dll_get_class_object(&miss, &IID_IUNKNOWN),
            CLASS_E_CLASSNOTAVAILABLE
        );
    }
}

#[test]
fn interface_ids_are_not_classes() {
    for (_, iid) in OBJECTS {
        assert_eq!(class_for_clsid(&iid), None);
    }
    assert_eq!(
        dll_get_class_object(&IID_IUPDATE_SESSION, &IID_IUNKNOWN),
        CLASS_E_CLASSNOTAVAILABLE
    );
}

#[test]
fn class_factory_answers_only_unknown_and_class_factory() {
    assert_eq!(class_factory_query_interface(&IID_IUNKNOWN), S_OK);
    assert_eq!(class_factory_query_interface(&IID_ICLASS_FACTORY), S_OK);
    assert_eq!(class_factory_query_interface(&IID_IDISPATCH), E_NOINTERFACE);
    assert_eq!(
        class_factory_query_interface(&near_miss(&IID_IUNKNOWN)),
        E_NOINTERFACE
    );
    assert_eq!(
        class_factory_query_interface(&near_miss(&IID_ICLASS_FACTORY)),
        E_NOINTERFACE
    );
    for (_, iid) in OBJECTS {
        assert_eq!(class_factory_query_interface(&iid), E_NOINTERFACE);
    }
}

#[test]
fn registered_class_factories_reject_object_interfaces() {
    for (_, clsid) in CLASSES {
        for (_, iid) in OBJECTS {
            assert_eq!(dll_get_class_object(&clsid, &iid), E_NOINTERFACE);
        }
    }
}

#[test]
fn object_accepts_own_interface_idispatch_and_iunknown() {
    for (object, own) in OBJECTS {
        assert_eq!(object_query_interface(object, &own), S_OK);
        assert_eq!(object_query_interface(object, &IID_IDISPATCH), S_OK);
        assert_eq!(object_query_interface(object, &IID_IUNKNOWN), S_OK);
        assert_eq!(
            object_query_interface(object, &near_miss(&own)),
            E_NOINTERFACE
        );
        assert_eq!(
            object_query_interface(object, &IID_ICLASS_FACTORY),
            E_NOINTERFACE
        );
    }
}

#[test]
fn object_rejects_the_interfaces_of_other_objects() {
    for (object, own) in OBJECTS {
        for (_, other) in OBJECTS {
            if other != own {
                assert_eq!(object_query_interface(object, &other), E_NOINTERFACE);
            }
        }
    }
}

#[test]
fn create_instance_refuses_an_outer_unknown_before_checking_the_iid() {
    for (class, _) in CLASSES {
        for iid in [IID_IUNKNOWN, IID_IDISPATCH, near_miss(&IID_IUNKNOWN)] {
            assert_eq!(create_instance(class, true, &iid), CLASS_E_NOAGGREGATION);
        }
    }
}

#[test]
fn create_instance_queries_the_object_the_class_creates() {
    for (class, own) in [
        (ComClass::UpdateSession, IID_IUPDATE_SESSION),
        (ComClass::AutomaticUpdates, IID_IAUTOMATIC_UPDATES),
        (ComClass::SystemInformation, IID_ISYSTEM_INFORMATION),
        (ComClass::UpdateInstaller, IID_IUPDATE_INSTALLER),
    ] {
        assert_eq!(create_instance(class, false, &own), S_OK);
        assert_eq!(create_instance(class, false, &IID_IUNKNOWN), S_OK);
        assert_eq!(create_instance(class, false, &IID_IDISPATCH), S_OK);
        assert_eq!(
            create_instance(class, false, &IID_ICLASS_FACTORY),
            E_NOINTERFACE
        );
        assert_eq!(
            create_instance(class, false, &near_miss(&own)),
            E_NOINTERFACE
        );
    }
}

#[test]
fn guid_equality_compares_every_field() {
    let mut differs = [IID_IUNKNOWN; 4];
    differs[0].data1 ^= 1;
    differs[1].data2 ^= 1;
    differs[2].data3 ^= 1;
    differs[3].data4[0] ^= 1;
    for other in differs {
        assert_ne!(IID_IUNKNOWN, other);
    }
    assert_eq!(IID_IUNKNOWN, Clone::clone(&IID_IUNKNOWN));
}

#[test]
fn guid_debug_prints_every_field() {
    assert_eq!(
        format!("{:?}", IID_IUNKNOWN),
        "Guid { data1: 0, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] }"
    );
}

#[test]
fn enums_debug_print_their_variant_and_compare_by_value() {
    assert_eq!(format!("{:?}", ComClass::UpdateSession), "UpdateSession");
    assert_eq!(
        format!("{:?}", ComClass::AutomaticUpdates),
        "AutomaticUpdates"
    );
    assert_eq!(
        format!("{:?}", ComClass::SystemInformation),
        "SystemInformation"
    );
    assert_eq!(
        format!("{:?}", ComClass::UpdateInstaller),
        "UpdateInstaller"
    );
    assert_eq!(format!("{:?}", ComObject::UpdateSession), "UpdateSession");
    assert_eq!(format!("{:?}", ComObject::UpdateSearcher), "UpdateSearcher");
    assert_eq!(
        format!("{:?}", ComObject::UpdateDownloader),
        "UpdateDownloader"
    );
    assert_eq!(
        format!("{:?}", ComObject::UpdateInstaller),
        "UpdateInstaller"
    );
    assert_eq!(
        format!("{:?}", ComObject::AutomaticUpdates),
        "AutomaticUpdates"
    );
    assert_eq!(
        format!("{:?}", ComObject::SystemInformation),
        "SystemInformation"
    );

    assert_eq!(
        Clone::clone(&ComClass::UpdateSession),
        ComClass::UpdateSession
    );
    assert_eq!(
        Clone::clone(&ComObject::UpdateSearcher),
        ComObject::UpdateSearcher
    );
    assert_ne!(ComClass::UpdateSession, ComClass::UpdateInstaller);
    assert_ne!(ComObject::UpdateSession, ComObject::UpdateSearcher);
}

#[test]
fn hresults_match_win32_values() {
    assert_eq!(S_OK as u32, 0x0000_0000);
    assert_eq!(E_NOINTERFACE as u32, 0x8000_4002);
    assert_eq!(CLASS_E_NOAGGREGATION as u32, 0x8004_0110);
    assert_eq!(CLASS_E_CLASSNOTAVAILABLE as u32, 0x8004_0111);
}

#[test]
fn guid_constructor_keeps_the_field_order() {
    let built = guid(0x0102_0304, 0x0506, 0x0708, [9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(
        (built.data1, built.data2, built.data3),
        (0x0102_0304, 0x0506, 0x0708)
    );
    assert_eq!(built.data4, [9, 10, 11, 12, 13, 14, 15, 16]);
}
