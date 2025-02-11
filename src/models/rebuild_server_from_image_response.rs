/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RebuildServerFromImageResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RebuildServerFromImageResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    /// New root password when not using SSH keys
    #[serde(rename = "root_password", skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
}

impl RebuildServerFromImageResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/rebuild
    pub fn new() -> RebuildServerFromImageResponse {
        RebuildServerFromImageResponse {
            action: None,
            root_password: None,
        }
    }
}
