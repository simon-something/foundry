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

/// SqlServerUserDetails : Represents a Sql Server user on the Cloud SQL instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlServerUserDetails {
    /// If the user has been disabled
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// The server roles for this user
    #[serde(rename = "serverRoles", skip_serializing_if = "Option::is_none")]
    pub server_roles: Option<Vec<String>>,
}

impl SqlServerUserDetails {
    /// Represents a Sql Server user on the Cloud SQL instance.
    pub fn new() -> SqlServerUserDetails {
        SqlServerUserDetails {
            disabled: None,
            server_roles: None,
        }
    }
}
