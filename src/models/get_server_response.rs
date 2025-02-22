/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetServerResponse {
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::Server>>,
}

impl GetServerResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}
    pub fn new() -> GetServerResponse {
        GetServerResponse { server: None }
    }
}
