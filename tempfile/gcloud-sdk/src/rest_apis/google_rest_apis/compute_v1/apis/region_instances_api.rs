/*
 * Compute Engine API
 *
 * Creates and runs virtual machines on Google Cloud Platform.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::google_rest_apis::compute_v1::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`compute_region_instances_bulk_insert`]
#[derive(Clone, Debug, Default)]
pub struct ComputePeriodRegionInstancesPeriodBulkInsertParams {
    /// Project ID for this request.
    pub project: String,
    /// The name of the region for this request.
    pub region: String,
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
    /// Legacy name for parameter that has been superseded by `quotaUser`.
    pub user_ip: Option<String>,
    /// An optional request ID to identify requests. Specify a unique request ID so that if you must retry your request, the server will know to ignore the request if it has already been completed. For example, consider a situation where you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if original operation with the same request ID was received, and if so, will ignore the second request. This prevents clients from accidentally creating duplicate commitments. The request ID must be a valid UUID with the exception that zero UUID is not supported ( 00000000-0000-0000-0000-000000000000).
    pub request_id: Option<String>,
    pub bulk_insert_instance_resource: Option<models::BulkInsertInstanceResource>,
}

/// struct for typed errors of method [`compute_region_instances_bulk_insert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputePeriodRegionInstancesPeriodBulkInsertError {
    UnknownValue(serde_json::Value),
}

/// Creates multiple instances in a given region. Count specifies the number of instances to create.
pub async fn compute_region_instances_bulk_insert(
    configuration: &configuration::Configuration,
    params: ComputePeriodRegionInstancesPeriodBulkInsertParams,
) -> Result<models::Operation, Error<ComputePeriodRegionInstancesPeriodBulkInsertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let project = params.project;
    let region = params.region;
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
    let user_ip = params.user_ip;
    let request_id = params.request_id;
    let bulk_insert_instance_resource = params.bulk_insert_instance_resource;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/projects/{project}/regions/{region}/instances/bulkInsert",
        local_var_configuration.base_path,
        project = crate::google_rest_apis::compute_v1::apis::urlencode(project),
        region = crate::google_rest_apis::compute_v1::apis::urlencode(region)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    if let Some(ref local_var_str) = user_ip {
        local_var_req_builder =
            local_var_req_builder.query(&[("userIp", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = request_id {
        local_var_req_builder =
            local_var_req_builder.query(&[("requestId", &local_var_str.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&bulk_insert_instance_resource);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ComputePeriodRegionInstancesPeriodBulkInsertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
