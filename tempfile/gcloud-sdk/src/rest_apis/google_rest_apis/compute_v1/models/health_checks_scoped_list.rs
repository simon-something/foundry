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
pub struct HealthChecksScopedList {
    /// A list of HealthChecks contained in this scope.
    #[serde(rename = "healthChecks", skip_serializing_if = "Option::is_none")]
    pub health_checks: Option<Vec<models::HealthCheck>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::BackendServicesScopedListWarning>>,
}

impl HealthChecksScopedList {
    pub fn new() -> HealthChecksScopedList {
        HealthChecksScopedList {
            health_checks: None,
            warning: None,
        }
    }
}
