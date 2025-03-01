/*
 * Cloud Storage JSON API
 *
 * Stores and retrieves potentially large, immutable data objects.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::google_rest_apis::storage_v1::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`storage_object_access_controls_delete`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodDeleteParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_object_access_controls_get`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodGetParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_object_access_controls_insert`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodInsertParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for passing parameters to the method [`storage_object_access_controls_list`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodListParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
}

/// struct for passing parameters to the method [`storage_object_access_controls_patch`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodPatchParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for passing parameters to the method [`storage_object_access_controls_update`]
#[derive(Clone, Debug, Default)]
pub struct StoragePeriodObjectAccessControlsPeriodUpdateParams {
    /// Name of a bucket.
    pub bucket: String,
    /// Name of the object. For information about how to URL encode object names to be path safe, see [Encoding URI Path Parts](https://cloud.google.com/storage/docs/request-endpoints#encoding).
    pub object: String,
    /// The entity holding the permission. Can be user-userId, user-emailAddress, group-groupId, group-emailAddress, allUsers, or allAuthenticatedUsers.
    pub entity: String,
    /// Data format for the response.
    pub alt: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// An opaque string that represents a user for quota purposes. Must not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"media\", \"multipart\", \"resumable\").
    pub upload_type: Option<String>,
    /// Deprecated. Please use quotaUser instead.
    pub user_ip: Option<String>,
    /// If present, selects a specific revision of this object (as opposed to the latest version, the default).
    pub generation: Option<String>,
    /// The project to be billed for this request. Required for Requester Pays buckets.
    pub user_project: Option<String>,
    pub object_access_control: Option<models::ObjectAccessControl>,
}

/// struct for typed errors of method [`storage_object_access_controls_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_object_access_controls_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_object_access_controls_insert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodInsertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_object_access_controls_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodListError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_object_access_controls_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodPatchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`storage_object_access_controls_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StoragePeriodObjectAccessControlsPeriodUpdateError {
    UnknownValue(serde_json::Value),
}

/// Permanently deletes the ACL entry for the specified entity on the specified object.
pub async fn storage_object_access_controls_delete(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodDeleteParams,
) -> Result<(), Error<StoragePeriodObjectAccessControlsPeriodDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the ACL entry for the specified entity on the specified object.
pub async fn storage_object_access_controls_get(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodGetParams,
) -> Result<models::ObjectAccessControl, Error<StoragePeriodObjectAccessControlsPeriodGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new ACL entry on the specified object.
pub async fn storage_object_access_controls_insert(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodInsertParams,
) -> Result<models::ObjectAccessControl, Error<StoragePeriodObjectAccessControlsPeriodInsertError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodInsertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves ACL entries on the specified object.
pub async fn storage_object_access_controls_list(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodListParams,
) -> Result<models::ObjectAccessControls, Error<StoragePeriodObjectAccessControlsPeriodListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Patches an ACL entry on the specified object.
pub async fn storage_object_access_controls_patch(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodPatchParams,
) -> Result<models::ObjectAccessControl, Error<StoragePeriodObjectAccessControlsPeriodPatchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodPatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an ACL entry on the specified object.
pub async fn storage_object_access_controls_update(
    configuration: &configuration::Configuration,
    params: StoragePeriodObjectAccessControlsPeriodUpdateParams,
) -> Result<models::ObjectAccessControl, Error<StoragePeriodObjectAccessControlsPeriodUpdateError>>
{
    let local_var_configuration = configuration;

    // unbox the parameters
    let bucket = params.bucket;
    let object = params.object;
    let entity = params.entity;
    let alt = params.alt;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_type = params.upload_type;
    let user_ip = params.user_ip;
    let generation = params.generation;
    let user_project = params.user_project;
    let object_access_control = params.object_access_control;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/b/{bucket}/o/{object}/acl/{entity}",
        local_var_configuration.base_path,
        bucket = crate::google_rest_apis::storage_v1::apis::urlencode(bucket),
        object = crate::google_rest_apis::storage_v1::apis::urlencode(object),
        entity = crate::google_rest_apis::storage_v1::apis::urlencode(entity)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = key {
        local_var_req_builder = local_var_req_builder.query(&[("key", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oauth_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("oauth_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = pretty_print {
        local_var_req_builder =
            local_var_req_builder.query(&[("prettyPrint", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = quota_user {
        local_var_req_builder =
            local_var_req_builder.query(&[("quotaUser", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = generation {
        local_var_req_builder =
            local_var_req_builder.query(&[("generation", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = user_project {
        local_var_req_builder =
            local_var_req_builder.query(&[("userProject", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&object_access_control);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StoragePeriodObjectAccessControlsPeriodUpdateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
