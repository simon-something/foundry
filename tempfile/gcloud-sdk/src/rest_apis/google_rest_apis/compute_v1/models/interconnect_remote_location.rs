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

/// InterconnectRemoteLocation : Represents a Cross-Cloud Interconnect Remote Location resource. You can use this resource to find remote location details about an Interconnect attachment (VLAN).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterconnectRemoteLocation {
    /// [Output Only] The postal address of the Point of Presence, each line in the address is separated by a newline character.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(
        rename = "attachmentConfigurationConstraints",
        skip_serializing_if = "Option::is_none"
    )]
    pub attachment_configuration_constraints:
        Option<Box<models::InterconnectAttachmentConfigurationConstraints>>,
    /// [Output Only] Metropolitan area designator that indicates which city an interconnect is located. For example: \"Chicago, IL\", \"Amsterdam, Netherlands\".
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Box<models::InterconnectRemoteLocationConstraints>>,
    /// [Output Only] Continent for this location, which can take one of the following values: - AFRICA - ASIA_PAC - EUROPE - NORTH_AMERICA - SOUTH_AMERICA
    #[serde(rename = "continent", skip_serializing_if = "Option::is_none")]
    pub continent: Option<Continent>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// [Output Only] An optional description of the resource.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The name of the provider for this facility (e.g., EQUINIX).
    #[serde(rename = "facilityProvider", skip_serializing_if = "Option::is_none")]
    pub facility_provider: Option<String>,
    /// [Output Only] A provider-assigned Identifier for this facility (e.g., Ashburn-DC1).
    #[serde(
        rename = "facilityProviderFacilityId",
        skip_serializing_if = "Option::is_none"
    )]
    pub facility_provider_facility_id: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of the resource. Always compute#interconnectRemoteLocation for interconnect remote locations.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Link Aggregation Control Protocol (LACP) constraints, which can take one of the following values: LACP_SUPPORTED, LACP_UNSUPPORTED
    #[serde(rename = "lacp", skip_serializing_if = "Option::is_none")]
    pub lacp: Option<Lacp>,
    /// [Output Only] The maximum number of 100 Gbps ports supported in a link aggregation group (LAG). When linkType is 100 Gbps, requestedLinkCount cannot exceed max_lag_size_100_gbps.
    #[serde(rename = "maxLagSize100Gbps", skip_serializing_if = "Option::is_none")]
    pub max_lag_size100_gbps: Option<i32>,
    /// [Output Only] The maximum number of 10 Gbps ports supported in a link aggregation group (LAG). When linkType is 10 Gbps, requestedLinkCount cannot exceed max_lag_size_10_gbps.
    #[serde(rename = "maxLagSize10Gbps", skip_serializing_if = "Option::is_none")]
    pub max_lag_size10_gbps: Option<i32>,
    /// [Output Only] Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] The peeringdb identifier for this facility (corresponding with a netfac type in peeringdb).
    #[serde(
        rename = "peeringdbFacilityId",
        skip_serializing_if = "Option::is_none"
    )]
    pub peeringdb_facility_id: Option<String>,
    /// [Output Only] Permitted connections.
    #[serde(
        rename = "permittedConnections",
        skip_serializing_if = "Option::is_none"
    )]
    pub permitted_connections: Option<Vec<models::InterconnectRemoteLocationPermittedConnections>>,
    /// [Output Only] Indicates the service provider present at the remote location. Example values: \"Amazon Web Services\", \"Microsoft Azure\".
    #[serde(rename = "remoteService", skip_serializing_if = "Option::is_none")]
    pub remote_service: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The status of this InterconnectRemoteLocation, which can take one of the following values: - CLOSED: The InterconnectRemoteLocation is closed and is unavailable for provisioning new Cross-Cloud Interconnects. - AVAILABLE: The InterconnectRemoteLocation is available for provisioning new Cross-Cloud Interconnects.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl InterconnectRemoteLocation {
    /// Represents a Cross-Cloud Interconnect Remote Location resource. You can use this resource to find remote location details about an Interconnect attachment (VLAN).
    pub fn new() -> InterconnectRemoteLocation {
        InterconnectRemoteLocation {
            address: None,
            attachment_configuration_constraints: None,
            city: None,
            constraints: None,
            continent: None,
            creation_timestamp: None,
            description: None,
            facility_provider: None,
            facility_provider_facility_id: None,
            id: None,
            kind: None,
            lacp: None,
            max_lag_size100_gbps: None,
            max_lag_size10_gbps: None,
            name: None,
            peeringdb_facility_id: None,
            permitted_connections: None,
            remote_service: None,
            self_link: None,
            status: None,
        }
    }
}
/// [Output Only] Continent for this location, which can take one of the following values: - AFRICA - ASIA_PAC - EUROPE - NORTH_AMERICA - SOUTH_AMERICA
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Continent {
    #[serde(rename = "AFRICA")]
    Africa,
    #[serde(rename = "ASIA_PAC")]
    AsiaPac,
    #[serde(rename = "EUROPE")]
    Europe,
    #[serde(rename = "NORTH_AMERICA")]
    NorthAmerica,
    #[serde(rename = "SOUTH_AMERICA")]
    SouthAmerica,
}

impl Default for Continent {
    fn default() -> Continent {
        Self::Africa
    }
}
/// [Output Only] Link Aggregation Control Protocol (LACP) constraints, which can take one of the following values: LACP_SUPPORTED, LACP_UNSUPPORTED
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Lacp {
    #[serde(rename = "LACP_SUPPORTED")]
    Supported,
    #[serde(rename = "LACP_UNSUPPORTED")]
    Unsupported,
}

impl Default for Lacp {
    fn default() -> Lacp {
        Self::Supported
    }
}
/// [Output Only] The status of this InterconnectRemoteLocation, which can take one of the following values: - CLOSED: The InterconnectRemoteLocation is closed and is unavailable for provisioning new Cross-Cloud Interconnects. - AVAILABLE: The InterconnectRemoteLocation is available for provisioning new Cross-Cloud Interconnects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "CLOSED")]
    Closed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Available
    }
}
