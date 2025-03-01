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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetPoolAggregatedList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of TargetPool resources.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<std::collections::HashMap<String, models::TargetPoolsScopedList>>,
    /// [Output Only] Type of resource. Always compute#targetPoolAggregatedList for aggregated lists of target pools.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// [Output Only] Server-defined URL for this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] Unreachable resources.
    #[serde(rename = "unreachables", skip_serializing_if = "Option::is_none")]
    pub unreachables: Option<Vec<String>>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::AcceleratorTypeAggregatedListWarning>>,
}

impl TargetPoolAggregatedList {
    pub fn new() -> TargetPoolAggregatedList {
        TargetPoolAggregatedList {
            id: None,
            items: None,
            kind: None,
            next_page_token: None,
            self_link: None,
            unreachables: None,
            warning: None,
        }
    }
}
