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

/// HttpRedirectAction : Specifies settings for an HTTP redirect.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRedirectAction {
    /// The host that is used in the redirect response instead of the one that was supplied in the request. The value must be from 1 to 255 characters.
    #[serde(rename = "hostRedirect", skip_serializing_if = "Option::is_none")]
    pub host_redirect: Option<String>,
    /// If set to true, the URL scheme in the redirected request is set to HTTPS. If set to false, the URL scheme of the redirected request remains the same as that of the request. This must only be set for URL maps used in TargetHttpProxys. Setting this true for TargetHttpsProxy is not permitted. The default is set to false.
    #[serde(rename = "httpsRedirect", skip_serializing_if = "Option::is_none")]
    pub https_redirect: Option<bool>,
    /// The path that is used in the redirect response instead of the one that was supplied in the request. pathRedirect cannot be supplied together with prefixRedirect. Supply one alone or neither. If neither is supplied, the path of the original request is used for the redirect. The value must be from 1 to 1024 characters.
    #[serde(rename = "pathRedirect", skip_serializing_if = "Option::is_none")]
    pub path_redirect: Option<String>,
    /// The prefix that replaces the prefixMatch specified in the HttpRouteRuleMatch, retaining the remaining portion of the URL before redirecting the request. prefixRedirect cannot be supplied together with pathRedirect. Supply one alone or neither. If neither is supplied, the path of the original request is used for the redirect. The value must be from 1 to 1024 characters.
    #[serde(rename = "prefixRedirect", skip_serializing_if = "Option::is_none")]
    pub prefix_redirect: Option<String>,
    /// The HTTP Status code to use for this RedirectAction. Supported values are: - MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301. - FOUND, which corresponds to 302. - SEE_OTHER which corresponds to 303. - TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method is retained. - PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method is retained.
    #[serde(
        rename = "redirectResponseCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub redirect_response_code: Option<RedirectResponseCode>,
    /// If set to true, any accompanying query portion of the original URL is removed before redirecting the request. If set to false, the query portion of the original URL is retained. The default is set to false.
    #[serde(rename = "stripQuery", skip_serializing_if = "Option::is_none")]
    pub strip_query: Option<bool>,
}

impl HttpRedirectAction {
    /// Specifies settings for an HTTP redirect.
    pub fn new() -> HttpRedirectAction {
        HttpRedirectAction {
            host_redirect: None,
            https_redirect: None,
            path_redirect: None,
            prefix_redirect: None,
            redirect_response_code: None,
            strip_query: None,
        }
    }
}
/// The HTTP Status code to use for this RedirectAction. Supported values are: - MOVED_PERMANENTLY_DEFAULT, which is the default value and corresponds to 301. - FOUND, which corresponds to 302. - SEE_OTHER which corresponds to 303. - TEMPORARY_REDIRECT, which corresponds to 307. In this case, the request method is retained. - PERMANENT_REDIRECT, which corresponds to 308. In this case, the request method is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedirectResponseCode {
    #[serde(rename = "FOUND")]
    Found,
    #[serde(rename = "MOVED_PERMANENTLY_DEFAULT")]
    MovedPermanentlyDefault,
    #[serde(rename = "PERMANENT_REDIRECT")]
    PermanentRedirect,
    #[serde(rename = "SEE_OTHER")]
    SeeOther,
    #[serde(rename = "TEMPORARY_REDIRECT")]
    TemporaryRedirect,
}

impl Default for RedirectResponseCode {
    fn default() -> RedirectResponseCode {
        Self::Found
    }
}
