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

/// Database : Represents a SQL database on the Cloud SQL instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Database {
    /// The Cloud SQL charset value.
    #[serde(rename = "charset", skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    /// The Cloud SQL collation value.
    #[serde(rename = "collation", skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    /// This field is deprecated and will be removed from a future version of the API.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    #[serde(rename = "instance", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// This is always `sql#database`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of the database in the Cloud SQL instance. This does not include the project ID or instance name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The project ID of the project containing the Cloud SQL database. The Google apps domain is prefixed if applicable.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The URI of this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(
        rename = "sqlserverDatabaseDetails",
        skip_serializing_if = "Option::is_none"
    )]
    pub sqlserver_database_details: Option<Box<models::SqlServerDatabaseDetails>>,
}

impl Database {
    /// Represents a SQL database on the Cloud SQL instance.
    pub fn new() -> Database {
        Database {
            charset: None,
            collation: None,
            etag: None,
            instance: None,
            kind: None,
            name: None,
            project: None,
            self_link: None,
            sqlserver_database_details: None,
        }
    }
}
