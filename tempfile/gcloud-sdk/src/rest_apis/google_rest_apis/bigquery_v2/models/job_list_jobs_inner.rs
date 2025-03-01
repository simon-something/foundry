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
pub struct JobListJobsInner {
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<models::JobConfiguration>>,
    #[serde(rename = "errorResult", skip_serializing_if = "Option::is_none")]
    pub error_result: Option<Box<models::ErrorProto>>,
    /// Unique opaque ID of the job.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "jobReference", skip_serializing_if = "Option::is_none")]
    pub job_reference: Option<Box<models::JobReference>>,
    /// The resource type.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Running state of the job. When the state is DONE, errorResult can be checked to determine whether the job succeeded or failed.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "statistics", skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Box<models::JobStatistics>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::JobStatus>>,
    /// [Full-projection-only] Email address of the user who ran the job.
    #[serde(rename = "user_email", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
}

impl JobListJobsInner {
    pub fn new() -> JobListJobsInner {
        JobListJobsInner {
            configuration: None,
            error_result: None,
            id: None,
            job_reference: None,
            kind: None,
            state: None,
            statistics: None,
            status: None,
            user_email: None,
        }
    }
}
