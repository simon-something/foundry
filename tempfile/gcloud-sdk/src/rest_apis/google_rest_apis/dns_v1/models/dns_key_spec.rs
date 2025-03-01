/*
 * Cloud DNS API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::google_rest_apis::dns_v1::models;
use serde::{Deserialize, Serialize};

/// DnsKeySpec : Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone and as default when adding a new DnsKey.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsKeySpec {
    /// String mnemonic specifying the DNSSEC algorithm of this key.
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<Algorithm>,
    /// Length of the keys in bits.
    #[serde(rename = "keyLength", skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i32>,
    /// Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets.
    #[serde(rename = "keyType", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DnsKeySpec {
    /// Parameters for DnsKey key generation. Used for generating initial keys for a new ManagedZone and as default when adding a new DnsKey.
    pub fn new() -> DnsKeySpec {
        DnsKeySpec {
            algorithm: None,
            key_length: None,
            key_type: None,
            kind: None,
        }
    }
}
/// String mnemonic specifying the DNSSEC algorithm of this key.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(rename = "rsasha1")]
    Rsasha1,
    #[serde(rename = "rsasha256")]
    Rsasha256,
    #[serde(rename = "rsasha512")]
    Rsasha512,
    #[serde(rename = "ecdsap256sha256")]
    Ecdsap256sha256,
    #[serde(rename = "ecdsap384sha384")]
    Ecdsap384sha384,
}

impl Default for Algorithm {
    fn default() -> Algorithm {
        Self::Rsasha1
    }
}
/// Specifies whether this is a key signing key (KSK) or a zone signing key (ZSK). Key signing keys have the Secure Entry Point flag set and, when active, are only used to sign resource record sets of type DNSKEY. Zone signing keys do not have the Secure Entry Point flag set and are used to sign all other types of resource record sets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum KeyType {
    #[serde(rename = "keySigning")]
    KeySigning,
    #[serde(rename = "zoneSigning")]
    ZoneSigning,
}

impl Default for KeyType {
    fn default() -> KeyType {
        Self::KeySigning
    }
}
