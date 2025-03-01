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
pub struct NodeGroupsSimulateMaintenanceEventRequest {
    /// Names of the nodes to go under maintenance simulation.
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
}

impl NodeGroupsSimulateMaintenanceEventRequest {
    pub fn new() -> NodeGroupsSimulateMaintenanceEventRequest {
        NodeGroupsSimulateMaintenanceEventRequest { nodes: None }
    }
}
