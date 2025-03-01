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

/// AutoscalingPolicyScaleInControl : Configuration that allows for slower scale in so that even if Autoscaler recommends an abrupt scale in of a MIG, it will be throttled as specified by the parameters below.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscalingPolicyScaleInControl {
    #[serde(
        rename = "maxScaledInReplicas",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_scaled_in_replicas: Option<Box<models::FixedOrPercent>>,
    /// How far back autoscaling looks when computing recommendations to include directives regarding slower scale in, as described above.
    #[serde(rename = "timeWindowSec", skip_serializing_if = "Option::is_none")]
    pub time_window_sec: Option<i32>,
}

impl AutoscalingPolicyScaleInControl {
    /// Configuration that allows for slower scale in so that even if Autoscaler recommends an abrupt scale in of a MIG, it will be throttled as specified by the parameters below.
    pub fn new() -> AutoscalingPolicyScaleInControl {
        AutoscalingPolicyScaleInControl {
            max_scaled_in_replicas: None,
            time_window_sec: None,
        }
    }
}
