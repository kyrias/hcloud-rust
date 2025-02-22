/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.12.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// RetryIssuanceOrRenewalResponse : Response to POST https://api.hetzner.cloud/v1/certificates/{id}/actions/retry

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetryIssuanceOrRenewalResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl RetryIssuanceOrRenewalResponse {
    #![allow(clippy::too_many_arguments)]
    /// Response to POST https://api.hetzner.cloud/v1/certificates/{id}/actions/retry
    pub fn new(action: crate::models::Action) -> RetryIssuanceOrRenewalResponse {
        RetryIssuanceOrRenewalResponse {
            action: Box::new(action),
        }
    }
}
