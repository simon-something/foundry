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

/// RrSetRoutingPolicyWrrPolicy : Configures a RRSetRoutingPolicy that routes in a weighted round robin fashion.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RrSetRoutingPolicyWrrPolicy {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::RrSetRoutingPolicyWrrPolicyWrrPolicyItem>>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl RrSetRoutingPolicyWrrPolicy {
    /// Configures a RRSetRoutingPolicy that routes in a weighted round robin fashion.
    pub fn new() -> RrSetRoutingPolicyWrrPolicy {
        RrSetRoutingPolicyWrrPolicy {
            items: None,
            kind: None,
        }
    }
}
