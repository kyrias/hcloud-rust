/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreatePlacementgroupRequest : Request for POST https://api.hetzner.cloud/v1/placement_groups



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
    pub _type: Type,
}

impl CreatePlacementgroupRequest {
    /// Request for POST https://api.hetzner.cloud/v1/placement_groups
    pub fn new(name: String, _type: Type) -> CreatePlacementgroupRequest {
        CreatePlacementgroupRequest {
            labels: None,
            name,
            _type,
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

