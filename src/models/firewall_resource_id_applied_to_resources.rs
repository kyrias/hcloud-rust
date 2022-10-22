/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.11.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FirewallResourceIdAppliedToResources {
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<Box<crate::models::ResourceId>>,
    /// Type of resource referenced
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl FirewallResourceIdAppliedToResources {
    #![allow(clippy::too_many_arguments)]
    pub fn new() -> FirewallResourceIdAppliedToResources {
        FirewallResourceIdAppliedToResources {
            server: None,
            r#type: None,
        }
    }
}

/// Type of resource referenced
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "server")]
    Server,
}

impl Default for Type {
    fn default() -> Type {
        Self::Server
    }
}
