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

/// HttpHeaderOption : Specification determining how headers are added to requests or responses.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaderOption {
    /// The name of the header.
    #[serde(rename = "headerName", skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    /// The value of the header to add.
    #[serde(rename = "headerValue", skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
    /// If false, headerValue is appended to any values that already exist for the header. If true, headerValue is set for the header, discarding any values that were set for that header. The default value is false.
    #[serde(rename = "replace", skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
}

impl HttpHeaderOption {
    /// Specification determining how headers are added to requests or responses.
    pub fn new() -> HttpHeaderOption {
        HttpHeaderOption {
            header_name: None,
            header_value: None,
            replace: None,
        }
    }
}
