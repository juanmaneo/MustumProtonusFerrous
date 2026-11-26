use crate::codes::DF_DS2;

/// The fields of `TW_IDENTITY` that the Source Manager reads and writes.
#[derive(Clone, Debug, PartialEq)]
pub struct Identity {
    pub id: u32,
    pub supported_groups: u32,
    pub product_name: String,
}

impl Identity {
    pub fn new(id: u32, supported_groups: u32, product_name: &str) -> Self {
        Self {
            id,
            supported_groups,
            product_name: product_name.to_owned(),
        }
    }

    pub fn has_ds2(&self) -> bool {
        (self.supported_groups & DF_DS2) != 0
    }

    /// The identity the Source Manager keeps for an open source: `OpenDS` copies only `Id` and
    /// `ProductName`, so `SupportedGroups` is 0 in the stored copy.
    pub fn active_copy(&self) -> Self {
        Self::new(self.id, 0, &self.product_name)
    }
}
