/*
 * Cloud Storage JSON API
 *
 * Stores and retrieves potentially large, immutable data objects.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::storage_v1::models;
use serde::{Deserialize, Serialize};

/// Notifications : A list of notification subscriptions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notifications {
    /// The list of items.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::Notification>>,
    /// The kind of item this is. For lists of notifications, this is always storage#notifications.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl Notifications {
    /// A list of notification subscriptions.
    pub fn new() -> Notifications {
        Notifications {
            items: None,
            kind: None,
        }
    }
}
