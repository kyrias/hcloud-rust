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
pub struct Iso {
    /// ISO 8601 timestamp of deprecation, null if ISO is still available. After the deprecation time it will no longer be possible to attach the ISO to Servers.
    #[serde(rename = "deprecated")]
    pub deprecated: Option<String>,
    /// Description of the ISO
    #[serde(rename = "description")]
    pub description: String,
    /// ID of the Resource
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier of the ISO. Only set for public ISOs
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Type of the ISO
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl Iso {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        deprecated: Option<String>,
        description: String,
        id: i32,
        name: Option<String>,
        r#type: Type,
    ) -> Iso {
        Iso {
            deprecated,
            description,
            id,
            name,
            r#type,
        }
    }
}

/// Type of the ISO
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}

impl Default for Type {
    fn default() -> Type {
        Self::Private
    }
}
