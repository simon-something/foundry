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
pub struct AllocationAggregateReservationReservedResourceInfo {
    #[serde(rename = "accelerator", skip_serializing_if = "Option::is_none")]
    pub accelerator:
        Option<Box<models::AllocationAggregateReservationReservedResourceInfoAccelerator>>,
}

impl AllocationAggregateReservationReservedResourceInfo {
    pub fn new() -> AllocationAggregateReservationReservedResourceInfo {
        AllocationAggregateReservationReservedResourceInfo { accelerator: None }
    }
}
