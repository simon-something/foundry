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

/// InstancesImportRequest : Database instance import request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesImportRequest {
    #[serde(rename = "importContext", skip_serializing_if = "Option::is_none")]
    pub import_context: Option<Box<models::ImportContext>>,
}

impl InstancesImportRequest {
    /// Database instance import request.
    pub fn new() -> InstancesImportRequest {
        InstancesImportRequest {
            import_context: None,
        }
    }
}
