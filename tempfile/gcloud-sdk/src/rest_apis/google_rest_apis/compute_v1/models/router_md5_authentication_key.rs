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
pub struct RouterMd5AuthenticationKey {
    /// [Input only] Value of the key. For patch and update calls, it can be skipped to copy the value from the previous configuration. This is allowed if the key with the same name existed before the operation. Maximum length is 80 characters. Can only contain printable ASCII characters.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Name used to identify the key. Must be unique within a router. Must be referenced by exactly one bgpPeer. Must comply with RFC1035.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl RouterMd5AuthenticationKey {
    pub fn new() -> RouterMd5AuthenticationKey {
        RouterMd5AuthenticationKey {
            key: None,
            name: None,
        }
    }
}
