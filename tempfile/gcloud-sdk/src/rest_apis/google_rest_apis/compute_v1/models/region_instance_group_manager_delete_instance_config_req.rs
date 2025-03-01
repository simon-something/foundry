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

/// RegionInstanceGroupManagerDeleteInstanceConfigReq : RegionInstanceGroupManagers.deletePerInstanceConfigs
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionInstanceGroupManagerDeleteInstanceConfigReq {
    /// The list of instance names for which we want to delete per-instance configs on this managed instance group.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

impl RegionInstanceGroupManagerDeleteInstanceConfigReq {
    /// RegionInstanceGroupManagers.deletePerInstanceConfigs
    pub fn new() -> RegionInstanceGroupManagerDeleteInstanceConfigReq {
        RegionInstanceGroupManagerDeleteInstanceConfigReq { names: None }
    }
}
