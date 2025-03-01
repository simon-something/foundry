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

/// ResourcePolicyHourlyCycle : Time window specified for hourly operations.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcePolicyHourlyCycle {
    /// [Output only] Duration of the time window, automatically chosen to be smallest possible in the given scenario.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Defines a schedule with units measured in hours. The value determines how many hours pass between the start of each cycle.
    #[serde(rename = "hoursInCycle", skip_serializing_if = "Option::is_none")]
    pub hours_in_cycle: Option<i32>,
    /// Time within the window to start the operations. It must be in format \"HH:MM\", where HH : [00-23] and MM : [00-00] GMT.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

impl ResourcePolicyHourlyCycle {
    /// Time window specified for hourly operations.
    pub fn new() -> ResourcePolicyHourlyCycle {
        ResourcePolicyHourlyCycle {
            duration: None,
            hours_in_cycle: None,
            start_time: None,
        }
    }
}
