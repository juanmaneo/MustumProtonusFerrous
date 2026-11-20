use super::{
    Pathname, ADS_FORMAT_LEAF, ADS_FORMAT_PROVIDER, ADS_FORMAT_SERVER, ADS_FORMAT_X500,
    ADS_FORMAT_X500_DN, ADS_SETTYPE_DN, ADS_SETTYPE_FULL, ADS_SETTYPE_PROVIDER, ADS_SETTYPE_SERVER,
    E_ADS_BAD_PATHNAME, E_INVALIDARG, E_OUTOFMEMORY, HRESULT_ERROR_INVALID_INDEX, S_OK,
};

#[test]
fn default_and_retrieve_formats() {
    let path = Pathname::default();
    assert_eq!(path.retrieve(ADS_FORMAT_PROVIDER), Ok("LDAP".to_owned()));
    assert_eq!(path.retrieve(ADS_FORMAT_SERVER), Err(E_OUTOFMEMORY));
    assert_eq!(path.retrieve(ADS_FORMAT_X500_DN), Ok(String::new()));
    assert_eq!(path.retrieve(ADS_FORMAT_LEAF), Ok(String::new()));
    assert_eq!(path.retrieve(ADS_FORMAT_X500), Ok("LDAP://".to_owned()));
    assert_eq!(path.retrieve(-1), Ok("LDAP://".to_owned()));
    assert_eq!(path.num_elements(), 0);
    assert_eq!(path.element(0), Err(HRESULT_ERROR_INVALID_INDEX));
}

#[test]
fn set_component_types_and_retrieve_composition() {
    let mut path = Pathname::default();
    assert_eq!(path.set("WinNT", ADS_SETTYPE_PROVIDER), S_OK);
    assert_eq!(path.set("server", ADS_SETTYPE_SERVER), S_OK);
    assert_eq!(path.set("cn=User", ADS_SETTYPE_DN), S_OK);
    assert_eq!(path.retrieve(ADS_FORMAT_PROVIDER), Ok("WinNT".to_owned()));
    assert_eq!(path.retrieve(ADS_FORMAT_SERVER), Ok("server".to_owned()));
    assert_eq!(path.retrieve(ADS_FORMAT_X500_DN), Ok("cn=User".to_owned()));
    assert_eq!(
        path.retrieve(ADS_FORMAT_X500),
        Ok("WinNT://server/cn=User".to_owned())
    );
    assert_eq!(path.retrieve(ADS_FORMAT_LEAF), Ok("cn=User".to_owned()));
    assert_eq!(path.num_elements(), 1);
    assert_eq!(path.element(0), Ok("cn=User".to_owned()));

    assert_eq!(path.set("", ADS_SETTYPE_SERVER), S_OK);
    assert_eq!(path.set("", ADS_SETTYPE_DN), S_OK);
    assert_eq!(path.retrieve(ADS_FORMAT_SERVER), Ok(String::new()));
    assert_eq!(path.retrieve(ADS_FORMAT_X500), Ok("WinNT:///".to_owned()));
    assert_eq!(path.retrieve(ADS_FORMAT_LEAF), Ok(String::new()));
    assert_eq!(path.num_elements(), 1);
    assert_eq!(path.element(0), Ok(String::new()));
}

#[test]
fn element_splitting_and_invalid_indices() {
    let mut path = Pathname::default();
    assert_eq!(path.set("a,,c,", ADS_SETTYPE_DN), S_OK);
    assert_eq!(path.retrieve(ADS_FORMAT_LEAF), Ok("a".to_owned()));
    assert_eq!(path.retrieve(ADS_FORMAT_X500_DN), Ok("a,,c,".to_owned()));
    assert_eq!(path.num_elements(), 4);
    assert_eq!(path.element(0), Ok("a".to_owned()));
    assert_eq!(path.element(1), Ok(String::new()));
    assert_eq!(path.element(2), Ok("c".to_owned()));
    assert_eq!(path.element(3), Ok(String::new()));
    assert_eq!(path.element(-1), Err(HRESULT_ERROR_INVALID_INDEX));
    assert_eq!(path.element(4), Err(HRESULT_ERROR_INVALID_INDEX));
}

#[test]
fn invalid_type_and_failed_full_set_leave_state_intact() {
    let mut path = Pathname::default();
    assert_eq!(path.set("LDAP://server/dn", ADS_SETTYPE_FULL), S_OK);
    let before = path.clone();
    assert_eq!(path.set("LDAP:/bad", ADS_SETTYPE_FULL), E_ADS_BAD_PATHNAME);
    assert_eq!(path, before);
    assert_eq!(path.set("value", 0), E_INVALIDARG);
    assert_eq!(path, before);
}

#[test]
fn empty_provider_and_missing_optional_components() {
    let mut path = Pathname::default();
    assert_eq!(path.set("", ADS_SETTYPE_PROVIDER), S_OK);
    assert_eq!(path.retrieve(ADS_FORMAT_PROVIDER), Ok(String::new()));
    assert_eq!(path.retrieve(ADS_FORMAT_X500), Ok("://".to_owned()));
    assert_eq!(path.set("LDAP://only-server", ADS_SETTYPE_FULL), S_OK);
    assert_eq!(
        path.retrieve(ADS_FORMAT_X500),
        Ok("LDAP://only-server/".to_owned())
    );
    assert_eq!(path.retrieve(ADS_FORMAT_X500_DN), Ok(String::new()));
    assert_eq!(path.num_elements(), 0);
}
