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
pub struct InstanceGroupManagerVersion {
    /// The URL of the instance template that is specified for this managed instance group. The group uses this template to create new instances in the managed instance group until the `targetSize` for this version is reached. The templates for existing instances in the group do not change unless you run recreateInstances, run applyUpdatesToInstances, or set the group's updatePolicy.type to PROACTIVE; in those cases, existing instances are updated until the `targetSize` for this version is reached.
    #[serde(rename = "instanceTemplate", skip_serializing_if = "Option::is_none")]
    pub instance_template: Option<String>,
    /// Name of the version. Unique among all versions in the scope of this managed instance group.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "targetSize", skip_serializing_if = "Option::is_none")]
    pub target_size: Option<Box<models::FixedOrPercent>>,
}

impl InstanceGroupManagerVersion {
    pub fn new() -> InstanceGroupManagerVersion {
        InstanceGroupManagerVersion {
            instance_template: None,
            name: None,
            target_size: None,
        }
    }
}
