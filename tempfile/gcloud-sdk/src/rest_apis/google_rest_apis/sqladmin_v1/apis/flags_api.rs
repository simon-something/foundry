/*
 * Cloud SQL Admin API
 *
 * API for Cloud SQL database instance management
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::google_rest_apis::sqladmin_v1::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`sql_flags_list`]
#[derive(Clone, Debug, Default)]
pub struct SqlPeriodFlagsPeriodListParams {
    /// V1 error format.
    pub dollar_xgafv: Option<String>,
    /// OAuth access token.
    pub access_token: Option<String>,
    /// Data format for response.
    pub alt: Option<String>,
    /// JSONP
    pub callback: Option<String>,
    /// Selector specifying which fields to include in a partial response.
    pub fields: Option<String>,
    /// API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token.
    pub key: Option<String>,
    /// OAuth 2.0 token for the current user.
    pub oauth_token: Option<String>,
    /// Returns response with indentations and line breaks.
    pub pretty_print: Option<bool>,
    /// Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters.
    pub quota_user: Option<String>,
    /// Upload protocol for media (e.g. \"raw\", \"multipart\").
    pub upload_protocol: Option<String>,
    /// Legacy upload protocol for media (e.g. \"media\", \"multipart\").
    pub upload_type: Option<String>,
    /// Database type and version you want to retrieve flags for. By default, this method returns flags for all database types and versions.
    pub database_version: Option<String>,
}

/// struct for typed errors of method [`sql_flags_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SqlPeriodFlagsPeriodListError {
    UnknownValue(serde_json::Value),
}

/// Lists all available database flags for Cloud SQL instances.
pub async fn sql_flags_list(
    configuration: &configuration::Configuration,
    params: SqlPeriodFlagsPeriodListParams,
) -> Result<models::FlagsListResponse, Error<SqlPeriodFlagsPeriodListError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let dollar_xgafv = params.dollar_xgafv;
    let access_token = params.access_token;
    let alt = params.alt;
    let callback = params.callback;
    let fields = params.fields;
    let key = params.key;
    let oauth_token = params.oauth_token;
    let pretty_print = params.pretty_print;
    let quota_user = params.quota_user;
    let upload_protocol = params.upload_protocol;
    let upload_type = params.upload_type;
    let database_version = params.database_version;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/flags", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = dollar_xgafv {
        local_var_req_builder =
            local_var_req_builder.query(&[("$.xgafv", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = access_token {
        local_var_req_builder =
            local_var_req_builder.query(&[("access_token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = alt {
        local_var_req_builder = local_var_req_builder.query(&[("alt", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback {
        local_var_req_builder =
            local_var_req_builder.query(&[("callback", &local_var_str.to_string())]);
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
    if let Some(ref local_var_str) = upload_protocol {
        local_var_req_builder =
            local_var_req_builder.query(&[("upload_protocol", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = upload_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("uploadType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = database_version {
        local_var_req_builder =
            local_var_req_builder.query(&[("databaseVersion", &local_var_str.to_string())]);
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
        let local_var_entity: Option<SqlPeriodFlagsPeriodListError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
