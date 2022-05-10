/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.7.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResetRootPasswordOfServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset_password



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetRootPasswordOfServerResponse {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Box<crate::models::Action>>,
    /// Password that will be set for this Server once the Action succeeds
    #[serde(rename = "root_password", skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
}

impl ResetRootPasswordOfServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/reset_password
    pub fn new() -> ResetRootPasswordOfServerResponse {
        ResetRootPasswordOfServerResponse {
            action: None,
            root_password: None,
        }
    }
}


