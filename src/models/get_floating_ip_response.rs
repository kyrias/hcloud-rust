/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFloatingIpResponse : Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFloatingIpResponse {
    #[serde(rename = "floating_ip")]
    pub floating_ip: Box<crate::models::FloatingIp>,
}

impl GetFloatingIpResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to GET https://api.hetzner.cloud/v1/floating_ips/{id}
    pub fn new(floating_ip: crate::models::FloatingIp) -> GetFloatingIpResponse {
        GetFloatingIpResponse {
            floating_ip: Box::new(floating_ip),
        }
    }
}
