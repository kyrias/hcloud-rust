/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.10.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// ChangeAlgorithmResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_algorithm

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeAlgorithmResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeAlgorithmResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/change_algorithm
    pub fn new(action: crate::models::Action) -> ChangeAlgorithmResponse {
        ChangeAlgorithmResponse {
            action: Box::new(action),
        }
    }
}
