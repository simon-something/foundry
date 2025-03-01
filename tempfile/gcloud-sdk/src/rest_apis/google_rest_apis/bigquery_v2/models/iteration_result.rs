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
pub struct IterationResult {
    /// Time taken to run the iteration in milliseconds.
    #[serde(rename = "durationMs", skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<String>,
    /// Loss computed on the eval data at the end of iteration.
    #[serde(rename = "evalLoss", skip_serializing_if = "Option::is_none")]
    pub eval_loss: Option<f64>,
    /// Index of the iteration, 0 based.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// Learn rate used for this iteration.
    #[serde(rename = "learnRate", skip_serializing_if = "Option::is_none")]
    pub learn_rate: Option<f64>,
    /// Loss computed on the training data at the end of iteration.
    #[serde(rename = "trainingLoss", skip_serializing_if = "Option::is_none")]
    pub training_loss: Option<f64>,
}

impl IterationResult {
    pub fn new() -> IterationResult {
        IterationResult {
            duration_ms: None,
            eval_loss: None,
            index: None,
            learn_rate: None,
            training_loss: None,
        }
    }
}
