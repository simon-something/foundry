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

/// FixedOrPercent : Encapsulates numeric value that can be either absolute or relative.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedOrPercent {
    /// [Output Only] Absolute value of VM instances calculated based on the specific mode. - If the value is fixed, then the calculated value is equal to the fixed value. - If the value is a percent, then the calculated value is percent/100 * targetSize. For example, the calculated value of a 80% of a managed instance group with 150 instances would be (80/100 * 150) = 120 VM instances. If there is a remainder, the number is rounded.
    #[serde(rename = "calculated", skip_serializing_if = "Option::is_none")]
    pub calculated: Option<i32>,
    /// Specifies a fixed number of VM instances. This must be a positive integer.
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
    pub fixed: Option<i32>,
    /// Specifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%.
    #[serde(rename = "percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
}

impl FixedOrPercent {
    /// Encapsulates numeric value that can be either absolute or relative.
    pub fn new() -> FixedOrPercent {
        FixedOrPercent {
            calculated: None,
            fixed: None,
            percent: None,
        }
    }
}
