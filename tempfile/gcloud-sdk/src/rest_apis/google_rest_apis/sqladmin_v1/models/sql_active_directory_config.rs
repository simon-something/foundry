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

/// SqlActiveDirectoryConfig : Active Directory configuration, relevant only for Cloud SQL for SQL Server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlActiveDirectoryConfig {
    /// The name of the domain (e.g., mydomain.com).
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// This is always sql#activeDirectoryConfig.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl SqlActiveDirectoryConfig {
    /// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
    pub fn new() -> SqlActiveDirectoryConfig {
        SqlActiveDirectoryConfig {
            domain: None,
            kind: None,
        }
    }
}
