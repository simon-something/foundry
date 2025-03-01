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

/// DiskInstantiationConfig : A specification of the desired way to instantiate a disk in the instance template when its created from a source instance.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskInstantiationConfig {
    /// Specifies whether the disk will be auto-deleted when the instance is deleted (but not when the disk is detached from the instance).
    #[serde(rename = "autoDelete", skip_serializing_if = "Option::is_none")]
    pub auto_delete: Option<bool>,
    /// The custom source image to be used to restore this disk when instantiating this instance template.
    #[serde(rename = "customImage", skip_serializing_if = "Option::is_none")]
    pub custom_image: Option<String>,
    /// Specifies the device name of the disk to which the configurations apply to.
    #[serde(rename = "deviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    /// Specifies whether to include the disk and what image to use. Possible values are: - source-image: to use the same image that was used to create the source instance's corresponding disk. Applicable to the boot disk and additional read-write disks. - source-image-family: to use the same image family that was used to create the source instance's corresponding disk. Applicable to the boot disk and additional read-write disks. - custom-image: to use a user-provided image url for disk creation. Applicable to the boot disk and additional read-write disks. - attach-read-only: to attach a read-only disk. Applicable to read-only disks. - do-not-include: to exclude a disk from the template. Applicable to additional read-write disks, local SSDs, and read-only disks.
    #[serde(rename = "instantiateFrom", skip_serializing_if = "Option::is_none")]
    pub instantiate_from: Option<InstantiateFrom>,
}

impl DiskInstantiationConfig {
    /// A specification of the desired way to instantiate a disk in the instance template when its created from a source instance.
    pub fn new() -> DiskInstantiationConfig {
        DiskInstantiationConfig {
            auto_delete: None,
            custom_image: None,
            device_name: None,
            instantiate_from: None,
        }
    }
}
/// Specifies whether to include the disk and what image to use. Possible values are: - source-image: to use the same image that was used to create the source instance's corresponding disk. Applicable to the boot disk and additional read-write disks. - source-image-family: to use the same image family that was used to create the source instance's corresponding disk. Applicable to the boot disk and additional read-write disks. - custom-image: to use a user-provided image url for disk creation. Applicable to the boot disk and additional read-write disks. - attach-read-only: to attach a read-only disk. Applicable to read-only disks. - do-not-include: to exclude a disk from the template. Applicable to additional read-write disks, local SSDs, and read-only disks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstantiateFrom {
    #[serde(rename = "ATTACH_READ_ONLY")]
    AttachReadOnly,
    #[serde(rename = "BLANK")]
    Blank,
    #[serde(rename = "CUSTOM_IMAGE")]
    CustomImage,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "DO_NOT_INCLUDE")]
    DoNotInclude,
    #[serde(rename = "SOURCE_IMAGE")]
    SourceImage,
    #[serde(rename = "SOURCE_IMAGE_FAMILY")]
    SourceImageFamily,
}

impl Default for InstantiateFrom {
    fn default() -> InstantiateFrom {
        Self::AttachReadOnly
    }
}
