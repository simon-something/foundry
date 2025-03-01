/*
 * Cloud DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponsePolicyNetwork {
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The fully qualified URL of the VPC network to bind to. This should be formatted like https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}
    #[serde(rename = "networkUrl", skip_serializing_if = "Option::is_none")]
    pub network_url: Option<String>,
}

impl ResponsePolicyNetwork {
    pub fn new() -> ResponsePolicyNetwork {
        ResponsePolicyNetwork {
            kind: None,
            network_url: None,
        }
    }
}
