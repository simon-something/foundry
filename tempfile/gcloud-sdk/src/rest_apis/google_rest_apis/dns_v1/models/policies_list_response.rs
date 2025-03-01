/*
 * Cloud DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoliciesListResponse {
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<Box<models::ResponseHeader>>,
    /// Type of resource.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The presence of this field indicates that there exist more results following your last page of results in pagination order. To fetch them, make another list request using this value as your page token. This lets you the complete contents of even very large collections one page at a time. However, if the contents of the collection change between the first and last paginated list request, the set of all elements returned are an inconsistent view of the collection. You cannot retrieve a consistent snapshot of a collection larger than the maximum page size.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The policy resources.
    #[serde(rename = "policies", skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<models::Policy>>,
}

impl PoliciesListResponse {
    pub fn new() -> PoliciesListResponse {
        PoliciesListResponse {
            header: None,
            kind: None,
            next_page_token: None,
            policies: None,
        }
    }
}
