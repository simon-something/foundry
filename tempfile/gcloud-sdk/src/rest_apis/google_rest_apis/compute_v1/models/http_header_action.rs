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

/// HttpHeaderAction : The request and response header transformations that take effect before the request is passed along to the selected backendService.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpHeaderAction {
    /// Headers to add to a matching request before forwarding the request to the backendService.
    #[serde(
        rename = "requestHeadersToAdd",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers_to_add: Option<Vec<models::HttpHeaderOption>>,
    /// A list of header names for headers that need to be removed from the request before forwarding the request to the backendService.
    #[serde(
        rename = "requestHeadersToRemove",
        skip_serializing_if = "Option::is_none"
    )]
    pub request_headers_to_remove: Option<Vec<String>>,
    /// Headers to add the response before sending the response back to the client.
    #[serde(
        rename = "responseHeadersToAdd",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_headers_to_add: Option<Vec<models::HttpHeaderOption>>,
    /// A list of header names for headers that need to be removed from the response before sending the response back to the client.
    #[serde(
        rename = "responseHeadersToRemove",
        skip_serializing_if = "Option::is_none"
    )]
    pub response_headers_to_remove: Option<Vec<String>>,
}

impl HttpHeaderAction {
    /// The request and response header transformations that take effect before the request is passed along to the selected backendService.
    pub fn new() -> HttpHeaderAction {
        HttpHeaderAction {
            request_headers_to_add: None,
            request_headers_to_remove: None,
            response_headers_to_add: None,
            response_headers_to_remove: None,
        }
    }
}
