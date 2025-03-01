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

/// InstancesListServerCasResponse : Instances ListServerCas response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstancesListServerCasResponse {
    #[serde(rename = "activeVersion", skip_serializing_if = "Option::is_none")]
    pub active_version: Option<String>,
    /// List of server CA certificates for the instance.
    #[serde(rename = "certs", skip_serializing_if = "Option::is_none")]
    pub certs: Option<Vec<models::SslCert>>,
    /// This is always `sql#instancesListServerCas`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl InstancesListServerCasResponse {
    /// Instances ListServerCas response.
    pub fn new() -> InstancesListServerCasResponse {
        InstancesListServerCasResponse {
            active_version: None,
            certs: None,
            kind: None,
        }
    }
}
