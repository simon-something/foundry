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
pub struct ModelReference {
    /// [Required] The ID of the dataset containing this model.
    #[serde(rename = "datasetId", skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    /// [Required] The ID of the model. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters.
    #[serde(rename = "modelId", skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    /// [Required] The ID of the project containing this model.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl ModelReference {
    pub fn new() -> ModelReference {
        ModelReference {
            dataset_id: None,
            model_id: None,
            project_id: None,
        }
    }
}
