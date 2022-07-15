/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssignFloatingIpToServerRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssignFloatingIpToServerRequest {
    /// ID of the Server the Floating IP shall be assigned to
    #[serde(rename = "server")]
    pub server: i32,
}

impl AssignFloatingIpToServerRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/assign
    pub fn new(server: i32) -> AssignFloatingIpToServerRequest {
        AssignFloatingIpToServerRequest { server }
    }
}
