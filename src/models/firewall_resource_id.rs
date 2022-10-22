/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// FirewallResourceId : Resource a Firewall should be applied to.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FirewallResourceId {
    #[serde(
        rename = "applied_to_resources",
        skip_serializing_if = "Option::is_none"
    )]
    pub applied_to_resources: Option<Vec<crate::models::FirewallResourceIdAppliedToResources>>,
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<Box<crate::models::LabelSelector>>,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::ResourceId>>,
    /// Type of resource referenced
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl FirewallResourceId {
    #![allow(clippy::too_many_arguments)]
    /// Resource a Firewall should be applied to.
    pub fn new(r#type: Type) -> FirewallResourceId {
        FirewallResourceId {
            applied_to_resources: None,
            label_selector: None,
            server: None,
            r#type,
        }
    }
}

/// Type of resource referenced
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "label_selector")]
    LabelSelector,
    #[serde(rename = "server")]
    Server,
}

impl Default for Type {
    fn default() -> Type {
        Self::LabelSelector
    }
}
