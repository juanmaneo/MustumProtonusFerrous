/// The two addons `addons.c` installs, with the values its `__x86_64__` branches select.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addon {
    Gecko,
    Mono,
}

impl Addon {
    /// `GECKO_VERSION` or `MONO_VERSION`.
    pub fn version(self) -> &'static str {
        match self {
            Self::Gecko => "2.47.4",
            Self::Mono => "11.3.0",
        }
    }

    /// `GECKO_ARCH` or `MONO_ARCH`.
    pub fn arch(self) -> &'static str {
        match self {
            Self::Gecko => "x86_64",
            Self::Mono => "x86",
        }
    }

    /// `GECKO_SHA` or `MONO_SHA`: the lowercase hex SHA-256 of the MSI.
    pub fn sha(self) -> &'static str {
        match self {
            Self::Gecko => "e590b7d988a32d6aa4cf1d8aa3aa3d33766fdd4cf4c89c2dcc2095ecb28d066f",
            Self::Mono => "df2dfc1665c2511882e7cabd56eafd0c0a3d94e5a7e86f969277f6c189d418d3",
        }
    }

    /// `url_default`: the download URL used when the registry has none.
    pub fn url_default(self) -> &'static str {
        match self {
            Self::Gecko => "http://source.winehq.org/winegecko.php",
            Self::Mono => "http://source.winehq.org/winemono.php",
        }
    }

    /// `subdir_name`: the directory under the Wine data directory.
    pub fn subdir(self) -> &'static str {
        match self {
            Self::Gecko => "gecko",
            Self::Mono => "mono",
        }
    }

    /// `file_name`: the MSI file name.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Gecko => "wine-gecko-2.47.4-x86_64.msi",
            Self::Mono => "wine-mono-11.3.0-x86.msi",
        }
    }
}
