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
pub struct Streamingbuffer {
    /// [Output-only] A lower-bound estimate of the number of bytes currently in the streaming buffer.
    #[serde(rename = "estimatedBytes", skip_serializing_if = "Option::is_none")]
    pub estimated_bytes: Option<String>,
    /// [Output-only] A lower-bound estimate of the number of rows currently in the streaming buffer.
    #[serde(rename = "estimatedRows", skip_serializing_if = "Option::is_none")]
    pub estimated_rows: Option<String>,
    /// [Output-only] Contains the timestamp of the oldest entry in the streaming buffer, in milliseconds since the epoch, if the streaming buffer is available.
    #[serde(rename = "oldestEntryTime", skip_serializing_if = "Option::is_none")]
    pub oldest_entry_time: Option<String>,
}

impl Streamingbuffer {
    pub fn new() -> Streamingbuffer {
        Streamingbuffer {
            estimated_bytes: None,
            estimated_rows: None,
            oldest_entry_time: None,
        }
    }
}
