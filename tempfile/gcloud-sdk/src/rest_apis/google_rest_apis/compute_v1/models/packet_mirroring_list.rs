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

/// PacketMirroringList : Contains a list of PacketMirroring resources.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketMirroringList {
    /// [Output Only] Unique identifier for the resource; defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of PacketMirroring resources.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::PacketMirroring>>,
    /// [Output Only] Type of resource. Always compute#packetMirroring for packetMirrorings.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] This token allows you to get the next page of results for list requests. If the number of results is larger than maxResults, use the nextPageToken as a value for the query parameter pageToken in the next list request. Subsequent list requests will have their own nextPageToken to continue paging through the results.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// [Output Only] Server-defined URL for this resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(rename = "warning", skip_serializing_if = "Option::is_none")]
    pub warning: Option<Box<models::AcceleratorTypeAggregatedListWarning>>,
}

impl PacketMirroringList {
    /// Contains a list of PacketMirroring resources.
    pub fn new() -> PacketMirroringList {
        PacketMirroringList {
            id: None,
            items: None,
            kind: None,
            next_page_token: None,
            self_link: None,
            warning: None,
        }
    }
}
