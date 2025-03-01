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
pub struct InstanceGroupManagersSetInstanceTemplateRequest {
    /// The URL of the instance template that is specified for this managed instance group. The group uses this template to create all new instances in the managed instance group. The templates for existing instances in the group do not change unless you run recreateInstances, run applyUpdatesToInstances, or set the group's updatePolicy.type to PROACTIVE.
    #[serde(rename = "instanceTemplate", skip_serializing_if = "Option::is_none")]
    pub instance_template: Option<String>,
}

impl InstanceGroupManagersSetInstanceTemplateRequest {
    pub fn new() -> InstanceGroupManagersSetInstanceTemplateRequest {
        InstanceGroupManagersSetInstanceTemplateRequest {
            instance_template: None,
        }
    }
}
