/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeFloatingIpProtectionResponse : Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeFloatingIpProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeFloatingIpProtectionResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/floating_ips/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeFloatingIpProtectionResponse {
        ChangeFloatingIpProtectionResponse {
            action: Box::new(action),
        }
    }
}
