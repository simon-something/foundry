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
pub struct WafExpressionSetExpression {
    /// Expression ID should uniquely identify the origin of the expression. E.g. owasp-crs-v020901-id973337 identifies Owasp core rule set version 2.9.1 rule id 973337. The ID could be used to determine the individual attack definition that has been detected. It could also be used to exclude it from the policy in case of false positive. required
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The sensitivity value associated with the WAF rule ID. This corresponds to the ModSecurity paranoia level, ranging from 1 to 4. 0 is reserved for opt-in only rules.
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<i32>,
}

impl WafExpressionSetExpression {
    pub fn new() -> WafExpressionSetExpression {
        WafExpressionSetExpression {
            id: None,
            sensitivity: None,
        }
    }
}
