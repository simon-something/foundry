/*
 * BigQuery API
 *
 * A data platform for customers to create, manage, share and query data.
 *
 * The version of the OpenAPI document: v2
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::bigquery_v2::models;
use serde::{Deserialize, Serialize};

/// PrincipalComponentInfo : Principal component infos, used only for eigen decomposition based models, e.g., PCA. Ordered by explained_variance in the descending order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrincipalComponentInfo {
    /// The explained_variance is pre-ordered in the descending order to compute the cumulative explained variance ratio.
    #[serde(
        rename = "cumulativeExplainedVarianceRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub cumulative_explained_variance_ratio: Option<f64>,
    /// Explained variance by this principal component, which is simply the eigenvalue.
    #[serde(rename = "explainedVariance", skip_serializing_if = "Option::is_none")]
    pub explained_variance: Option<f64>,
    /// Explained_variance over the total explained variance.
    #[serde(
        rename = "explainedVarianceRatio",
        skip_serializing_if = "Option::is_none"
    )]
    pub explained_variance_ratio: Option<f64>,
    /// Id of the principal component.
    #[serde(
        rename = "principalComponentId",
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_component_id: Option<String>,
}

impl PrincipalComponentInfo {
    /// Principal component infos, used only for eigen decomposition based models, e.g., PCA. Ordered by explained_variance in the descending order.
    pub fn new() -> PrincipalComponentInfo {
        PrincipalComponentInfo {
            cumulative_explained_variance_ratio: None,
            explained_variance: None,
            explained_variance_ratio: None,
            principal_component_id: None,
        }
    }
}
