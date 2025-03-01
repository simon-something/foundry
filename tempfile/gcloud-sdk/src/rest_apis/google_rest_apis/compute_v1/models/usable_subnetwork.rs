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

/// UsableSubnetwork : Subnetwork which the current user has compute.subnetworks.use permission on.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsableSubnetwork {
    /// [Output Only] The external IPv6 address range that is assigned to this subnetwork.
    #[serde(rename = "externalIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub external_ipv6_prefix: Option<String>,
    /// [Output Only] The internal IPv6 address range that is assigned to this subnetwork.
    #[serde(rename = "internalIpv6Prefix", skip_serializing_if = "Option::is_none")]
    pub internal_ipv6_prefix: Option<String>,
    /// The range of internal addresses that are owned by this subnetwork.
    #[serde(rename = "ipCidrRange", skip_serializing_if = "Option::is_none")]
    pub ip_cidr_range: Option<String>,
    /// The access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation or the first time the subnet is updated into IPV4_IPV6 dual stack.
    #[serde(rename = "ipv6AccessType", skip_serializing_if = "Option::is_none")]
    pub ipv6_access_type: Option<Ipv6AccessType>,
    /// Network URL.
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    /// The purpose of the resource. This field can be either PRIVATE, GLOBAL_MANAGED_PROXY, REGIONAL_MANAGED_PROXY, PRIVATE_SERVICE_CONNECT, or PRIVATE is the default purpose for user-created subnets or subnets that are automatically created in auto mode networks. Subnets with purpose set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY are user-created subnetworks that are reserved for Envoy-based load balancers. A subnet with purpose set to PRIVATE_SERVICE_CONNECT is used to publish services using Private Service Connect. If unspecified, the subnet purpose defaults to PRIVATE. The enableFlowLogs field isn't supported if the subnet purpose field is set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY.
    #[serde(rename = "purpose", skip_serializing_if = "Option::is_none")]
    pub purpose: Option<Purpose>,
    /// The role of subnetwork. Currently, this field is only used when purpose is set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY. The value can be set to ACTIVE or BACKUP. An ACTIVE subnetwork is one that is currently being used for Envoy-based load balancers in a region. A BACKUP subnetwork is one that is ready to be promoted to ACTIVE or is currently draining. This field can be updated with a patch request.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Secondary IP ranges.
    #[serde(rename = "secondaryIpRanges", skip_serializing_if = "Option::is_none")]
    pub secondary_ip_ranges: Option<Vec<models::UsableSubnetworkSecondaryRange>>,
    /// The stack type for the subnet. If set to IPV4_ONLY, new VMs in the subnet are assigned IPv4 addresses only. If set to IPV4_IPV6, new VMs in the subnet can be assigned both IPv4 and IPv6 addresses. If not specified, IPV4_ONLY is used. This field can be both set at resource creation time and updated using patch.
    #[serde(rename = "stackType", skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<StackType>,
    /// Subnetwork URL.
    #[serde(rename = "subnetwork", skip_serializing_if = "Option::is_none")]
    pub subnetwork: Option<String>,
}

impl UsableSubnetwork {
    /// Subnetwork which the current user has compute.subnetworks.use permission on.
    pub fn new() -> UsableSubnetwork {
        UsableSubnetwork {
            external_ipv6_prefix: None,
            internal_ipv6_prefix: None,
            ip_cidr_range: None,
            ipv6_access_type: None,
            network: None,
            purpose: None,
            role: None,
            secondary_ip_ranges: None,
            stack_type: None,
            subnetwork: None,
        }
    }
}
/// The access type of IPv6 address this subnet holds. It's immutable and can only be specified during creation or the first time the subnet is updated into IPV4_IPV6 dual stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Ipv6AccessType {
    #[serde(rename = "EXTERNAL")]
    External,
    #[serde(rename = "INTERNAL")]
    Internal,
}

impl Default for Ipv6AccessType {
    fn default() -> Ipv6AccessType {
        Self::External
    }
}
/// The purpose of the resource. This field can be either PRIVATE, GLOBAL_MANAGED_PROXY, REGIONAL_MANAGED_PROXY, PRIVATE_SERVICE_CONNECT, or PRIVATE is the default purpose for user-created subnets or subnets that are automatically created in auto mode networks. Subnets with purpose set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY are user-created subnetworks that are reserved for Envoy-based load balancers. A subnet with purpose set to PRIVATE_SERVICE_CONNECT is used to publish services using Private Service Connect. If unspecified, the subnet purpose defaults to PRIVATE. The enableFlowLogs field isn't supported if the subnet purpose field is set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Purpose {
    #[serde(rename = "GLOBAL_MANAGED_PROXY")]
    GlobalManagedProxy,
    #[serde(rename = "INTERNAL_HTTPS_LOAD_BALANCER")]
    InternalHttpsLoadBalancer,
    #[serde(rename = "PRIVATE")]
    Private,
    #[serde(rename = "PRIVATE_NAT")]
    PrivateNat,
    #[serde(rename = "PRIVATE_RFC_1918")]
    PrivateRfc1918,
    #[serde(rename = "PRIVATE_SERVICE_CONNECT")]
    PrivateServiceConnect,
    #[serde(rename = "REGIONAL_MANAGED_PROXY")]
    RegionalManagedProxy,
}

impl Default for Purpose {
    fn default() -> Purpose {
        Self::GlobalManagedProxy
    }
}
/// The role of subnetwork. Currently, this field is only used when purpose is set to GLOBAL_MANAGED_PROXY or REGIONAL_MANAGED_PROXY. The value can be set to ACTIVE or BACKUP. An ACTIVE subnetwork is one that is currently being used for Envoy-based load balancers in a region. A BACKUP subnetwork is one that is ready to be promoted to ACTIVE or is currently draining. This field can be updated with a patch request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "BACKUP")]
    Backup,
}

impl Default for Role {
    fn default() -> Role {
        Self::Active
    }
}
/// The stack type for the subnet. If set to IPV4_ONLY, new VMs in the subnet are assigned IPv4 addresses only. If set to IPV4_IPV6, new VMs in the subnet can be assigned both IPv4 and IPv6 addresses. If not specified, IPV4_ONLY is used. This field can be both set at resource creation time and updated using patch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StackType {
    #[serde(rename = "IPV4_IPV6")]
    Ipv6,
    #[serde(rename = "IPV4_ONLY")]
    Only,
}

impl Default for StackType {
    fn default() -> StackType {
        Self::Ipv6
    }
}
