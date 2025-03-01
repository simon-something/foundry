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
pub struct Uint128 {
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
}

impl Uint128 {
    pub fn new() -> Uint128 {
        Uint128 {
            high: None,
            low: None,
        }
    }
}
