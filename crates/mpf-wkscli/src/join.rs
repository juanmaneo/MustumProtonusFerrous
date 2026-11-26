use crate::status::{ERROR_INVALID_PARAMETER, NERR_SUCCESS};

/// `NetSetupWorkgroupName` from the `NETSETUP_JOIN_STATUS` enum in `include/lmjoin.h`.
pub const NET_SETUP_WORKGROUP_NAME: u32 = 2;

/// The `Workgroup` name the C body copies into the buffer, with its terminating NUL.
fn workgroup_name() -> Vec<u16> {
    "Workgroup"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}

/// `NetGetJoinInformation` from `dlls/wkscli/wkscli.c`. `name` and `join_type` are the `Name` and
/// `type` out-parameters, and `None` stands for a NULL pointer. The C body only traces the
/// `Server` argument, so the port does not take it.
pub fn net_get_join_information(
    name: Option<&mut Option<Vec<u16>>>,
    join_type: Option<&mut u32>,
) -> u32 {
    let (Some(name), Some(join_type)) = (name, join_type) else {
        return ERROR_INVALID_PARAMETER;
    };
    *name = Some(workgroup_name());
    *join_type = NET_SETUP_WORKGROUP_NAME;
    NERR_SUCCESS
}
