/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateFloatingIpRequest : Request for POST https://api.hetzner.cloud/v1/floating_ips

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFloatingIpRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Home Location (routing is optimized for that Location). Only optional if Server argument is passed.
    #[serde(rename = "home_location", skip_serializing_if = "Option::is_none")]
    pub home_location: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Server to assign the Floating IP to
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: crate::models::IpType,
}

impl CreateFloatingIpRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/floating_ips
    pub fn new(r#type: crate::models::IpType) -> CreateFloatingIpRequest {
        CreateFloatingIpRequest {
            description: None,
            home_location: None,
            labels: None,
            name: None,
            server: None,
            r#type,
        }
    }
}
