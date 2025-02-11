/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ServerPublicNet : Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerPublicNet {
    /// Firewalls applied to the public network interface of this Server
    #[serde(rename = "firewalls", skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<crate::models::ServerPublicNetFirewall>>,
    /// IDs of Floating IPs assigned to this Server
    #[serde(rename = "floating_ips")]
    pub floating_ips: Vec<i32>,
    #[serde(rename = "ipv4")]
    pub ipv4: Option<Box<crate::models::Ipv4>>,
    #[serde(rename = "ipv6")]
    pub ipv6: Option<Box<crate::models::Ipv6>>,
}

impl ServerPublicNet {
    #![allow(clippy::too_many_arguments)]
    /// Public network information. The Server's IPv4 address can be found in `public_net->ipv4->ip`
    pub fn new(
        floating_ips: Vec<i32>,
        ipv4: Option<crate::models::Ipv4>,
        ipv6: Option<crate::models::Ipv6>,
    ) -> ServerPublicNet {
        ServerPublicNet {
            firewalls: None,
            floating_ips,
            ipv4: ipv4.map(Box::new),
            ipv6: ipv6.map(Box::new),
        }
    }
}
