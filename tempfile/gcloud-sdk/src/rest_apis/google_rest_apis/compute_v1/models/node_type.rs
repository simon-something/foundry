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

/// NodeType : Represent a sole-tenant Node Type resource. Each node within a node group must have a node type. A node type specifies the total amount of cores and memory for that node. Currently, the only available node type is n1-node-96-624 node type that has 96 vCPUs and 624 GB of memory, available in multiple zones. For more information read Node types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeType {
    /// [Output Only] The CPU platform used by this node type.
    #[serde(rename = "cpuPlatform", skip_serializing_if = "Option::is_none")]
    pub cpu_platform: Option<String>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Box<models::DeprecationStatus>>,
    /// [Output Only] An optional textual description of the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The number of virtual CPUs that are available to the node type.
    #[serde(rename = "guestCpus", skip_serializing_if = "Option::is_none")]
    pub guest_cpus: Option<i32>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] The type of the resource. Always compute#nodeType for node types.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Local SSD available to the node type, defined in GB.
    #[serde(rename = "localSsdGb", skip_serializing_if = "Option::is_none")]
    pub local_ssd_gb: Option<i32>,
    /// [Output Only] The amount of physical memory available to the node type, defined in MB.
    #[serde(rename = "memoryMb", skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    /// [Output Only] Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The name of the zone where the node type resides, such as us-central1-a.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl NodeType {
    /// Represent a sole-tenant Node Type resource. Each node within a node group must have a node type. A node type specifies the total amount of cores and memory for that node. Currently, the only available node type is n1-node-96-624 node type that has 96 vCPUs and 624 GB of memory, available in multiple zones. For more information read Node types.
    pub fn new() -> NodeType {
        NodeType {
            cpu_platform: None,
            creation_timestamp: None,
            deprecated: None,
            description: None,
            guest_cpus: None,
            id: None,
            kind: None,
            local_ssd_gb: None,
            memory_mb: None,
            name: None,
            self_link: None,
            zone: None,
        }
    }
}
