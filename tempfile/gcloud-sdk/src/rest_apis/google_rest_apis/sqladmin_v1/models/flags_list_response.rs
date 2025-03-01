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

/// FlagsListResponse : Flags list response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlagsListResponse {
    /// List of flags.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::Flag>>,
    /// This is always `sql#flagsList`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl FlagsListResponse {
    /// Flags list response.
    pub fn new() -> FlagsListResponse {
        FlagsListResponse {
            items: None,
            kind: None,
        }
    }
}
