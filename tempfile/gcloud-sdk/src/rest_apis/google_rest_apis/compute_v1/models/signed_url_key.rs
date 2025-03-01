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

/// SignedUrlKey : Represents a customer-supplied Signing Key used by Cloud CDN Signed URLs
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedUrlKey {
    /// Name of the key. The name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// 128-bit key value used for signing the URL. The key value must be a valid RFC 4648 Section 5 base64url encoded string.
    #[serde(rename = "keyValue", skip_serializing_if = "Option::is_none")]
    pub key_value: Option<String>,
}

impl SignedUrlKey {
    /// Represents a customer-supplied Signing Key used by Cloud CDN Signed URLs
    pub fn new() -> SignedUrlKey {
        SignedUrlKey {
            key_name: None,
            key_value: None,
        }
    }
}
