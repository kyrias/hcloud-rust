/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// EnableAndConfigureBackupsForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_backup

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableAndConfigureBackupsForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl EnableAndConfigureBackupsForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/enable_backup
    pub fn new(action: crate::models::Action) -> EnableAndConfigureBackupsForServerResponse {
        EnableAndConfigureBackupsForServerResponse {
            action: Box::new(action),
        }
    }
}
