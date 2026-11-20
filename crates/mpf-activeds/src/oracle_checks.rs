use super::{
    Pathname, ADS_FORMAT_PROVIDER, ADS_FORMAT_SERVER, ADS_FORMAT_X500_DN, ADS_SETTYPE_FULL,
    E_ADS_BAD_PATHNAME, S_OK,
};

#[test]
fn wine_parse_path_oracle_rows() {
    let rows = [
        ("LDAP:", S_OK, Some("LDAP"), None, None),
        ("ldap:", S_OK, Some("ldap"), None, None),
        ("LdAp:", S_OK, Some("LdAp"), None, None),
        ("LDAP://host", S_OK, Some("LDAP"), Some("host"), None),
        (
            "LDAP://host/dc=example",
            S_OK,
            Some("LDAP"),
            Some("host"),
            Some("dc=example"),
        ),
        (
            "LDAP://host:389/cn=Alice,ou=People",
            S_OK,
            Some("LDAP"),
            Some("host:389"),
            Some("cn=Alice,ou=People"),
        ),
        ("LDAP:///dn", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP:///", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP://host/", E_ADS_BAD_PATHNAME, None, None, None),
        (
            "LDAP://host//dn",
            S_OK,
            Some("LDAP"),
            Some("host"),
            Some("/dn"),
        ),
        (
            "LDAP://host/dn/",
            S_OK,
            Some("LDAP"),
            Some("host"),
            Some("dn/"),
        ),
        (
            "LDAP://host/a/b",
            S_OK,
            Some("LDAP"),
            Some("host"),
            Some("a/b"),
        ),
        (
            "LDAP://host,foo/dn",
            S_OK,
            Some("LDAP"),
            Some("host,foo"),
            Some("dn"),
        ),
        (
            "LDAP:// host/dn",
            S_OK,
            Some("LDAP"),
            Some(" host"),
            Some("dn"),
        ),
        ("ldapx:", E_ADS_BAD_PATHNAME, None, None, None),
        ("HTTP://host/dn", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP:/x", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP:////dn", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP://?", S_OK, Some("LDAP"), Some("?"), None),
        ("LDAP://host?x", S_OK, Some("LDAP"), Some("host?x"), None),
        (" LDAP://host", E_ADS_BAD_PATHNAME, None, None, None),
        ("LDAP://host//", S_OK, Some("LDAP"), Some("host"), Some("/")),
    ];

    for (input, expected_hr, provider, server, dn) in rows {
        let mut path = Pathname::default();
        assert_eq!(path.set(input, ADS_SETTYPE_FULL), expected_hr, "{input}");
        if expected_hr == S_OK {
            assert_eq!(
                path.retrieve(ADS_FORMAT_PROVIDER).as_deref(),
                Ok(provider.unwrap())
            );
            assert_eq!(path.retrieve(ADS_FORMAT_SERVER).ok().as_deref(), server);
            assert_eq!(
                path.retrieve(ADS_FORMAT_X500_DN).as_deref(),
                Ok(dn.unwrap_or(""))
            );
        } else {
            assert_eq!(path.retrieve(ADS_FORMAT_PROVIDER).as_deref(), Ok("LDAP"));
            assert_eq!(path.retrieve(ADS_FORMAT_SERVER), Err(super::E_OUTOFMEMORY));
            assert_eq!(path.retrieve(ADS_FORMAT_X500_DN).as_deref(), Ok(""));
        }
    }
}
