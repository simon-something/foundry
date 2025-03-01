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

/// License : Represents a License resource. A License represents billing and aggregate usage data for public and marketplace images. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    /// [Output Only] Deprecated. This field no longer reflects whether a license charges a usage fee.
    #[serde(rename = "chargesUseFee", skip_serializing_if = "Option::is_none")]
    pub charges_use_fee: Option<bool>,
    /// [Output Only] Creation timestamp in RFC3339 text format.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// An optional textual description of the resource; provided by the client when the resource is created.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The unique identifier for the resource. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] Type of resource. Always compute#license for licenses.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] The unique code used to attach this license to images, snapshots, and disks.
    #[serde(rename = "licenseCode", skip_serializing_if = "Option::is_none")]
    pub license_code: Option<String>,
    /// Name of the resource. The name must be 1-63 characters long and comply with RFC1035.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "resourceRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_requirements: Option<Box<models::LicenseResourceRequirements>>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    /// If false, licenses will not be copied from the source resource when creating an image from a disk, disk from snapshot, or snapshot from disk.
    #[serde(rename = "transferable", skip_serializing_if = "Option::is_none")]
    pub transferable: Option<bool>,
}

impl License {
    /// Represents a License resource. A License represents billing and aggregate usage data for public and marketplace images. *Caution* This resource is intended for use only by third-party partners who are creating Cloud Marketplace images.
    pub fn new() -> License {
        License {
            charges_use_fee: None,
            creation_timestamp: None,
            description: None,
            id: None,
            kind: None,
            license_code: None,
            name: None,
            resource_requirements: None,
            self_link: None,
            transferable: None,
        }
    }
}
