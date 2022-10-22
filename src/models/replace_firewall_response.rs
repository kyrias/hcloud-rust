/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceFirewallResponse : Response to PUT https://api.hetzner.cloud/v1/firewalls/{id}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReplaceFirewallResponse {
    #[serde(rename = "firewall")]
    pub firewall: Box<crate::models::Firewall>,
}

impl ReplaceFirewallResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to PUT https://api.hetzner.cloud/v1/firewalls/{id}
    pub fn new(firewall: crate::models::Firewall) -> ReplaceFirewallResponse {
        ReplaceFirewallResponse {
            firewall: Box::new(firewall),
        }
    }
}
