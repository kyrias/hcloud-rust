/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AttachServerToNetworkResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachServerToNetworkResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AttachServerToNetworkResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_to_network
    pub fn new(action: crate::models::Action) -> AttachServerToNetworkResponse {
        AttachServerToNetworkResponse {
            action: Box::new(action),
        }
    }
}
