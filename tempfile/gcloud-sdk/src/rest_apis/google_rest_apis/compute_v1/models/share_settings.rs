/*
 * Compute Engine API
 *
 * Creates and runs virtual machines on Google Cloud Platform.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::compute_v1::models;
use serde::{Deserialize, Serialize};

/// ShareSettings : The share setting for reservations and sole tenancy node groups.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareSettings {
    /// A map of project id and project config. This is only valid when share_type's value is SPECIFIC_PROJECTS.
    #[serde(rename = "projectMap", skip_serializing_if = "Option::is_none")]
    pub project_map: Option<std::collections::HashMap<String, models::ShareSettingsProjectConfig>>,
    /// Type of sharing for this shared-reservation
    #[serde(rename = "shareType", skip_serializing_if = "Option::is_none")]
    pub share_type: Option<ShareType>,
}

impl ShareSettings {
    /// The share setting for reservations and sole tenancy node groups.
    pub fn new() -> ShareSettings {
        ShareSettings {
            project_map: None,
            share_type: None,
        }
    }
}
/// Type of sharing for this shared-reservation
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ShareType {
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "ORGANIZATION")]
    Organization,
    #[serde(rename = "SHARE_TYPE_UNSPECIFIED")]
    ShareTypeUnspecified,
    #[serde(rename = "SPECIFIC_PROJECTS")]
    SpecificProjects,
}

impl Default for ShareType {
    fn default() -> ShareType {
        Self::Local
    }
}
