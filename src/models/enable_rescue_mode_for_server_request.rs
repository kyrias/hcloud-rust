/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EnableRescueModeForServerRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_rescue

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableRescueModeForServerRequest {
    /// Array of SSH key IDs which should be injected into the rescue system.
    #[serde(rename = "ssh_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<i32>>,
    /// Type of rescue system to boot (default: `linux64`)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl EnableRescueModeForServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_rescue
    pub fn new() -> EnableRescueModeForServerRequest {
        EnableRescueModeForServerRequest {
            ssh_keys: None,
            r#type: None,
        }
    }
}

/// Type of rescue system to boot (default: `linux64`)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "linux32")]
    Linux32,
    #[serde(rename = "linux64")]
    Linux64,
}

impl Default for Type {
    fn default() -> Type {
        Self::Linux32
    }
}
