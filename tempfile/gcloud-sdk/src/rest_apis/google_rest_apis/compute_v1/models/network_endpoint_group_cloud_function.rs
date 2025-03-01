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

/// NetworkEndpointGroupCloudFunction : Configuration for a Cloud Function network endpoint group (NEG). The function must be provided explicitly or in the URL mask. Note: Cloud Function must be in the same project and located in the same region as the Serverless NEG.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkEndpointGroupCloudFunction {
    /// A user-defined name of the Cloud Function. The function name is case-sensitive and must be 1-63 characters long. Example value: func1.
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    /// An URL mask is one of the main components of the Cloud Function. A template to parse function field from a request URL. URL mask allows for routing to multiple Cloud Functions without having to create multiple Network Endpoint Groups and backend services. For example, request URLs mydomain.com/function1 and mydomain.com/function2 can be backed by the same Serverless NEG with URL mask /<function>. The URL mask will parse them to { function = \"function1\" } and { function = \"function2\" } respectively.
    #[serde(rename = "urlMask", skip_serializing_if = "Option::is_none")]
    pub url_mask: Option<String>,
}

impl NetworkEndpointGroupCloudFunction {
    /// Configuration for a Cloud Function network endpoint group (NEG). The function must be provided explicitly or in the URL mask. Note: Cloud Function must be in the same project and located in the same region as the Serverless NEG.
    pub fn new() -> NetworkEndpointGroupCloudFunction {
        NetworkEndpointGroupCloudFunction {
            function: None,
            url_mask: None,
        }
    }
}
