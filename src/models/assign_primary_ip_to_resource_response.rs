/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// AssignPrimaryIpToResourceResponse : Response to POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/assign

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssignPrimaryIpToResourceResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AssignPrimaryIpToResourceResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/primary_ips/{id}/actions/assign
    pub fn new(action: crate::models::Action) -> AssignPrimaryIpToResourceResponse {
        AssignPrimaryIpToResourceResponse {
            action: Box::new(action),
        }
    }
}
