/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreateFirewallRequest : Request for POST https://api.hetzner.cloud/v1/firewalls

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateFirewallRequest {
    /// Resources the Firewall should be applied to after creation
    #[serde(rename = "apply_to", skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<Vec<crate::models::FirewallResourceWithRequiredType>>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Name of the Firewall
    #[serde(rename = "name")]
    pub name: String,
    /// Array of rules
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<crate::models::Rule>>,
}

impl CreateFirewallRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/firewalls
    pub fn new(name: String) -> CreateFirewallRequest {
        CreateFirewallRequest {
            apply_to: None,
            labels: None,
            name,
            rules: None,
        }
    }
}
