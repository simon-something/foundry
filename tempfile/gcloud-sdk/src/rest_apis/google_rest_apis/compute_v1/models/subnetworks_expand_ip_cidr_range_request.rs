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
pub struct SubnetworksExpandIpCidrRangeRequest {
    /// The IP (in CIDR format or netmask) of internal addresses that are legal on this Subnetwork. This range should be disjoint from other subnetworks within this network. This range can only be larger than (i.e. a superset of) the range previously defined before the update.
    #[serde(rename = "ipCidrRange", skip_serializing_if = "Option::is_none")]
    pub ip_cidr_range: Option<String>,
}

impl SubnetworksExpandIpCidrRangeRequest {
    pub fn new() -> SubnetworksExpandIpCidrRangeRequest {
        SubnetworksExpandIpCidrRangeRequest {
            ip_cidr_range: None,
        }
    }
}
