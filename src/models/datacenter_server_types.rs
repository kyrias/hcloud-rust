/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DatacenterServerTypes : The Server types the Datacenter can handle

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DatacenterServerTypes {
    /// IDs of Server types that are supported and for which the Datacenter has enough resources left
    #[serde(rename = "available")]
    pub available: Vec<i32>,
    /// IDs of Server types that are supported and for which the Datacenter has enough resources left
    #[serde(rename = "available_for_migration")]
    pub available_for_migration: Vec<i32>,
    /// IDs of Server types that are supported in the Datacenter
    #[serde(rename = "supported")]
    pub supported: Vec<i32>,
}

impl DatacenterServerTypes {
    #![allow(clippy::too_many_arguments)]
    /// The Server types the Datacenter can handle
    pub fn new(
        available: Vec<i32>,
        available_for_migration: Vec<i32>,
        supported: Vec<i32>,
    ) -> DatacenterServerTypes {
        DatacenterServerTypes {
            available,
            available_for_migration,
            supported,
        }
    }
}
