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

/// ObjectAccessControl : An access-control entry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectAccessControl {
    /// The name of the bucket.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// The domain associated with the entity, if any.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email address associated with the entity, if any.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The entity holding the permission, in one of the following forms:  - user-userId  - user-email  - group-groupId  - group-email  - domain-domain  - project-team-projectId  - allUsers  - allAuthenticatedUsers Examples:  - The user liz@example.com would be user-liz@example.com.  - The group example@googlegroups.com would be group-example@googlegroups.com.  - To refer to all members of the Google Apps for Business domain example.com, the entity would be domain-example.com.
    #[serde(rename = "entity", skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    /// The ID for the entity, if any.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The content generation of the object, if applied to an object.
    #[serde(rename = "generation", skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    /// The ID of the access-control entry.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The kind of item this is. For object access control entries, this is always storage#objectAccessControl.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// The name of the object, if applied to an object.
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "projectTeam", skip_serializing_if = "Option::is_none")]
    pub project_team: Option<Box<models::BucketAccessControlProjectTeam>>,
    /// The access permission for the entity.
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The link to this access-control entry.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
}

impl ObjectAccessControl {
    /// An access-control entry.
    pub fn new() -> ObjectAccessControl {
        ObjectAccessControl {
            bucket: None,
            domain: None,
            email: None,
            entity: None,
            entity_id: None,
            etag: None,
            generation: None,
            id: None,
            kind: None,
            object: None,
            project_team: None,
            role: None,
            self_link: None,
        }
    }
}
