/*
 * Compute Engine API
 *
 * Creates and runs virtual machines on Google Cloud Platform.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreservedStatePreservedNetworkIp {
    /// These stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted.
    #[serde(rename = "autoDelete", skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<AutoDelete>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<Box<models::PreservedStatePreservedNetworkIpIpAddress>>,
}

impl PreservedStatePreservedNetworkIp {
    pub fn new() -> PreservedStatePreservedNetworkIp {
        PreservedStatePreservedNetworkIp {
            auto_delete: None,
            ip_address: None,
        }
    }
}
/// These stateful IPs will never be released during autohealing, update or VM instance recreate operations. This flag is used to configure if the IP reservation should be deleted after it is no longer used by the group, e.g. when the given instance or the whole group is deleted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AutoDelete {
    #[serde(rename = "NEVER")]
    Never,
    #[serde(rename = "ON_PERMANENT_INSTANCE_DELETION")]
    OnPermanentInstanceDeletion,
}

impl Default for AutoDelete {
    fn default() -> AutoDelete {
        Self::Never
    }
}
