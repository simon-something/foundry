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
pub struct InterconnectAttachmentConfigurationConstraintsBgpPeerAsnRange {
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
}

impl InterconnectAttachmentConfigurationConstraintsBgpPeerAsnRange {
    pub fn new() -> InterconnectAttachmentConfigurationConstraintsBgpPeerAsnRange {
        InterconnectAttachmentConfigurationConstraintsBgpPeerAsnRange {
            max: None,
            min: None,
        }
    }
}
