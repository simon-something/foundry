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
pub struct InstanceGroupManagerUpdatePolicy {
    /// The instance redistribution policy for regional managed instance groups. Valid values are: - PROACTIVE (default): The group attempts to maintain an even distribution of VM instances across zones in the region. - NONE: For non-autoscaled groups, proactive redistribution is disabled.
    #[serde(
        rename = "instanceRedistributionType",
        skip_serializing_if = "Option::is_none"
    )]
    pub instance_redistribution_type: Option<InstanceRedistributionType>,
    #[serde(rename = "maxSurge", skip_serializing_if = "Option::is_none")]
    pub max_surge: Option<Box<models::FixedOrPercent>>,
    #[serde(rename = "maxUnavailable", skip_serializing_if = "Option::is_none")]
    pub max_unavailable: Option<Box<models::FixedOrPercent>>,
    /// Minimal action to be taken on an instance. Use this option to minimize disruption as much as possible or to apply a more disruptive action than is necessary. - To limit disruption as much as possible, set the minimal action to REFRESH. If your update requires a more disruptive action, Compute Engine performs the necessary action to execute the update. - To apply a more disruptive action than is strictly necessary, set the minimal action to RESTART or REPLACE. For example, Compute Engine does not need to restart a VM to change its metadata. But if your application reads instance metadata only when a VM is restarted, you can set the minimal action to RESTART in order to pick up metadata changes.
    #[serde(rename = "minimalAction", skip_serializing_if = "Option::is_none")]
    pub minimal_action: Option<MinimalAction>,
    /// Most disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to avoid restarting the VM and to limit disruption as much as possible. RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all.
    #[serde(
        rename = "mostDisruptiveAllowedAction",
        skip_serializing_if = "Option::is_none"
    )]
    pub most_disruptive_allowed_action: Option<MostDisruptiveAllowedAction>,
    /// What action should be used to replace instances. See minimal_action.REPLACE
    #[serde(rename = "replacementMethod", skip_serializing_if = "Option::is_none")]
    pub replacement_method: Option<ReplacementMethod>,
    /// The type of update process. You can specify either PROACTIVE so that the MIG automatically updates VMs to the latest configurations or OPPORTUNISTIC so that you can select the VMs that you want to update.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl InstanceGroupManagerUpdatePolicy {
    pub fn new() -> InstanceGroupManagerUpdatePolicy {
        InstanceGroupManagerUpdatePolicy {
            instance_redistribution_type: None,
            max_surge: None,
            max_unavailable: None,
            minimal_action: None,
            most_disruptive_allowed_action: None,
            replacement_method: None,
            r#type: None,
        }
    }
}
/// The instance redistribution policy for regional managed instance groups. Valid values are: - PROACTIVE (default): The group attempts to maintain an even distribution of VM instances across zones in the region. - NONE: For non-autoscaled groups, proactive redistribution is disabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstanceRedistributionType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PROACTIVE")]
    Proactive,
}

impl Default for InstanceRedistributionType {
    fn default() -> InstanceRedistributionType {
        Self::None
    }
}
/// Minimal action to be taken on an instance. Use this option to minimize disruption as much as possible or to apply a more disruptive action than is necessary. - To limit disruption as much as possible, set the minimal action to REFRESH. If your update requires a more disruptive action, Compute Engine performs the necessary action to execute the update. - To apply a more disruptive action than is strictly necessary, set the minimal action to RESTART or REPLACE. For example, Compute Engine does not need to restart a VM to change its metadata. But if your application reads instance metadata only when a VM is restarted, you can set the minimal action to RESTART in order to pick up metadata changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MinimalAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "RESTART")]
    Restart,
}

impl Default for MinimalAction {
    fn default() -> MinimalAction {
        Self::None
    }
}
/// Most disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to avoid restarting the VM and to limit disruption as much as possible. RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MostDisruptiveAllowedAction {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "REFRESH")]
    Refresh,
    #[serde(rename = "REPLACE")]
    Replace,
    #[serde(rename = "RESTART")]
    Restart,
}

impl Default for MostDisruptiveAllowedAction {
    fn default() -> MostDisruptiveAllowedAction {
        Self::None
    }
}
/// What action should be used to replace instances. See minimal_action.REPLACE
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReplacementMethod {
    #[serde(rename = "RECREATE")]
    Recreate,
    #[serde(rename = "SUBSTITUTE")]
    Substitute,
}

impl Default for ReplacementMethod {
    fn default() -> ReplacementMethod {
        Self::Recreate
    }
}
/// The type of update process. You can specify either PROACTIVE so that the MIG automatically updates VMs to the latest configurations or OPPORTUNISTIC so that you can select the VMs that you want to update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "OPPORTUNISTIC")]
    Opportunistic,
    #[serde(rename = "PROACTIVE")]
    Proactive,
}

impl Default for Type {
    fn default() -> Type {
        Self::Opportunistic
    }
}
