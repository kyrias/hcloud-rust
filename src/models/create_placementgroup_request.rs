/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// CreatePlacementgroupRequest : Request for POST https://api.hetzner.cloud/v1/placement_groups

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePlacementgroupRequest {
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Name of the PlacementGroup
    #[serde(rename = "name")]
    pub name: String,
    /// Define the Placement Group Type.
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl CreatePlacementgroupRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/placement_groups
    pub fn new(name: String, r#type: Type) -> CreatePlacementgroupRequest {
        CreatePlacementgroupRequest {
            labels: None,
            name,
            r#type,
        }
    }
}

/// Define the Placement Group Type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "spread")]
    Spread,
}

impl Default for Type {
    fn default() -> Type {
        Self::Spread
    }
}
