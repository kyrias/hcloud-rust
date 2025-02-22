/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachIsoToServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_iso

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachIsoToServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AttachIsoToServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_iso
    pub fn new(action: crate::models::Action) -> AttachIsoToServerResponse {
        AttachIsoToServerResponse {
            action: Box::new(action),
        }
    }
}
