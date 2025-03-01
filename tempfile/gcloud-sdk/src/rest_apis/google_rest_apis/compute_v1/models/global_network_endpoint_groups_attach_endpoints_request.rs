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
pub struct GlobalNetworkEndpointGroupsAttachEndpointsRequest {
    /// The list of network endpoints to be attached.
    #[serde(rename = "networkEndpoints", skip_serializing_if = "Option::is_none")]
    pub network_endpoints: Option<Vec<models::NetworkEndpoint>>,
}

impl GlobalNetworkEndpointGroupsAttachEndpointsRequest {
    pub fn new() -> GlobalNetworkEndpointGroupsAttachEndpointsRequest {
        GlobalNetworkEndpointGroupsAttachEndpointsRequest {
            network_endpoints: None,
        }
    }
}
