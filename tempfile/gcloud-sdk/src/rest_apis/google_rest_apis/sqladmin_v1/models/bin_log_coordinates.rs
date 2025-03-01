/*
 * Cloud SQL Admin API
 *
 * API for Cloud SQL database instance management
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// BinLogCoordinates : Binary log coordinates.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinLogCoordinates {
    /// Name of the binary log file for a Cloud SQL instance.
    #[serde(rename = "binLogFileName", skip_serializing_if = "Option::is_none")]
    pub bin_log_file_name: Option<String>,
    /// Position (offset) within the binary log file.
    #[serde(rename = "binLogPosition", skip_serializing_if = "Option::is_none")]
    pub bin_log_position: Option<String>,
    /// This is always `sql#binLogCoordinates`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl BinLogCoordinates {
    /// Binary log coordinates.
    pub fn new() -> BinLogCoordinates {
        BinLogCoordinates {
            bin_log_file_name: None,
            bin_log_position: None,
            kind: None,
        }
    }
}
