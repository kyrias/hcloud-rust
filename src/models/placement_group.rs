/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PlacementGroup {
    /// Point in time when the Resource was created (in ISO-8601 format)
    #[serde(rename = "created")]
    pub created: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    /// Name of the Resource. Must be unique per Project.
    #[serde(rename = "name")]
    pub name: String,
    /// Array of IDs of Servers that are part of this Placement Group
    #[serde(rename = "servers")]
    pub servers: Vec<i32>,
    /// Type of the Placement Group
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl PlacementGroup {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        created: String,
        id: i32,
        labels: ::std::collections::HashMap<String, String>,
        name: String,
        servers: Vec<i32>,
        r#type: Type,
    ) -> PlacementGroup {
        PlacementGroup {
            created,
            id,
            labels,
            name,
            servers,
            r#type,
        }
    }
}

/// Type of the Placement Group
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
