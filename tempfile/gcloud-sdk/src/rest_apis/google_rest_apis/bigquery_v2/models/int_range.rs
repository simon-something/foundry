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

/// IntRange : Range of an int hyperparameter.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntRange {
    /// Max value of the int parameter.
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    /// Min value of the int parameter.
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<String>,
}

impl IntRange {
    /// Range of an int hyperparameter.
    pub fn new() -> IntRange {
        IntRange {
            max: None,
            min: None,
        }
    }
}
