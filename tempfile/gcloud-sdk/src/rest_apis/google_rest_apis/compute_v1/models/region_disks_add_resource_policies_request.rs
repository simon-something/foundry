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
pub struct RegionDisksAddResourcePoliciesRequest {
    /// Resource policies to be added to this disk.
    #[serde(rename = "resourcePolicies", skip_serializing_if = "Option::is_none")]
    pub resource_policies: Option<Vec<String>>,
}

impl RegionDisksAddResourcePoliciesRequest {
    pub fn new() -> RegionDisksAddResourcePoliciesRequest {
        RegionDisksAddResourcePoliciesRequest {
            resource_policies: None,
        }
    }
}
