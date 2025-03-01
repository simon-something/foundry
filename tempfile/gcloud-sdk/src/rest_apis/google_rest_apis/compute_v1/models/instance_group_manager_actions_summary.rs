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
pub struct InstanceGroupManagerActionsSummary {
    /// [Output Only] The total number of instances in the managed instance group that are scheduled to be abandoned. Abandoning an instance removes it from the managed instance group without deleting it.
    #[serde(rename = "abandoning", skip_serializing_if = "Option::is_none")]
    pub abandoning: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be created or are currently being created. If the group fails to create any of these instances, it tries again until it creates the instance successfully. If you have disabled creation retries, this field will not be populated; instead, the creatingWithoutRetries field will be populated.
    #[serde(rename = "creating", skip_serializing_if = "Option::is_none")]
    pub creating: Option<i32>,
    /// [Output Only] The number of instances that the managed instance group will attempt to create. The group attempts to create each instance only once. If the group fails to create any of these instances, it decreases the group's targetSize value accordingly.
    #[serde(
        rename = "creatingWithoutRetries",
        skip_serializing_if = "Option::is_none"
    )]
    pub creating_without_retries: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be deleted or are currently being deleted.
    #[serde(rename = "deleting", skip_serializing_if = "Option::is_none")]
    pub deleting: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are running and have no scheduled actions.
    #[serde(rename = "none", skip_serializing_if = "Option::is_none")]
    pub none: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be recreated or are currently being being recreated. Recreating an instance deletes the existing root persistent disk and creates a new disk from the image that is defined in the instance template.
    #[serde(rename = "recreating", skip_serializing_if = "Option::is_none")]
    pub recreating: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are being reconfigured with properties that do not require a restart or a recreate action. For example, setting or removing target pools for the instance.
    #[serde(rename = "refreshing", skip_serializing_if = "Option::is_none")]
    pub refreshing: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be restarted or are currently being restarted.
    #[serde(rename = "restarting", skip_serializing_if = "Option::is_none")]
    pub restarting: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be resumed or are currently being resumed.
    #[serde(rename = "resuming", skip_serializing_if = "Option::is_none")]
    pub resuming: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be started or are currently being started.
    #[serde(rename = "starting", skip_serializing_if = "Option::is_none")]
    pub starting: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be stopped or are currently being stopped.
    #[serde(rename = "stopping", skip_serializing_if = "Option::is_none")]
    pub stopping: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are scheduled to be suspended or are currently being suspended.
    #[serde(rename = "suspending", skip_serializing_if = "Option::is_none")]
    pub suspending: Option<i32>,
    /// [Output Only] The number of instances in the managed instance group that are being verified. See the managedInstances[].currentAction property in the listManagedInstances method documentation.
    #[serde(rename = "verifying", skip_serializing_if = "Option::is_none")]
    pub verifying: Option<i32>,
}

impl InstanceGroupManagerActionsSummary {
    pub fn new() -> InstanceGroupManagerActionsSummary {
        InstanceGroupManagerActionsSummary {
            abandoning: None,
            creating: None,
            creating_without_retries: None,
            deleting: None,
            none: None,
            recreating: None,
            refreshing: None,
            restarting: None,
            resuming: None,
            starting: None,
            stopping: None,
            suspending: None,
            verifying: None,
        }
    }
}
