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
pub struct InstancesSetNameRequest {
    /// The current name of this resource, used to prevent conflicts. Provide the latest name when making a request to change name.
    #[serde(rename = "currentName", skip_serializing_if = "Option::is_none")]
    pub current_name: Option<String>,
    /// The name to be applied to the instance. Needs to be RFC 1035 compliant.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl InstancesSetNameRequest {
    pub fn new() -> InstancesSetNameRequest {
        InstancesSetNameRequest {
            current_name: None,
            name: None,
        }
    }
}
