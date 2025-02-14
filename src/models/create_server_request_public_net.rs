/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerRequestPublicNet : Public Network options

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerRequestPublicNet {
    /// Attach an IPv4 on the public NIC. If false, no IPv4 address will be attached. Defaults to true.
    #[serde(rename = "enable_ipv4", skip_serializing_if = "Option::is_none")]
    pub enable_ipv4: Option<bool>,
    /// Attach an IPv6 on the public NIC. If false, no IPv6 address will be attached. Defaults to true.
    #[serde(rename = "enable_ipv6", skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// ID of the ipv4 Primary IP to use. If omitted and enable_ipv4 is true, a new ipv4 Primary IP will automatically be created.
    #[serde(rename = "ipv4", skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<i32>,
    /// ID of the ipv6 Primary IP to use. If omitted and enable_ipv6 is true, a new ipv6 Primary IP will automatically be created.
    #[serde(rename = "ipv6", skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<i32>,
}

impl CreateServerRequestPublicNet {
    #![allow(clippy::too_many_arguments)]
    /// Public Network options
    pub fn new() -> CreateServerRequestPublicNet {
        CreateServerRequestPublicNet {
            enable_ipv4: None,
            enable_ipv6: None,
            ipv4: None,
            ipv6: None,
        }
    }
}
