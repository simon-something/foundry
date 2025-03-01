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

/// RouterStatusNatStatusNatRuleStatus : Status of a NAT Rule contained in this NAT.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouterStatusNatStatusNatRuleStatus {
    /// A list of active IPs for NAT. Example: [\"1.1.1.1\", \"179.12.26.133\"].
    #[serde(rename = "activeNatIps", skip_serializing_if = "Option::is_none")]
    pub active_nat_ips: Option<Vec<String>>,
    /// A list of IPs for NAT that are in drain mode. Example: [\"1.1.1.1\", \"179.12.26.133\"].
    #[serde(rename = "drainNatIps", skip_serializing_if = "Option::is_none")]
    pub drain_nat_ips: Option<Vec<String>>,
    /// The number of extra IPs to allocate. This will be greater than 0 only if the existing IPs in this NAT Rule are NOT enough to allow all configured VMs to use NAT.
    #[serde(rename = "minExtraIpsNeeded", skip_serializing_if = "Option::is_none")]
    pub min_extra_ips_needed: Option<i32>,
    /// Number of VM endpoints (i.e., NICs) that have NAT Mappings from this NAT Rule.
    #[serde(
        rename = "numVmEndpointsWithNatMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_vm_endpoints_with_nat_mappings: Option<i32>,
    /// Rule number of the rule.
    #[serde(rename = "ruleNumber", skip_serializing_if = "Option::is_none")]
    pub rule_number: Option<i32>,
}

impl RouterStatusNatStatusNatRuleStatus {
    /// Status of a NAT Rule contained in this NAT.
    pub fn new() -> RouterStatusNatStatusNatRuleStatus {
        RouterStatusNatStatusNatRuleStatus {
            active_nat_ips: None,
            drain_nat_ips: None,
            min_extra_ips_needed: None,
            num_vm_endpoints_with_nat_mappings: None,
            rule_number: None,
        }
    }
}
