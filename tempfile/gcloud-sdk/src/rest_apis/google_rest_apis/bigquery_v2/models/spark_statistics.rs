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
pub struct SparkStatistics {
    /// [Output-only] Endpoints generated for the Spark job.
    #[serde(rename = "endpoints", skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "logging_info", skip_serializing_if = "Option::is_none")]
    pub logging_info: Option<Box<models::SparkLoggingInfo>>,
    /// [Output-only] Spark job id if a Spark job is created successfully.
    #[serde(rename = "spark_job_id", skip_serializing_if = "Option::is_none")]
    pub spark_job_id: Option<String>,
    /// [Output-only] Location where the Spark job is executed.
    #[serde(rename = "spark_job_location", skip_serializing_if = "Option::is_none")]
    pub spark_job_location: Option<String>,
}

impl SparkStatistics {
    pub fn new() -> SparkStatistics {
        SparkStatistics {
            endpoints: None,
            logging_info: None,
            spark_job_id: None,
            spark_job_location: None,
        }
    }
}
