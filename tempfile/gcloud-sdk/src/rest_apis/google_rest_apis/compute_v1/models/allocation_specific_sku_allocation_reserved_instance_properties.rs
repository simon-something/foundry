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

/// AllocationSpecificSkuAllocationReservedInstanceProperties : Properties of the SKU instances being reserved. Next ID: 9
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllocationSpecificSkuAllocationReservedInstanceProperties {
    /// Specifies accelerator type and count.
    #[serde(rename = "guestAccelerators", skip_serializing_if = "Option::is_none")]
    pub guest_accelerators: Option<Vec<models::AcceleratorConfig>>,
    /// Specifies amount of local ssd to reserve with each instance. The type of disk is local-ssd.
    #[serde(rename = "localSsds", skip_serializing_if = "Option::is_none")]
    pub local_ssds:
        Option<Vec<models::AllocationSpecificSkuAllocationAllocatedInstancePropertiesReservedDisk>>,
    /// An opaque location hint used to place the allocation close to other resources. This field is for use by internal tools that use the public API.
    #[serde(rename = "locationHint", skip_serializing_if = "Option::is_none")]
    pub location_hint: Option<String>,
    /// Specifies type of machine (name only) which has fixed number of vCPUs and fixed amount of memory. This also includes specifying custom machine type following custom-NUMBER_OF_CPUS-AMOUNT_OF_MEMORY pattern.
    #[serde(rename = "machineType", skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,
    /// Minimum cpu platform the reservation.
    #[serde(rename = "minCpuPlatform", skip_serializing_if = "Option::is_none")]
    pub min_cpu_platform: Option<String>,
}

impl AllocationSpecificSkuAllocationReservedInstanceProperties {
    /// Properties of the SKU instances being reserved. Next ID: 9
    pub fn new() -> AllocationSpecificSkuAllocationReservedInstanceProperties {
        AllocationSpecificSkuAllocationReservedInstanceProperties {
            guest_accelerators: None,
            local_ssds: None,
            location_hint: None,
            machine_type: None,
            min_cpu_platform: None,
        }
    }
}
