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
pub struct TargetTcpProxiesSetProxyHeaderRequest {
    /// The new type of proxy header to append before sending data to the backend. NONE or PROXY_V1 are allowed.
    #[serde(rename = "proxyHeader", skip_serializing_if = "Option::is_none")]
    pub proxy_header: Option<ProxyHeader>,
}

impl TargetTcpProxiesSetProxyHeaderRequest {
    pub fn new() -> TargetTcpProxiesSetProxyHeaderRequest {
        TargetTcpProxiesSetProxyHeaderRequest { proxy_header: None }
    }
}
/// The new type of proxy header to append before sending data to the backend. NONE or PROXY_V1 are allowed.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProxyHeader {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PROXY_V1")]
    ProxyV1,
}

impl Default for ProxyHeader {
    fn default() -> ProxyHeader {
        Self::None
    }
}
