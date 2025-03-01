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

/// AliasIpRange : An alias IP range attached to an instance's network interface.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasIpRange {
    /// The IP alias ranges to allocate for this interface. This IP CIDR range must belong to the specified subnetwork and cannot contain IP addresses reserved by system or used by other network interfaces. This range may be a single IP address (such as 10.2.3.4), a netmask (such as /24) or a CIDR-formatted string (such as 10.1.2.0/24).
    #[serde(rename = "ipCidrRange", skip_serializing_if = "Option::is_none")]
    pub ip_cidr_range: Option<String>,
    /// The name of a subnetwork secondary IP range from which to allocate an IP alias range. If not specified, the primary range of the subnetwork is used.
    #[serde(
        rename = "subnetworkRangeName",
        skip_serializing_if = "Option::is_none"
    )]
    pub subnetwork_range_name: Option<String>,
}

impl AliasIpRange {
    /// An alias IP range attached to an instance's network interface.
    pub fn new() -> AliasIpRange {
        AliasIpRange {
            ip_cidr_range: None,
            subnetwork_range_name: None,
        }
    }
}
