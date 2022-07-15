/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// DeleteServiceResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteServiceResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl DeleteServiceResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/delete_service
    pub fn new(action: crate::models::Action) -> DeleteServiceResponse {
        DeleteServiceResponse {
            action: Box::new(action),
        }
    }
}
