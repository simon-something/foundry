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
pub struct InstantSnapshotResourceStatus {
    /// [Output Only] The storage size of this instant snapshot.
    #[serde(rename = "storageSizeBytes", skip_serializing_if = "Option::is_none")]
    pub storage_size_bytes: Option<String>,
}

impl InstantSnapshotResourceStatus {
    pub fn new() -> InstantSnapshotResourceStatus {
        InstantSnapshotResourceStatus {
            storage_size_bytes: None,
        }
    }
}
