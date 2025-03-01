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

/// RouterStatusNatStatus : Status of a NAT contained in this router.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterStatusNatStatus {
    /// A list of IPs auto-allocated for NAT. Example: [\"1.1.1.1\", \"129.2.16.89\"]
    #[serde(
        rename = "autoAllocatedNatIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_allocated_nat_ips: Option<Vec<String>>,
    /// A list of IPs auto-allocated for NAT that are in drain mode. Example: [\"1.1.1.1\", \"179.12.26.133\"].
    #[serde(
        rename = "drainAutoAllocatedNatIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_auto_allocated_nat_ips: Option<Vec<String>>,
    /// A list of IPs user-allocated for NAT that are in drain mode. Example: [\"1.1.1.1\", \"179.12.26.133\"].
    #[serde(
        rename = "drainUserAllocatedNatIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub drain_user_allocated_nat_ips: Option<Vec<String>>,
    /// The number of extra IPs to allocate. This will be greater than 0 only if user-specified IPs are NOT enough to allow all configured VMs to use NAT. This value is meaningful only when auto-allocation of NAT IPs is *not* used.
    #[serde(
        rename = "minExtraNatIpsNeeded",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_extra_nat_ips_needed: Option<i32>,
    /// Unique name of this NAT.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Number of VM endpoints (i.e., Nics) that can use NAT.
    #[serde(
        rename = "numVmEndpointsWithNatMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_vm_endpoints_with_nat_mappings: Option<i32>,
    /// Status of rules in this NAT.
    #[serde(rename = "ruleStatus", skip_serializing_if = "Option::is_none")]
    pub rule_status: Option<Vec<models::RouterStatusNatStatusNatRuleStatus>>,
    /// A list of fully qualified URLs of reserved IP address resources.
    #[serde(
        rename = "userAllocatedNatIpResources",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_allocated_nat_ip_resources: Option<Vec<String>>,
    /// A list of IPs user-allocated for NAT. They will be raw IP strings like \"179.12.26.133\".
    #[serde(
        rename = "userAllocatedNatIps",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_allocated_nat_ips: Option<Vec<String>>,
}

impl RouterStatusNatStatus {
    /// Status of a NAT contained in this router.
    pub fn new() -> RouterStatusNatStatus {
        RouterStatusNatStatus {
            auto_allocated_nat_ips: None,
            drain_auto_allocated_nat_ips: None,
            drain_user_allocated_nat_ips: None,
            min_extra_nat_ips_needed: None,
            name: None,
            num_vm_endpoints_with_nat_mappings: None,
            rule_status: None,
            user_allocated_nat_ip_resources: None,
            user_allocated_nat_ips: None,
        }
    }
}
