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

/// RegionInstanceGroupManagerUpdateInstanceConfigReq : RegionInstanceGroupManagers.updatePerInstanceConfigs
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionInstanceGroupManagerUpdateInstanceConfigReq {
    /// The list of per-instance configurations to insert or patch on this managed instance group.
    #[serde(rename = "perInstanceConfigs", skip_serializing_if = "Option::is_none")]
    pub per_instance_configs: Option<Vec<models::PerInstanceConfig>>,
}

impl RegionInstanceGroupManagerUpdateInstanceConfigReq {
    /// RegionInstanceGroupManagers.updatePerInstanceConfigs
    pub fn new() -> RegionInstanceGroupManagerUpdateInstanceConfigReq {
        RegionInstanceGroupManagerUpdateInstanceConfigReq {
            per_instance_configs: None,
        }
    }
}
