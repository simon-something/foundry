/*
 * Cloud SQL Admin API
 *
 * API for Cloud SQL database instance management
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::sqladmin_v1::models;
use serde::{Deserialize, Serialize};

/// InstancesCloneRequest : Database instance clone request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesCloneRequest {
    #[serde(rename = "cloneContext", skip_serializing_if = "Option::is_none")]
    pub clone_context: Option<Box<models::CloneContext>>,
}

impl InstancesCloneRequest {
    /// Database instance clone request.
    pub fn new() -> InstancesCloneRequest {
        InstancesCloneRequest {
            clone_context: None,
        }
    }
}
