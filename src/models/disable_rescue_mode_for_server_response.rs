/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DisableRescueModeForServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_rescue

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableRescueModeForServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DisableRescueModeForServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/disable_rescue
    pub fn new(action: crate::models::Action) -> DisableRescueModeForServerResponse {
        DisableRescueModeForServerResponse {
            action: Box::new(action),
        }
    }
}
