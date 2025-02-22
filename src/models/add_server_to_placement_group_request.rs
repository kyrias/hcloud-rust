/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddServerToPlacementGroupRequest : Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/add_to_placement_group

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddServerToPlacementGroupRequest {
    /// ID of Placement Group the Server should be added to
    #[serde(rename = "placement_group")]
    pub placement_group: i32,
}

impl AddServerToPlacementGroupRequest {
    #![allow(clippy::too_many_arguments)]
    /// Request for POST https://api.hetzner.cloud/v1/servers/{id}/actions/add_to_placement_group
    pub fn new(placement_group: i32) -> AddServerToPlacementGroupRequest {
        AddServerToPlacementGroupRequest { placement_group }
    }
}
