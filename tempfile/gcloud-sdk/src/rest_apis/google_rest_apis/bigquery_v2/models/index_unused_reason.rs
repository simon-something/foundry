/*
 * BigQuery API
 *
 * A data platform for customers to create, manage, share and query data.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::bigquery_v2::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexUnusedReason {
    #[serde(rename = "base_table", skip_serializing_if = "Option::is_none")]
    pub base_table: Option<Box<models::TableReference>>,
    /// [Output-only] Specifies the high-level reason for the scenario when no search index was used.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// [Output-only] Specifies the name of the unused search index, if available.
    #[serde(rename = "index_name", skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// [Output-only] Free form human-readable reason for the scenario when no search index was used.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl IndexUnusedReason {
    pub fn new() -> IndexUnusedReason {
        IndexUnusedReason {
            base_table: None,
            code: None,
            index_name: None,
            message: None,
        }
    }
}
