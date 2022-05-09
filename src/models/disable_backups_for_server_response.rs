/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DisableBackupsForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableBackupsForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DisableBackupsForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_backup
    pub fn new(action: crate::models::Action) -> DisableBackupsForServerResponse {
        DisableBackupsForServerResponse {
            action: Box::new(action),
        }
    }
}


