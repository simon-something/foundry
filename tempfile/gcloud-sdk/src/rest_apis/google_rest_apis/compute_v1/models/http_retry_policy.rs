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

/// HttpRetryPolicy : The retry policy associates with HttpRouteRule
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRetryPolicy {
    /// Specifies the allowed number retries. This number must be > 0. If not specified, defaults to 1.
    #[serde(rename = "numRetries", skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i32>,
    #[serde(rename = "perTryTimeout", skip_serializing_if = "Option::is_none")]
    pub per_try_timeout: Option<Box<models::Duration>>,
    /// Specifies one or more conditions when this retry policy applies. Valid values are: - 5xx: retry is attempted if the instance or endpoint responds with any 5xx response code, or if the instance or endpoint does not respond at all. For example, disconnects, reset, read timeout, connection failure, and refused streams. - gateway-error: Similar to 5xx, but only applies to response codes 502, 503 or 504. - connect-failure: a retry is attempted on failures connecting to the instance or endpoint. For example, connection timeouts. - retriable-4xx: a retry is attempted if the instance or endpoint responds with a 4xx response code. The only error that you can retry is error code 409. - refused-stream: a retry is attempted if the instance or endpoint resets the stream with a REFUSED_STREAM error code. This reset type indicates that it is safe to retry. - cancelled: a retry is attempted if the gRPC status code in the response header is set to cancelled. - deadline-exceeded: a retry is attempted if the gRPC status code in the response header is set to deadline-exceeded. - internal: a retry is attempted if the gRPC status code in the response header is set to internal. - resource-exhausted: a retry is attempted if the gRPC status code in the response header is set to resource-exhausted. - unavailable: a retry is attempted if the gRPC status code in the response header is set to unavailable. Only the following codes are supported when the URL map is bound to target gRPC proxy that has validateForProxyless field set to true. - cancelled - deadline-exceeded - internal - resource-exhausted - unavailable
    #[serde(rename = "retryConditions", skip_serializing_if = "Option::is_none")]
    pub retry_conditions: Option<Vec<String>>,
}

impl HttpRetryPolicy {
    /// The retry policy associates with HttpRouteRule
    pub fn new() -> HttpRetryPolicy {
        HttpRetryPolicy {
            num_retries: None,
            per_try_timeout: None,
            retry_conditions: None,
        }
    }
}
