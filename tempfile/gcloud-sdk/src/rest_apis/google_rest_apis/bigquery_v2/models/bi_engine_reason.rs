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
pub struct BiEngineReason {
    /// [Output-only] High-level BI Engine reason for partial or disabled acceleration.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// [Output-only] Free form human-readable reason for partial or disabled acceleration.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl BiEngineReason {
    pub fn new() -> BiEngineReason {
        BiEngineReason {
            code: None,
            message: None,
        }
    }
}
