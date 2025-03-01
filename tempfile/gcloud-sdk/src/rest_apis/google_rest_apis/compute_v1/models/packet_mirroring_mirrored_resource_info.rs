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
pub struct PacketMirroringMirroredResourceInfo {
    /// A set of virtual machine instances that are being mirrored. They must live in zones contained in the same region as this packetMirroring. Note that this config will apply only to those network interfaces of the Instances that belong to the network specified in this packetMirroring. You may specify a maximum of 50 Instances.
    #[serde(rename = "instances", skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<models::PacketMirroringMirroredResourceInfoInstanceInfo>>,
    /// A set of subnetworks for which traffic from/to all VM instances will be mirrored. They must live in the same region as this packetMirroring. You may specify a maximum of 5 subnetworks.
    #[serde(rename = "subnetworks", skip_serializing_if = "Option::is_none")]
    pub subnetworks: Option<Vec<models::PacketMirroringMirroredResourceInfoSubnetInfo>>,
    /// A set of mirrored tags. Traffic from/to all VM instances that have one or more of these tags will be mirrored.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl PacketMirroringMirroredResourceInfo {
    pub fn new() -> PacketMirroringMirroredResourceInfo {
        PacketMirroringMirroredResourceInfo {
            instances: None,
            subnetworks: None,
            tags: None,
        }
    }
}
