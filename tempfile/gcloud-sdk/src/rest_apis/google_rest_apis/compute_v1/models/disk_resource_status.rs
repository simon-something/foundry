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
pub struct DiskResourceStatus {
    #[serde(rename = "asyncPrimaryDisk", skip_serializing_if = "Option::is_none")]
    pub async_primary_disk: Option<Box<models::DiskResourceStatusAsyncReplicationStatus>>,
    /// Key: disk, value: AsyncReplicationStatus message
    #[serde(
        rename = "asyncSecondaryDisks",
        skip_serializing_if = "Option::is_none"
    )]
    pub async_secondary_disks:
        Option<std::collections::HashMap<String, models::DiskResourceStatusAsyncReplicationStatus>>,
}

impl DiskResourceStatus {
    pub fn new() -> DiskResourceStatus {
        DiskResourceStatus {
            async_primary_disk: None,
            async_secondary_disks: None,
        }
    }
}
