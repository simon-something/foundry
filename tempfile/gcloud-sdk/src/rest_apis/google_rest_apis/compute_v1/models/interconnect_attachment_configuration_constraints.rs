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
pub struct InterconnectAttachmentConfigurationConstraints {
    /// [Output Only] Whether the attachment's BGP session requires/allows/disallows BGP MD5 authentication. This can take one of the following values: MD5_OPTIONAL, MD5_REQUIRED, MD5_UNSUPPORTED. For example, a Cross-Cloud Interconnect connection to a remote cloud provider that requires BGP MD5 authentication has the interconnectRemoteLocation attachment_configuration_constraints.bgp_md5 field set to MD5_REQUIRED, and that property is propagated to the attachment. Similarly, if BGP MD5 is MD5_UNSUPPORTED, an error is returned if MD5 is requested.
    #[serde(rename = "bgpMd5", skip_serializing_if = "Option::is_none")]
    pub bgp_md5: Option<BgpMd5>,
    /// [Output Only] List of ASN ranges that the remote location is known to support. Formatted as an array of inclusive ranges {min: min-value, max: max-value}. For example, [{min: 123, max: 123}, {min: 64512, max: 65534}] allows the peer ASN to be 123 or anything in the range 64512-65534. This field is only advisory. Although the API accepts other ranges, these are the ranges that we recommend.
    #[serde(rename = "bgpPeerAsnRanges", skip_serializing_if = "Option::is_none")]
    pub bgp_peer_asn_ranges:
        Option<Vec<models::InterconnectAttachmentConfigurationConstraintsBgpPeerAsnRange>>,
}

impl InterconnectAttachmentConfigurationConstraints {
    pub fn new() -> InterconnectAttachmentConfigurationConstraints {
        InterconnectAttachmentConfigurationConstraints {
            bgp_md5: None,
            bgp_peer_asn_ranges: None,
        }
    }
}
/// [Output Only] Whether the attachment's BGP session requires/allows/disallows BGP MD5 authentication. This can take one of the following values: MD5_OPTIONAL, MD5_REQUIRED, MD5_UNSUPPORTED. For example, a Cross-Cloud Interconnect connection to a remote cloud provider that requires BGP MD5 authentication has the interconnectRemoteLocation attachment_configuration_constraints.bgp_md5 field set to MD5_REQUIRED, and that property is propagated to the attachment. Similarly, if BGP MD5 is MD5_UNSUPPORTED, an error is returned if MD5 is requested.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BgpMd5 {
    #[serde(rename = "MD5_OPTIONAL")]
    Optional,
    #[serde(rename = "MD5_REQUIRED")]
    Required,
    #[serde(rename = "MD5_UNSUPPORTED")]
    Unsupported,
}

impl Default for BgpMd5 {
    fn default() -> BgpMd5 {
        Self::Optional
    }
}
