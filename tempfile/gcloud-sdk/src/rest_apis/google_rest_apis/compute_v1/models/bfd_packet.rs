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
pub struct BfdPacket {
    /// The Authentication Present bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(
        rename = "authenticationPresent",
        skip_serializing_if = "Option::is_none"
    )]
    pub authentication_present: Option<bool>,
    /// The Control Plane Independent bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(
        rename = "controlPlaneIndependent",
        skip_serializing_if = "Option::is_none"
    )]
    pub control_plane_independent: Option<bool>,
    /// The demand bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "demand", skip_serializing_if = "Option::is_none")]
    pub demand: Option<bool>,
    /// The diagnostic code specifies the local system's reason for the last change in session state. This allows remote systems to determine the reason that the previous session failed, for example. These diagnostic codes are specified in section 4.1 of RFC5880
    #[serde(rename = "diagnostic", skip_serializing_if = "Option::is_none")]
    pub diagnostic: Option<Diagnostic>,
    /// The Final bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub r#final: Option<bool>,
    /// The length of the BFD Control packet in bytes. This is specified in section 4.1 of RFC5880
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// The Required Min Echo RX Interval value in the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(
        rename = "minEchoRxIntervalMs",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_echo_rx_interval_ms: Option<i32>,
    /// The Required Min RX Interval value in the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "minRxIntervalMs", skip_serializing_if = "Option::is_none")]
    pub min_rx_interval_ms: Option<i32>,
    /// The Desired Min TX Interval value in the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "minTxIntervalMs", skip_serializing_if = "Option::is_none")]
    pub min_tx_interval_ms: Option<i32>,
    /// The detection time multiplier of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
    /// The multipoint bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "multipoint", skip_serializing_if = "Option::is_none")]
    pub multipoint: Option<bool>,
    /// The My Discriminator value in the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "myDiscriminator", skip_serializing_if = "Option::is_none")]
    pub my_discriminator: Option<i32>,
    /// The Poll bit of the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "poll", skip_serializing_if = "Option::is_none")]
    pub poll: Option<bool>,
    /// The current BFD session state as seen by the transmitting system. These states are specified in section 4.1 of RFC5880
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// The version number of the BFD protocol, as specified in section 4.1 of RFC5880.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    /// The Your Discriminator value in the BFD packet. This is specified in section 4.1 of RFC5880
    #[serde(rename = "yourDiscriminator", skip_serializing_if = "Option::is_none")]
    pub your_discriminator: Option<i32>,
}

impl BfdPacket {
    pub fn new() -> BfdPacket {
        BfdPacket {
            authentication_present: None,
            control_plane_independent: None,
            demand: None,
            diagnostic: None,
            r#final: None,
            length: None,
            min_echo_rx_interval_ms: None,
            min_rx_interval_ms: None,
            min_tx_interval_ms: None,
            multiplier: None,
            multipoint: None,
            my_discriminator: None,
            poll: None,
            state: None,
            version: None,
            your_discriminator: None,
        }
    }
}
/// The diagnostic code specifies the local system's reason for the last change in session state. This allows remote systems to determine the reason that the previous session failed, for example. These diagnostic codes are specified in section 4.1 of RFC5880
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Diagnostic {
    #[serde(rename = "ADMINISTRATIVELY_DOWN")]
    AdministrativelyDown,
    #[serde(rename = "CONCATENATED_PATH_DOWN")]
    ConcatenatedPathDown,
    #[serde(rename = "CONTROL_DETECTION_TIME_EXPIRED")]
    ControlDetectionTimeExpired,
    #[serde(rename = "DIAGNOSTIC_UNSPECIFIED")]
    DiagnosticUnspecified,
    #[serde(rename = "ECHO_FUNCTION_FAILED")]
    EchoFunctionFailed,
    #[serde(rename = "FORWARDING_PLANE_RESET")]
    ForwardingPlaneReset,
    #[serde(rename = "NEIGHBOR_SIGNALED_SESSION_DOWN")]
    NeighborSignaledSessionDown,
    #[serde(rename = "NO_DIAGNOSTIC")]
    NoDiagnostic,
    #[serde(rename = "PATH_DOWN")]
    PathDown,
    #[serde(rename = "REVERSE_CONCATENATED_PATH_DOWN")]
    ReverseConcatenatedPathDown,
}

impl Default for Diagnostic {
    fn default() -> Diagnostic {
        Self::AdministrativelyDown
    }
}
/// The current BFD session state as seen by the transmitting system. These states are specified in section 4.1 of RFC5880
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ADMIN_DOWN")]
    AdminDown,
    #[serde(rename = "DOWN")]
    Down,
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "STATE_UNSPECIFIED")]
    StateUnspecified,
    #[serde(rename = "UP")]
    Up,
}

impl Default for State {
    fn default() -> State {
        Self::AdminDown
    }
}
