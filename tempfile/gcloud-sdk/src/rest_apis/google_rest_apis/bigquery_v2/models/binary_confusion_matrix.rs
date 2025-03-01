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

/// BinaryConfusionMatrix : Confusion matrix for binary classification models.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryConfusionMatrix {
    /// The fraction of predictions given the correct label.
    #[serde(rename = "accuracy", skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    /// The equally weighted average of recall and precision.
    #[serde(rename = "f1Score", skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    /// Number of false samples predicted as false.
    #[serde(rename = "falseNegatives", skip_serializing_if = "Option::is_none")]
    pub false_negatives: Option<String>,
    /// Number of false samples predicted as true.
    #[serde(rename = "falsePositives", skip_serializing_if = "Option::is_none")]
    pub false_positives: Option<String>,
    /// Threshold value used when computing each of the following metric.
    #[serde(
        rename = "positiveClassThreshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub positive_class_threshold: Option<f64>,
    /// The fraction of actual positive predictions that had positive actual labels.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    /// The fraction of actual positive labels that were given a positive prediction.
    #[serde(rename = "recall", skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
    /// Number of true samples predicted as false.
    #[serde(rename = "trueNegatives", skip_serializing_if = "Option::is_none")]
    pub true_negatives: Option<String>,
    /// Number of true samples predicted as true.
    #[serde(rename = "truePositives", skip_serializing_if = "Option::is_none")]
    pub true_positives: Option<String>,
}

impl BinaryConfusionMatrix {
    /// Confusion matrix for binary classification models.
    pub fn new() -> BinaryConfusionMatrix {
        BinaryConfusionMatrix {
            accuracy: None,
            f1_score: None,
            false_negatives: None,
            false_positives: None,
            positive_class_threshold: None,
            precision: None,
            recall: None,
            true_negatives: None,
            true_positives: None,
        }
    }
}
