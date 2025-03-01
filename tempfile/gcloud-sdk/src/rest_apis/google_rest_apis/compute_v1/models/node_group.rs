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

use serde_with::serde_as;

/// NodeGroup : Represents a sole-tenant Node Group resource. A sole-tenant node is a physical server that is dedicated to hosting VM instances only for your specific project. Use sole-tenant nodes to keep your instances physically separated from instances in other projects, or to group your instances together on the same host hardware. For more information, read Sole-tenant nodes.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeGroup {
    #[serde(rename = "autoscalingPolicy", skip_serializing_if = "Option::is_none")]
    pub autoscaling_policy: Option<Box<models::NodeGroupAutoscalingPolicy>>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional description of this resource. Provide this property when you create the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "fingerprint", skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<Vec<u8>>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] The type of the resource. Always compute#nodeGroup for node group.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// An opaque location hint used to place the Node close to other resources. This field is for use by internal tools that use the public API. The location hint here on the NodeGroup overrides any location_hint present in the NodeTemplate.
    #[serde(rename = "locationHint", skip_serializing_if = "Option::is_none")]
    pub location_hint: Option<String>,
    /// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT. For more information, see Maintenance policies.
    #[serde(rename = "maintenancePolicy", skip_serializing_if = "Option::is_none")]
    pub maintenance_policy: Option<MaintenancePolicy>,
    #[serde(rename = "maintenanceWindow", skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<Box<models::NodeGroupMaintenanceWindow>>,
    /// The name of the resource, provided by the client when initially creating the resource. The resource name must be 1-63 characters long, and comply with RFC1035. Specifically, the name must be 1-63 characters long and match the regular expression `[a-z]([-a-z0-9]*[a-z0-9])?` which means the first character must be a lowercase letter, and all following characters must be a dash, lowercase letter, or digit, except the last character, which cannot be a dash.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL of the node template to create the node group from.
    #[serde(rename = "nodeTemplate", skip_serializing_if = "Option::is_none")]
    pub node_template: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(rename = "shareSettings", skip_serializing_if = "Option::is_none")]
    pub share_settings: Option<Box<models::ShareSettings>>,
    /// [Output Only] The total number of nodes in the node group.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] The name of the zone where the node group resides, such as us-central1-a.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl NodeGroup {
    /// Represents a sole-tenant Node Group resource. A sole-tenant node is a physical server that is dedicated to hosting VM instances only for your specific project. Use sole-tenant nodes to keep your instances physically separated from instances in other projects, or to group your instances together on the same host hardware. For more information, read Sole-tenant nodes.
    pub fn new() -> NodeGroup {
        NodeGroup {
            autoscaling_policy: None,
            creation_timestamp: None,
            description: None,
            fingerprint: None,
            id: None,
            kind: None,
            location_hint: None,
            maintenance_policy: None,
            maintenance_window: None,
            name: None,
            node_template: None,
            self_link: None,
            share_settings: None,
            size: None,
            status: None,
            zone: None,
        }
    }
}
/// Specifies how to handle instances when a node in the group undergoes maintenance. Set to one of: DEFAULT, RESTART_IN_PLACE, or MIGRATE_WITHIN_NODE_GROUP. The default value is DEFAULT. For more information, see Maintenance policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaintenancePolicy {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "MAINTENANCE_POLICY_UNSPECIFIED")]
    MaintenancePolicyUnspecified,
    #[serde(rename = "MIGRATE_WITHIN_NODE_GROUP")]
    MigrateWithinNodeGroup,
    #[serde(rename = "RESTART_IN_PLACE")]
    RestartInPlace,
}

impl Default for MaintenancePolicy {
    fn default() -> MaintenancePolicy {
        Self::Default
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "CREATING")]
    Creating,
    #[serde(rename = "DELETING")]
    Deleting,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "READY")]
    Ready,
}

impl Default for Status {
    fn default() -> Status {
        Self::Creating
    }
}
