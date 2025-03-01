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

/// InterconnectLocation : Represents an Interconnect Attachment (VLAN) Location resource. You can use this resource to find location details about an Interconnect attachment (VLAN). For more information about interconnect attachments, read Creating VLAN Attachments.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterconnectLocation {
    /// [Output Only] The postal address of the Point of Presence, each line in the address is separated by a newline character.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// [Output Only] Availability zone for this InterconnectLocation. Within a metropolitan area (metro), maintenance will not be simultaneously scheduled in more than one availability zone. Example: \"zone1\" or \"zone2\".
    #[serde(rename = "availabilityZone", skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// [Output only] List of features available at this InterconnectLocation, which can take one of the following values: - MACSEC
    #[serde(rename = "availableFeatures", skip_serializing_if = "Option::is_none")]
    pub available_features: Option<Vec<AvailableFeatures>>,
    /// [Output only] List of link types available at this InterconnectLocation, which can take one of the following values: - LINK_TYPE_ETHERNET_10G_LR - LINK_TYPE_ETHERNET_100G_LR
    #[serde(rename = "availableLinkTypes", skip_serializing_if = "Option::is_none")]
    pub available_link_types: Option<Vec<AvailableLinkTypes>>,
    /// [Output Only] Metropolitan area designator that indicates which city an interconnect is located. For example: \"Chicago, IL\", \"Amsterdam, Netherlands\".
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
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
    /// [Output Only] Type of the resource. Always compute#interconnectLocation for interconnect locations.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Name of the resource.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] The peeringdb identifier for this facility (corresponding with a netfac type in peeringdb).
    #[serde(
        rename = "peeringdbFacilityId",
        skip_serializing_if = "Option::is_none"
    )]
    pub peeringdb_facility_id: Option<String>,
    /// [Output Only] A list of InterconnectLocation.RegionInfo objects, that describe parameters pertaining to the relation between this InterconnectLocation and various Google Cloud regions.
    #[serde(rename = "regionInfos", skip_serializing_if = "Option::is_none")]
    pub region_infos: Option<Vec<models::InterconnectLocationRegionInfo>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// [Output Only] The status of this InterconnectLocation, which can take one of the following values: - CLOSED: The InterconnectLocation is closed and is unavailable for provisioning new Interconnects. - AVAILABLE: The InterconnectLocation is available for provisioning new Interconnects.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] Reserved for future use.
    #[serde(rename = "supportsPzs", skip_serializing_if = "Option::is_none")]
    pub supports_pzs: Option<bool>,
}

impl InterconnectLocation {
    /// Represents an Interconnect Attachment (VLAN) Location resource. You can use this resource to find location details about an Interconnect attachment (VLAN). For more information about interconnect attachments, read Creating VLAN Attachments.
    pub fn new() -> InterconnectLocation {
        InterconnectLocation {
            address: None,
            availability_zone: None,
            available_features: None,
            available_link_types: None,
            city: None,
            continent: None,
            creation_timestamp: None,
            description: None,
            facility_provider: None,
            facility_provider_facility_id: None,
            id: None,
            kind: None,
            name: None,
            peeringdb_facility_id: None,
            region_infos: None,
            self_link: None,
            status: None,
            supports_pzs: None,
        }
    }
}
/// [Output only] List of features available at this InterconnectLocation, which can take one of the following values: - MACSEC
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvailableFeatures {
    #[serde(rename = "IF_MACSEC")]
    IfMacsec,
}

impl Default for AvailableFeatures {
    fn default() -> AvailableFeatures {
        Self::IfMacsec
    }
}
/// [Output only] List of link types available at this InterconnectLocation, which can take one of the following values: - LINK_TYPE_ETHERNET_10G_LR - LINK_TYPE_ETHERNET_100G_LR
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AvailableLinkTypes {
    #[serde(rename = "LINK_TYPE_ETHERNET_100G_LR")]
    Variant100Glr,
    #[serde(rename = "LINK_TYPE_ETHERNET_10G_LR")]
    Variant10Glr,
}

impl Default for AvailableLinkTypes {
    fn default() -> AvailableLinkTypes {
        Self::Variant100Glr
    }
}
/// [Output Only] Continent for this location, which can take one of the following values: - AFRICA - ASIA_PAC - EUROPE - NORTH_AMERICA - SOUTH_AMERICA
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Continent {
    #[serde(rename = "AFRICA")]
    Africa,
    #[serde(rename = "ASIA_PAC")]
    AsiaPac,
    #[serde(rename = "C_AFRICA")]
    CAfrica,
    #[serde(rename = "C_ASIA_PAC")]
    CAsiaPac,
    #[serde(rename = "C_EUROPE")]
    CEurope,
    #[serde(rename = "C_NORTH_AMERICA")]
    CNorthAmerica,
    #[serde(rename = "C_SOUTH_AMERICA")]
    CSouthAmerica,
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
/// [Output Only] The status of this InterconnectLocation, which can take one of the following values: - CLOSED: The InterconnectLocation is closed and is unavailable for provisioning new Interconnects. - AVAILABLE: The InterconnectLocation is available for provisioning new Interconnects.
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
