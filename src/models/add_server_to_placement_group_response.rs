/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AddServerToPlacementGroupResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/add_to_placement_group

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddServerToPlacementGroupResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AddServerToPlacementGroupResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/add_to_placement_group
    pub fn new(action: crate::models::Action) -> AddServerToPlacementGroupResponse {
        AddServerToPlacementGroupResponse {
            action: Box::new(action),
        }
    }
}
