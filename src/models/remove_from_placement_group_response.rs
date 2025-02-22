/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RemoveFromPlacementGroupResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/remove_from_placement_group

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveFromPlacementGroupResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl RemoveFromPlacementGroupResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/remove_from_placement_group
    pub fn new(action: crate::models::Action) -> RemoveFromPlacementGroupResponse {
        RemoveFromPlacementGroupResponse {
            action: Box::new(action),
        }
    }
}
